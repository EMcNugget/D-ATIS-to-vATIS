<script setup lang="ts">
import Layout from "./Layout.vue";
import { fetch_atis } from "../lib/parser";
import { use_store } from "../lib/stores";
import {
  TAlert,
  TATISCode,
  facilities,
  vATIS,
  TATIS,
  alert_types,
} from "../lib/types";
import { invoke } from "@tauri-apps/api/core";
import { emit } from "@tauri-apps/api/event";
import { computed, ref, watch } from "vue";

const store = use_store();

const facility = computed({
  get: () => store.get_individual("facility"),
  set: (v) => store.set_individual("facility", v.toUpperCase()),
});

const profile = computed({
  get: () => store.get_individual("profile"),
  set: (v) => store.set_individual("profile", v),
});

const message = computed({
  get: () => store.get_alert(),
  set: (v) => store.set_alert(v.alert, v.slot),
});

const update_time = computed({
  get: () => store.get_individual("update_time"),
  set: (v) => store.set_individual("update_time", v),
});

const check_update = computed(() => store.get_individual("check_updates"));
const check_updates_freq = computed(() =>
  store.get_individual("check_updates_freq")
);

const open_vatis_on_fetch = computed(() =>
  store.get_individual("open_vatis_on_fetch")
);

const airports_in_profile = computed(() => store.get_airports_in_profile());
const fetch_for_profile = computed(() =>
  store.get_individual("fetch_for_profile")
);

let facs: string[] = [];
let codes: Record<string, string[]> = {};

const tooltip = ref("");

const validate_iaco = (value: string) => {
  if (!facilities.includes(value)) {
    tooltip.value = "Invalid facility";
    return false;
  } else if (
    !store.get_individual("file_path") &&
    store.get_individual("custom_path")
  ) {
    tooltip.value =
      "Please select the path to your vATIS installation in settings";
    return false;
  } else if (!airports_in_profile.value.includes(value)) {
    tooltip.value = "Facility not in profile";
    return false;
  } else {
    tooltip.value = "";
    return true;
  }
};

const get_atis_code = (atis: vATIS[]): TATISCode[] => {
  return atis.map((k) => {
    switch (k.atis_type) {
      case "arr":
        return {
          type: "Arrival",
          code: k.atis_code,
        };
      case "dep":
        return {
          type: "Departure",
          code: k.atis_code,
        };
      case "combined":
        return {
          type: "Combined",
          code: k.atis_code,
        };
      default:
        return {
          type: "Combined",
          code: k.atis_code,
        };
    }
  });
};

const get_alert_level = (level: string) => {
  switch (level) {
    case "error":
      return 0;
    case "warn":
      return 1;
    case "success":
      return 2;
    case "info":
      return 3;
    default:
      return 0;
  }
};

const get_atis = async () => {
  try {
    if (await invoke("is_vatis_running")) {
      message.value = {
        alert: {
          alert_type: "error",
          message: "Close vATIS before fetching ATIS",
        },
      };
      return;
    }

    let messages: { key: string; message: string }[] = [];
    let status: Record<string, string> = {};

    codes = {};

    if (store.get_individual("fetch_for_profile")) {
      airports_in_profile.value.forEach((k) => {
        if (facilities.includes(k) && !facs.includes(k)) {
          facs.push(k);
        }
      });
    } else {
      facs.push(facility.value);
    }

    const promises = facs.map(async (fac) => {
      let atis: vATIS[] = [];
      try {
        atis = await fetch_atis(fac);
      } catch (e) {
        const alert = e as TAlert;
        messages.push({ key: fac, message: alert.message as string });
        status[fac] = alert.alert_type;
        if (alert.payload) {
          atis.push(alert.payload as vATIS);
        }
      }

      const alert = await invoke<TAlert>("write_atis", {
        facility: fac,
        atis: atis,
      });

      const success = alert.alert_type === "success";
      alert.message = (alert.message as string).concat(
        success
          ? ` ${get_atis_code(atis)
              .map((k) => `${k.type}: ${k.code}`)
              .join(", ")}`
          : ""
      );

      messages.push({ key: fac, message: alert.message as string });
      status[fac] = alert.alert_type;
      codes[fac] = get_atis_code(atis).map((k) => k.code);
    });

    await Promise.all(promises);

    let alert_level = 0;

    Object.values(status).forEach((k) => {
      const level = get_alert_level(k);
      if (level < alert_level) {
        alert_level = level;
      }
    });

    const success = alert_level > 1;

    message.value = {
      alert: {
        alert_type: alert_types[alert_level + 1],
        message: messages,
      },
    };

    if (open_vatis_on_fetch.value && success) {
      await invoke("open_vatis", {
        custom_path: store.get_individual("custom_path"),
      });
    }
  } catch (e) {
    message.value = { alert: e as TAlert };
  }
};

const alert_new_codes = (rows: { key: string; message: string }[]) => {
  message.value = {
    alert: {
      alert_type: "info",
      message: rows,
    },
    slot: `New ATIS found for: ${facs.join(", ")}`,
  };
  emit("new-codes");
};

const get_zulu_time = () => {
  const now = new Date();

  const hours = now.getUTCHours().toString().padStart(2, "0");
  const minutes = now.getUTCMinutes().toString().padStart(2, "0");

  return hours + minutes;
};

let temp_time = ref(update_time.value);

watch(
  () => check_updates_freq.value,
  (val) => {
    if (val) {
      setInterval(() => {
        let time = get_zulu_time();
        const minutes = parseInt(time.slice(2, 4), 10);
        if (minutes >= 53 || minutes <= 3) {
          temp_time.value = 2;
        } else {
          temp_time.value = update_time.value;
        }
      }, 60000);
    }
  },
  { immediate: true }
);

let interval_id: NodeJS.Timeout | undefined;

watch(
  () => temp_time.value,
  (val) => {
    if (check_update.value) {
      if (interval_id) {
        clearInterval(interval_id);
      }

      let changed = false;
      let new_codes: Record<string, string[]> = {};

      interval_id = setInterval(async () => {
        facs.forEach(async (fac) => {
          const response = await fetch(
            `https://datis.clowd.io/api/${fac}`
          ).then((res) => res.json());

          response.forEach((k: TATIS) => {
            if (!codes.value.includes(k.code)) {
              changed = true;
              codes[fac] = new_codes[fac] || [];
            }
          });
        });

        if (changed) {
          alert_new_codes(
            Object.entries(new_codes).map(([k, v]) => ({
              key: k,
              message: v.join(", "),
            }))
          );
          codes = new_codes;
        }
      }, val * 60000);

      return interval_id;
    }
  },
  { immediate: true } // Run immediately to set up the initial interval
);
</script>

<template>
  <Layout>
    <div class="flex flex-col items-center">
      <input
        type="text"
        placeholder="Select a profile..."
        class="input input-bordered mb-4 text-center"
        readonly
        :value="profile === 'No Profile' ? null : profile"
        v-if="fetch_for_profile"
      />
      <input
        type="text"
        placeholder="Airport Code..."
        :class="
          'input input-bordered w-full max-w-xs mb-4 input-uppercase ' +
          (validate_iaco(facility) ? '' : ' input-error')
        "
        v-model="facility"
        maxlength="4"
        v-else
      />
      <div
        :class="tooltip ? 'tooltip tooltip-bottom' : ''"
        :data-tip="tooltip || ''"
      >
        <button
          class="btn btn-primary w-half max-w-xs mb-4"
          @click="get_atis()"
          :disabled="!fetch_for_profile && !validate_iaco(facility)"
        >
          Fetch
        </button>
      </div>
    </div>
  </Layout>
</template>

<style>
.input-uppercase::placeholder {
  text-transform: none;
}

.input-uppercase {
  text-transform: uppercase;
}
</style>

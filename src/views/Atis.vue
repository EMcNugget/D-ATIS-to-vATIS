<script setup lang="ts">
import { fetch_atis } from "../lib/parser";
import { use_store } from "../lib/stores";
import { TAlert, vATIS, TATIS, alert_types, facilities } from "../lib/types";
import { invoke } from "@tauri-apps/api/core";
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
  set: (v) => store.set_alert(v),
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
const fetch_for_profile = computed(() => {
  const k = store.get_individual("fetch_for_profile");
  if (k) {
    tooltip.value = "";
  }
  return k;
});

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

const get_alert_level = (level: string) => {
  switch (level) {
    case "error":
      return 0;
    case "warn":
      return 1;
    case "info":
      return 2;
    case "success":
      return 3;
    default:
      return 0;
  }
};

type WriteAtis = {
  alert: TAlert;
  codes: {
    key: string;
    codes: string[];
  };
};

const get_atis = async () => {
  try {
    if (await invoke("is_vatis_running")) {
      message.value = {
        alert_type: "error",
        message: "Close vATIS before fetching ATIS",
      };
      return;
    }

    let facs: string[] = [];
    let messages: { key: string; message: string }[] = [];
    let status: Record<string, string> = {};
    let code_str: {
      key: string;
      codes: string[];
    }[] = [];

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
        if (alert.payload) {
          atis.push(alert.payload as vATIS);
        }
        messages.push({ key: fac, message: alert.message as string });
        status[fac] = alert.alert_type;
      }

      const alert = await invoke<WriteAtis>("write_atis", {
        facility: fac,
        atis: atis,
      });

      code_str.push(alert.codes);

      const success = alert.alert.alert_type === "success";
      alert.alert.message = success ? alert.alert.message : "";

      if (!messages.some((k) => k.key === fac)) {
        messages.push({ key: fac, message: alert.alert.message as string });
        status[fac] = alert.alert.alert_type;
      }
    });

    await Promise.all(promises);

    messages.forEach((k) => {
      const index = code_str.findIndex((j) => j.key === k.key);
      if (index !== -1) {
        k.message = k.message.concat(` ${code_str[index].codes.join(", ")}`);
      }
    });

    let alert_level = 3;

    Object.values(status).forEach((k) => {
      const level = get_alert_level(k);
      if (level < alert_level) {
        alert_level = level;
      }
    });

    const success = alert_level > 1;

    message.value = {
      alert_type: alert_types[alert_level],
      message: messages,
    };

    if (open_vatis_on_fetch.value && success) {
      await invoke("open_vatis", {
        custom_path: store.get_individual("custom_path"),
      });
    }
  } catch (e) {
    message.value = e as TAlert;
  }
};

const alert_new_codes = (codes: Record<string, string[]>) => {
  let rows: Array<{ key: string; message: string }> = [];

  Object.entries(codes).forEach(([key, value]) => {
    rows.push({ key, message: value.join(", ") });
  });

  message.value = {
    alert_type: "info",
    message: rows,
    slot: `New ATIS's found for ${rows.map((k) => k.key).join(", ")}`,
  };
  if (!store.get_individual("suppress_notification")) {
    invoke("play_codes");
  }
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

      let facilities = [facility.value];

      if (fetch_for_profile.value) {
        facilities = airports_in_profile.value;
      }

      let changed = false;
      let new_codes: Record<string, string[]> = {};

      interval_id = setInterval(async () => {
        facilities.forEach(async (fac) => {
          const response = await fetch(
            `https://datis.clowd.io/api/${facility.value}`
          ).then((res) => res.json());

          response.forEach((k: TATIS) => {
            if (!codes.value.includes(k.code)) {
              changed = true;
              codes[fac] = new_codes[fac] || [];
            }
          });
        });

        if (changed) {
          alert_new_codes(new_codes);
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
        'input input-bordered w-full max-w-xs mb-4 input-uppercase text-center' +
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
</template>

<style>
.input-uppercase::placeholder {
  text-transform: none;
}

.input-uppercase {
  text-transform: uppercase;
}
</style>

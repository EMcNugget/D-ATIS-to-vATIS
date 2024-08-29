<script setup lang="ts">
import Layout from "./Layout.vue";
import { fetch_atis } from "../lib/parser";
import { use_store } from "../lib/stores";
import { TAlert, TATISCode, facilities, vATIS, TATIS } from "../lib/types";
import { invoke } from "@tauri-apps/api/core";
import { emit } from "@tauri-apps/api/event";
import { computed, ref, watch } from "vue";

const store = use_store();

const facility = computed({
  get: () => store.get_individual("facility"),
  set: (v) => store.set_individual("facility", v),
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

const codes = computed({
  get: () => store.get_codes(),
  set: (v) => store.set_codes(v),
});

const tooltip = ref("");

const validateICAO = (value: string) => {
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

const get_atis = async () => {
  try {
    console.log(store.get_individual("check_updates"));
    invoke("is_vatis_running").then(async (k) => {
      if (k) {
        message.value = {
          alert_type: "error",
          message: "Close vATIS before fetching ATIS",
        };
        return;
      } else {
        await fetch_atis(facility.value).then((atis) => {
          store.set_codes(atis.map((k) => k.atis_code));
          invoke("write_atis", {
            facility: facility.value,
            atis: atis,
          }).then((k) => {
            const v = k as TAlert;
            const success = v.alert_type === "success";
            v.message = v.message.concat(
              success
                ? ` | ${get_atis_code(atis)
                    .map((k) => `${k.type}: ${k.code}`)
                    .join(", ")}`
                : ""
            );
            message.value = v;
            if (open_vatis_on_fetch.value && success) {
              invoke("open_vatis", {
                custom_path: store.get_individual("custom_path")
                  ? store.get_individual("custom_path")
                  : null,
              });
            }
          });
        });
      }
    });
  } catch (e) {
    message.value = e as TAlert;
  }
};

const alert_new_codes = (codes: string[]) => {
  message.value = {
    alert_type: "info",
    message: `${facility.value} information: ${codes.join(", ")} is current`,
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

      interval_id = setInterval(async () => {
        const response = await fetch(
          `https://datis.clowd.io/api/${facility.value}`
        ).then((res) => res.json());

        let changed = false;
        let new_codes: string[] = [];

        response.forEach((k: TATIS) => {
          if (!codes.value.includes(k.code)) {
            changed = true;
            new_codes.push(k.code);
          }
        });

        if (changed) {
          alert_new_codes(new_codes);
          codes.value = new_codes;
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
        placeholder="Airport Code..."
        :class="
          'input input-bordered w-full max-w-xs mb-4 input-uppercase ' +
          (validateICAO(facility) ? '' : ' input-error')
        "
        v-model="facility"
        maxlength="4"
      />
      <div v-if="tooltip" class="tooltip tooltip-bottom" :data-tip="tooltip">
        <button
          class="btn btn-primary w-half max-w-xs mb-4"
          @click="get_atis()"
          :disabled="!validateICAO(facility)"
        >
          Fetch
        </button>
      </div>
      <div v-else>
        <button
          class="btn btn-primary w-half max-w-xs mb-4"
          @click="get_atis()"
          :disabled="!validateICAO(facility)"
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

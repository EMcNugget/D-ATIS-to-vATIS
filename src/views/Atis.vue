<script setup lang="ts">
import { fetch_atis } from "../util/parser";
import { use_store } from "../util/stores";
import { Alert, ATISCode, facilities, vATIS } from "../util/types";
import { invoke } from "@tauri-apps/api/core";
import { computed, ref } from "vue";

const store = use_store();

const facility = computed({
  get: () => store.get_facility(),
  set: (v) => store.set_facility(v),
});

const message = computed({
  get: () => store.get_message(),
  set: (v) => store.set_message(v),
});

const tooltip = ref("");

const validateICAO = (value: string) => {
  if (!facilities.includes(value)) {
    tooltip.value = "Invalid facility";
    return false;
  } else if (!store.get_file_path()) {
    tooltip.value =
      "Please select the path to your vATIS installation in settings";
    return false;
  } else return true;
};

const get_atis_code = (atis: vATIS[]): ATISCode[] => {
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

const fetch = async () => {
  try {
    await fetch_atis(facility.value).then((atis) => {
      store.set_atis(atis);
      invoke("write_atis", {
        facility: facility.value,
        atis: atis,
      }).then((k) => {
        const v: Alert = k as Alert;
        const success = v.alert_type === "success";
        v.message = v.message.concat(
          success
            ? ` | ${get_atis_code(atis)
                .map((k) => `${k.type}: ${k.code}`)
                .join(", ")}`
            : ""
        );
        message.value = v;
      });
    });
  } catch (e) {
    message.value = e as Alert;
  }
};
</script>

<template>
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
        @click="fetch()"
        :disabled="!validateICAO(facility)"
      >
        Fetch
      </button>
    </div>
    <div v-else>
      <button
        class="btn btn-primary w-half max-w-xs mb-4"
        @click="fetch()"
        :disabled="!validateICAO(facility)"
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

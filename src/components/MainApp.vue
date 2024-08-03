<script setup lang="ts">
import Alerts from "./Alerts.vue";
import { Ref, computed, ref, watch } from "vue";
import { use_settings, use_atis_store } from "@util/stores";
import { fetch_atis } from "@util/parser";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { Settings, facilities, Alert, vATIS, ATISCode } from "@util/types";

const open_path = () => {
  open({
    multiple: false,
    directory: true,
    filters: [],
  }).then((path) => {
    settings.set_file_path(path ? (path as string) : settings.get_file_path());
  });
};

const settings = use_settings();
const atis_store = use_atis_store();

const facility = computed({
  get: () => settings.get_facility(),
  set: (value) => settings.set_facility(value),
});

const file_path = computed({
  get: () => settings.get_file_path(),
  set: (value) => settings.set_file_path(value),
});

const save_facility = computed({
  get: () => settings.get_save_facility(),
  set: (value) => settings.set_save_facility(value),
});

const profile = computed({
  get: () => settings.get_profile(),
  set: (value) => settings.set_profile(value),
});

const message: Ref<Alert> = ref({ message: "", alert_type: "success" });
const showAlert = ref(false);

watch(
  () => message.value,
  () => {
    showAlert.value = true;
  }
);

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
      atis_store.set_atis(atis);
      invoke("write_atis", {
        facility: facility.value,
        atis: atis,
      }).then((k) => {
        const v: Alert = k as Alert;
        let success = v.alert_type === "success";
        v.message = v.message.concat(
          ` | ${
            success
              ? get_atis_code(atis)
                  .map((k) => `${k.type}: ${k.code}`)
                  .join(", ")
              : ""
          }`
        );
        message.value = v;
      });
    });
  } catch (e) {
    message.value = e as Alert;
  }
};

const validateICAO = (value: string) => {
  if (
    (value.length !== 4,
    !value.match(/^[A-Z]{4}$/),
    value.startsWith("K"),
    !facilities.includes(value))
  ) {
    return false;
  } else return true;
};

const save_settings = () => {
  invoke("write_settings", {
    settings: {
      facility: settings.get_save_facility() ? settings.get_facility() : "",
      file_path: settings.get_file_path(),
      save_facility: settings.get_save_facility(),
      profile: settings.get_profile(),
    },
  });
};

invoke("read_settings").then((k) => {
  settings.set_all(k as Settings);
});
</script>

<template>
  <div class="h-screen relative flex flex-col items-center justify-center">
    <Alerts :message="message" :show="showAlert" @close="showAlert = false" />
    <div class="flex flex-col items-center">
      <input
        type="text"
        placeholder="Airport Code..."
        :class="
          'input input-bordered w-full max-w-xs mb-4 input-uppercase' +
          (validateICAO(facility) ? '' : ' input-error')
        "
        v-model="facility"
        maxlength="4"
      />
      <button
        class="btn btn-primary w-half max-w-xs mb-4"
        @click="fetch()"
        :disabled="!validateICAO(facility)"
      >
        Fetch
      </button>
      <dialog id="my_modal_3" class="modal">
        <div class="modal-box">
          <form method="dialog">
            <button
              class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
            >
              ✕
            </button>
          </form>
          <h3 class="text-lg font-bold mb-6">Settings</h3>
          <div class="flex flex-row">
            <input
              type="text"
              v-model="file_path"
              readonly
              placeholder="File Path..."
              class="input input-bordered w-full mr-4 mb-4"
            />
            <button class="btn" @click="open_path()">Browse</button>
          </div>
          <input
            type="text"
            v-model="profile"
            placeholder="Profile..."
            class="input input-bordered w-full mr-4 mb-4"
          />
          <label class="label cursor-pointer justify-start">
            <span class="label-text text-lg mr-6 font-bold">Save Facility</span>
            <input type="checkbox" class="checkbox" v-model="save_facility" />
          </label>
          <button
            class="btn btn-active btn-primary mt-8 w-full"
            @click="save_settings()"
          >
            Save
          </button>
        </div>
      </dialog>
    </div>
    <button
      class="btn btn-circle fixed bottom-0 left-0 m-4 flex items-center justify-center"
      onclick="my_modal_3.showModal()"
    >
      <img src="/settings.svg" alt="Settings" class="md:h-6" />
    </button>
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

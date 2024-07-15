<script setup lang="ts">
import { computed } from "vue";
import { use_settings } from "../stores";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { Settings } from "../types";

const open_path = () => {
  open({
    multiple: false,
    directory: false,
    filters: [
      {
        name: "JSON",
        extensions: ["json"],
      },
    ],
  }).then((k) => {
    settings.set_file_path(k ? (k.path as string) : settings.get_file_path());
  });
};

const settings = use_settings();

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

settings.$subscribe(() => {
  invoke("write_settings", {
    settings: {
      facility: settings.get_save_facility() ? settings.get_facility() : "",
      file_path: settings.get_file_path(),
      save_facility: settings.get_save_facility(),
    },
  });
});

invoke("read_settings").then((k) => {
  const v: Settings = k as Settings;
  settings.set_facility(v.facility);
  settings.set_file_path(v.file_path);
  settings.set_save_facility(v.save_facility);
});
</script>

<template>
  <div class="h-screen flex items-center justify-center">
    <div class="flex flex-col items-center">
      <input
        type="text"
        placeholder="Airport Code..."
        class="input input-bordered w-full max-w-xs mb-4 input-uppercase"
        v-model="facility"
      />
      <button class="btn btn-primary w-half max-w-xs mb-4">Fetch</button>
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
          <div class="flex flex-row max-w-xs">
            <input
              type="text"
              v-model="file_path"
              readonly
              placeholder="File Path..."
              class="input input-bordered w-full mr-4 mb-4"
            />
            <button class="btn" @click="open_path()">Browse</button>
          </div>
          <div class="form-control">
            <label class="label cursor-pointer justify-start">
              <span class="label-text text-lg mr-6 font-bold"
                >Save Facility</span
              >
              <input type="checkbox" class="checkbox" v-model="save_facility" />
            </label>
          </div>
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

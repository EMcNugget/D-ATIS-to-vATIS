<script setup lang="ts">
import { computed } from "vue";
import { use_atis_store } from "../stores";
import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog";
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
    store.set_file_path(k as string);
  });
};

const store = use_atis_store();

const facility = computed({
  get: () => store.get_facility(),
  set: (value) => store.set_facility(value),
});

const file_path = computed({
  get: () => store.get_file_path(),
  set: (value) => store.set_file_path(value),
});

store.$subscribe(() => {
  invoke("write_settings", {
    settings: {
      facility: store.get_facility(),
      file_path: store.get_file_path(),
    },
  });
});

invoke("read_settings").then((k) => {
  const settings: Settings = k as Settings;
  store.set_facility(settings.facility);
  store.set_file_path(settings.file_path);
});
</script>

<template>
  <div class="h-screen flex items-center justify-center">
    <div class="flex flex-col items-center">
      <input
        type="text"
        placeholder="Airport Code..."
        class="input input-bordered w-full max-w-xs mb-4"
        v-model="facility"
      />
      <div class="flex flex-row max-w-xs">
        <input
          type="text"
          v-model="file_path"
          readonly
          placeholder="File Path..."
          class="input input-bordered w-full mr-4 mb-4"
        />
        <button class="btn btn-square text-xs" @click="open_path()">
          Browse
        </button>
      </div>
      <button class="btn btn-primary w-half max-w-xs">Fetch</button>
    </div>
  </div>
</template>

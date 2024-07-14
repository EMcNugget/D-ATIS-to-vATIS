<script setup lang="ts">
import { computed } from "vue";
import { use_atis_store } from "../stores";
import { invoke } from "@tauri-apps/api";
import { Settings } from "../types";

const store = use_atis_store();

const facility = computed({
  get: () => store.get_facility(),
  set: (value) => store.set_facility(value),
});

store.$subscribe(() => {
  invoke("write_settings", {
    settings: {
      facility: store.get_facility(),
      filepath: store.get_file_path(),
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
      <input
        type="file"
        class="file-input file-input-bordered w-full max-w-xs mb-4"
      />
      <button class="btn btn-primary w-half max-w-xs">Fetch</button>
    </div>
  </div>
</template>

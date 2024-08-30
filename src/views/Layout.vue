<script setup lang="ts">
import Alerts from "../components/Alerts.vue";
import Settings from "../components/Settings.vue";
import Update from "../components/Update.vue";
import { computed, ref, watch, onMounted } from "vue";
import { use_store } from "../lib/stores";
import { router } from "../lib/router";
import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { getVersion } from "@tauri-apps/api/app";

const store = use_store();

const version = ref("Unknown");
const show_update = ref(false);
const info = computed(() => `v${version.value} Copyright © 2024 Ethan Seys`);

const updateAndRelaunch = async () => {
  const update = await check();
  if (update?.available) {
    update.downloadAndInstall().then(() => {
      relaunch();
    });
  }
};

onMounted(async () => {
  version.value = await getVersion();
  if (!store.get_app_update()) {
    store.set_app_update(true);
    const update = await check();
    if (update?.available) {
      version.value = update.version;
      show_update.value = true;
    }
  }
});

const message = computed(() => store.get_alert());
const local_theme = computed(() => store.get_individual("theme"));
const show_alert = ref(false);
const show_settings = ref(false);

watch(
  () => message.value,
  () => {
    show_alert.value = true;
  }
);
</script>

<template>
  <div
    class="h-screen relative flex flex-col items-center justify-center"
    :data-theme="local_theme"
  >
    <Alerts :message="message" :show="show_alert" @close="show_alert = false" />
    <Update
      v-if="show_update"
      :show="show_update"
      :version="version"
      @close-update="show_update = false"
      @download-and-install="updateAndRelaunch"
    />
    <slot></slot>
    <Settings :showModal="show_settings" @close="show_settings = !show_settings" />
    <button
      class="btn btn-circle fixed bottom-0 left-0 m-4 flex items-center justify-center"
      @click="show_settings = !show_settings"
    >
      <img
        src="/settings.svg"
        alt="Settings"
        class="h-auto w-auto max-h-6 max-w-6"
      />
    </button>
    <button
      class="btn btn-circle fixed bottom-0 right-0 m-4 flex items-center justify-center"
      @click="router.back()"
      v-if="router.currentRoute.value.path !== '/'"
    >
      <img src="/back.svg" alt="Back" class="h-auto w-auto max-h-6 max-w-6" />
    </button>
    <div
      class="h-auto w-auto max-h-6 max-w-6 fixed bottom-0 right-0 m-4 flex items-center justify-center tooltip tooltip-left"
      :data-tip="info"
    >
      <img
        src="/info.svg"
        alt="Info"
        v-if="router.currentRoute.value.path === '/'"
      />
    </div>
  </div>
</template>

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

const version = ref<string>("Unknown");
const showUpdate = ref(false);
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
      showUpdate.value = true;
    }
  }
});

const message = computed(() => store.get_message());
const localTheme = computed(() => store.get_theme());
const showAlert = ref(false);
const showSettings = ref(false);

watch(
  () => message.value,
  () => {
    showAlert.value = true;
  }
);
</script>

<template>
  <div
    class="h-screen relative flex flex-col items-center justify-center"
    :data-theme="localTheme"
  >
    <Alerts :message="message" :show="showAlert" @close="showAlert = false" />
    <Update
      v-if="showUpdate"
      :show="showUpdate"
      :version="version"
      @close-update="showUpdate = false"
      @download-and-install="updateAndRelaunch"
    />
    <slot />
    <Settings :showModal="showSettings" @close="showSettings = !showSettings" />
    <button
      class="btn btn-circle fixed bottom-0 left-0 m-4 flex items-center justify-center"
      @click="showSettings = !showSettings"
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

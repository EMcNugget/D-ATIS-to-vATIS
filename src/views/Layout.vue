<script setup lang="ts">
import CAlerts from "../components/CAlerts.vue";
import CUpdate from "../components/CUpdate.vue";
import SettingsLayout from "./SettingsLayout.vue";
import { computed, ref, onBeforeMount } from "vue";
import { use_store } from "../lib/stores";
import { check, type Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { getVersion } from "@tauri-apps/api/app";
import { invoke } from "@tauri-apps/api/core";
import { TCustomContractions, TSettings } from "../lib/types";
import { info, error } from "@tauri-apps/plugin-log";

const store = use_store();

const version = ref("");
const show_update = ref(false);
const show_alert = ref(false);
let update_value: Update | null = null;

const undate_and_relaunch = async () => {
  if (update_value) {
    update_value.downloadAndInstall().then(() => {
      relaunch();
    });
  }
};

const setup = async () => {
  store.set_settings(await invoke<TSettings>("read_settings"));
  store.set_profiles(await invoke<string[]>("get_profiles"));
  store.set_contractions(await invoke<TCustomContractions>("get_contractions"));
  store.set_init(true);
  version.value = await getVersion();
  const update = await check();
  if (update?.available) {
    update_value = update;
    version.value = update.version;
    show_update.value = true;
  }
};

onBeforeMount(async () => {
  if (!store.get_init()) {
    setup()
      .then(() => {
        info("Initialization Complete");
      })
      .catch((e) => {
        error(e as string);
      });
  }
});

const message = computed(() => {
  show_alert.value = true;
  return store.get_alert();
});
const local_theme = computed(() => store.get_individual("theme"));
</script>

<template>
  <div
    class="h-screen relative flex flex-col items-center justify-center"
    :data-theme="local_theme"
  >
    <CAlerts
      :message="message"
      :show="show_alert"
      @close="show_alert = false"
    />
    <CUpdate
      v-if="show_update"
      :show="show_update"
      :version="version"
      @close-update="show_update = false"
      @download-and-install="undate_and_relaunch"
    />
    <slot></slot>
    <SettingsLayout />
    <button
      class="btn btn-circle fixed bottom-0 left-0 m-4 flex items-center justify-center"
      onclick="modal_1.showModal()"
    >
      <img
        src="/bars.svg"
        alt="Settings"
        class="h-auto w-auto max-h-6 max-w-6"
      />
    </button>
  </div>
</template>

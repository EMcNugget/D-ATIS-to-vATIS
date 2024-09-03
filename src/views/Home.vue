<script setup lang="ts">
import Layout from "./Layout.vue";
import { RouterLink, RouterView } from "vue-router";
import { TSettings } from "../lib/types";
import { use_store } from "../lib/stores";
import { invoke } from "@tauri-apps/api/core";
import { onMounted } from "vue";

const store = use_store();

let read = false;

const setup = async () => {
  if (!read) {
    read = true;
    store.set_settings(await invoke<TSettings>("read_settings"));
    store.set_profiles(await invoke<string[]>("get_profiles"));
  }
};

onMounted(setup);
</script>

<template>
  <Layout v-if="$route.path === '/'">
    <h1 class="absolute top-0 mt-6 font-semibold text-4xl">vATIS Utilites</h1>
    <div class="flex flex-row items-center justify-center space-x-8">
      <RouterLink to="/atis" class="btn btn-primary w-2/3"
        >vATIS to D-ATIS</RouterLink
      >
      <RouterLink to="/record" class="btn btn-primary w-2/3"
        >Record ATIS</RouterLink
      >
    </div>
  </Layout>
  <RouterView v-else />
</template>

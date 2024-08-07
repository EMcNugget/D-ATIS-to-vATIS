<script setup lang="ts">
import Alerts from "../components/Alerts.vue";
import Settings from "../components/Settings.vue";
import { computed, ref, watch } from "vue";
import { use_store } from "../lib/stores";
import { router } from "../lib/router";
const store = use_store();

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
  </div>
</template>

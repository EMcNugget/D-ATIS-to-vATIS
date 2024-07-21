<script setup lang="ts">
import type { Alert } from "../types";
const props = defineProps<{ message: Alert, show: boolean }>();
const emit = defineEmits(["close"]);

const getAlertClass = () => {
  switch (props.message.alert_type) {
    case "error":
      return "alert-error";
    case "warn":
      return "alert-warning";
    case "success":
      return "alert-success";
    default:
      return "success";
  }
};

const getAlertIconPath = () => {
  switch (props.message.alert_type) {
    case "error":
      return "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z";
    case "warn":
      return "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z";
    case "success":
      return "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z";
    default:
      return "";
  }
};
</script>

<template>
  <div class="fixed top-8 max-w-sm" v-if="show">
    <div :class="['alert', getAlertClass()]" role="alert">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="h-6 w-6 shrink-0 stroke-current"
        fill="none"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          :d="getAlertIconPath()"
        />
      </svg>
      <span>{{ props.message.message }}</span>
      <button class="btn btn-circle btn-ghost h-4" @click="emit('close')">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="h-4 w-4"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M6 18L18 6M6 6l12 12"
          />
        </svg>
      </button>
    </div>
  </div>
</template>

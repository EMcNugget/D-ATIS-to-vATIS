<script setup lang="ts">

defineProps<{
  version: string;
  show: boolean;
}>();

defineEmits(["downloadAndInstall", "closeUpdate"]);
</script>
<template>
  <div
    role="alert"
    class="alert alert-info fixed top-8 max-w-lg z-50"
    v-if="show"
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      class="h-6 w-6 shrink-0 stroke-current"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
      ></path>
    </svg>
    <span>Update {{ version }} is available!</span>
    <button
      class="btn btn-sm btn-info btn-active btn-outline"
      @click="$emit('downloadAndInstall')"
    >
      Download and Install
    </button>
    <button class="btn btn-circle btn-ghost h-4" @click="$emit('closeUpdate')">
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
</template>

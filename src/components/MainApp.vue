<script setup lang="ts">
import { computed } from "vue";
import { useATISSTore } from "../stores";

const store = useATISSTore();

const facility = computed({
  get: () => store.getFacility(),
  set: (value) => store.setFacility(value),
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

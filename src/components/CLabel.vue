<script setup lang="ts">
defineProps<{ title: string }>();
</script>


<template>
  <label class="label cursor-pointer justify-start">
    <span class="label-text mr-6 text-base font-semibold">
      {{ title }}
    </span>
    <slot></slot>
  </label>
</template>

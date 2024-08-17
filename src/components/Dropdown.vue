<script setup lang="ts">
defineProps<{
  name: string;
  click: () => void;
  show: boolean;
  tab_index: number;
}>();
</script>

<template>
  <div class="dropdown">
    <label :tabindex="tab_index" class="btn m-1" @click="click">
      {{ name }}
      <img
        v-if="show"
        src="/dropdown_up.svg"
        alt="Dropdown"
        class="h-auto w-auto max-h-6 max-w-6"
      />
      <img
        v-if="!show"
        src="/dropdown_down.svg"
        alt="Dropdown"
        class="h-auto w-auto max-h-6 max-w-6"
      />
    </label>
    <ul
      v-if="show"
      :tabindex="tab_index"
      class="dropdown-content menu p-2 shadow bg-base-200 rounded-box w-52 z-50"
    >
      <slot></slot>
    </ul>
  </div>
</template>

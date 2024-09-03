<script setup lang="ts">
defineProps<{
  head?: string[];
  rows: {
    key: string;
    message: string;
  }[];
}>();

</script>

<template>
  <div class="overflow-x-auto">
    <table class="table table-xs">
      <thead>
        <tr>
          <th></th>
          <th v-if="head && head.length" v-for="h in head" :key="h">{{ h }}</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(row, i) in rows" :key="i">
          <th>{{ row.key }}</th>
          <td>{{ row.message }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

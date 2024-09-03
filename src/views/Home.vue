<script setup lang="ts">
import Layout from "./Layout.vue";
import { RouterLink, RouterView } from "vue-router";
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

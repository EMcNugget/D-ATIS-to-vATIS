<script setup lang="ts">
import CLayout from "./components/CLayout.vue";
import { RouterLink, RouterView } from "vue-router";
</script>

<template>
  <Suspense>
    <CLayout>
      <div v-if="$route.path === '/'">
        <h1 class="absolute top-0 mt-6 font-semibold text-4xl">
          vATIS Utilites
        </h1>
        <div class="flex flex-row items-center justify-center space-x-8">
          <RouterLink to="/atis" class="btn btn-primary w-2/3"
            >vATIS to D-ATIS</RouterLink
          >
          <RouterLink to="/contractions" class="btn btn-primary w-2/3"
            >Edit Contractions</RouterLink
          >
        </div>
      </div>
      <RouterView v-else />
    </CLayout>
  </Suspense>
</template>

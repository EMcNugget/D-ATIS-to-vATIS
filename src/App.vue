<script setup lang="ts">
import Layout from "./views/Layout.vue";
import Atis from "./views/Atis.vue";
</script>

<template>
  <Suspense>
    <Layout>
      <Atis />
    </Layout>
  </Suspense>
</template>

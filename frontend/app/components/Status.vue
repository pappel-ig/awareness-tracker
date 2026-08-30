<script setup lang="ts">
defineProps<{
  description: String
}>()
</script>

<template>
  <p class="text-black text-md leading-relaxed mb-6">
    {{description}}
  </p>
  <slot/>
</template>

<style scoped>

</style>
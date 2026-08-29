<script setup lang="ts">
defineProps<{
  hasNext: boolean,
  hasPrev: boolean
}>()

const emit = defineEmits<{
  (e: 'next'): void
  (e: 'prev'): void
}>()
</script>

<template>
  <slot/>
  <div class="flex items-center justify-between w-full mt-6">
    <UButton v-if="hasPrev" @click="emit('prev')" variant="outline" size="md" trailing-icon="i-heroicons-arrow-left-20-solid" class="rounded-sm px-4 py-2">Zurück</UButton>
    <div v-else />
    <UButton v-if="hasNext" @click="emit('next')" variant="outline" size="md" trailing-icon="i-heroicons-arrow-right-20-solid" class="rounded-sm px-4 py-2">Nächste</UButton>
  </div>
</template>

<style scoped>

</style>
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
  <UButton v-if="hasNext && !hasPrev" @click="emit('next')" size="md" trailing-icon="i-heroicons-arrow-right-20-solid" class="rounded-sm px-4 py-2">Umfrage Starten</UButton>
  <div class="flex items-center justify-between w-full mt-6" v-else>
    <UButton v-if="hasPrev" @click="emit('prev')" size="md" trailing-icon="i-heroicons-arrow-left-20-solid" variant="outline" color="neutral" class="rounded-sm px-4 py-2">Zurück</UButton>
    <div v-else />
    <UButton v-if="hasNext" @click="emit('next')" size="md" trailing-icon="i-heroicons-arrow-right-20-solid" class="rounded-sm px-4 py-2">Nächste</UButton>
  </div>
</template>

<style scoped>

</style>
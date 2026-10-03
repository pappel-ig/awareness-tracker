<script setup lang="ts">
defineProps<{
  hasNext: boolean,
  hasPrev: boolean,
  canNext: boolean
}>()

const emit = defineEmits<{
  (e: 'next'): void
  (e: 'prev'): void
  (e: 'finish'): void
}>()
</script>

<template>
  <slot/>
  <UButton v-if="hasNext && !hasPrev" :disabled="!canNext" @click="emit('next')" size="md" trailing-icon="i-heroicons-arrow-right-20-solid" class="rounded-sm px-4 py-2">Umfrage Starten</UButton>
  <div class="flex items-center justify-between w-full mt-6" v-else>
    <UButton v-if="hasPrev" @click="emit('prev')" size="md" trailing-icon="i-heroicons-arrow-left-20-solid" variant="outline" color="neutral" class="rounded-sm px-4 py-2">Zurück</UButton>
    <div v-else />
    <div v-if="hasNext" class="flex items-center gap-3">
      <span v-if="!canNext" class="text-xs text-stone-500">Bitte beantworte alle mit <span class="text-error ms-0.5">*</span> notierten Fragen</span>
      <UButton :disabled="!canNext" @click="emit('next')" size="md" trailing-icon="i-heroicons-arrow-right-20-solid" class="rounded-sm px-4 py-2">Nächste</UButton>
    </div>
    <div v-else class="flex items-center gap-3">
      <span v-if="!canNext" class="text-xs text-stone-500">Bitte beantworte alle mit <span class="text-error ms-0.5">*</span> notierten Fragen</span>
      <UButton :disabled="!canNext" @click="emit('finish')" size="md" trailing-icon="i-heroicons-arrow-right-20-solid" class="rounded-sm px-4 py-2">Absenden</UButton>
    </div>
  </div>
</template>

<style scoped>

</style>

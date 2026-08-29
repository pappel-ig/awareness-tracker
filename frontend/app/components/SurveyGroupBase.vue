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
  <UCard class="w-full rounded-md shadow-sm p-9 mb-3">
    <div class="text-xs text-primary tracking-wider uppercase flex items-center gap-1.5 font-extrabold mb-2">
      <span>Forschungsprojekt</span>
      <span>·</span>
      <span>Jonas Metzger</span>
      <span>·</span>
      <span>Security Awareness</span>
    </div>
    <h1 class="font-serif text-3xl sm:text-4xl text-black mb-5 tracking-tight">
      Umfrage Security Awareness
    </h1>
    <slot/>
    <div class="flex items-center justify-between w-full mt-6">
      <UButton v-if="hasPrev" @click="emit('prev')" variant="outline" size="md" trailing-icon="i-heroicons-arrow-left-20-solid" class="rounded-sm px-4 py-2">Zurück</UButton>
      <div v-else />
      <UButton v-if="hasNext" @click="emit('next')" variant="outline" size="md" trailing-icon="i-heroicons-arrow-right-20-solid" class="rounded-sm px-4 py-2">Nächste</UButton>
    </div>
  </UCard>
</template>

<style scoped>

</style>
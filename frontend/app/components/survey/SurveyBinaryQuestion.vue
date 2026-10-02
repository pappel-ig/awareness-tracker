<script setup lang="ts">
defineProps<{
  label: string
}>()

const model = defineModel<boolean | null>({ required: true })

function select(value: boolean) {
  model.value = model.value === value ? null : value
}
</script>

<template>
  <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3 rounded-sm border border-default px-4 py-3">
    <span class="text-sm">{{ label }}</span>
    <UFieldGroup size="sm" class="shrink-0">
      <UButton
        icon="i-heroicons-check-20-solid"
        :color="model === true ? 'primary' : 'neutral'"
        :variant="model === true ? 'solid' : 'outline'"
        class="rounded-sm"
        @click="select(true)"
      >Ja</UButton>
      <UButton
        icon="i-heroicons-x-mark-20-solid"
        :color="model === false ? 'primary' : 'neutral'"
        :variant="model === false ? 'solid' : 'outline'"
        class="rounded-sm"
        @click="select(false)"
      >Nein</UButton>
    </UFieldGroup>
  </div>
</template>

<style scoped>

</style>

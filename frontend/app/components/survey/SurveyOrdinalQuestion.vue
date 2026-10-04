<script setup lang="ts">
withDefaults(defineProps<{
  label?: string
  lowLabel?: string
  highLabel?: string
}>(), {
  label: 'Wie besorgt bist du, dass solche Daten im Internet existieren?',
  lowLabel: 'Nicht besorgt',
  highLabel: 'Besorgt'
})

const model = defineModel<number | null>({ required: true })

const scale = [1, 2, 3, 4, 5]

function select(value: number) {
  model.value = model.value === value ? null : value
}
</script>

<template>
  <UFormField required :label="label">
    <div class="flex items-center gap-3 mt-3">
      <span class="text-xs text-stone-500 whitespace-nowrap">{{ lowLabel }}</span>
      <UFieldGroup size="sm" class="flex-1">
        <UButton
          v-for="value in scale"
          :key="value"
          :color="model === value ? 'primary' : 'neutral'"
          :variant="model === value ? 'solid' : 'outline'"
          class="rounded-sm flex-1 justify-center"
          @click="select(value)"
        >{{ value }}</UButton>
      </UFieldGroup>
      <span class="text-xs text-stone-500 whitespace-nowrap">{{ highLabel }}</span>
    </div>
  </UFormField>
</template>

<style scoped>

</style>

<script setup lang="ts">
const age_values = ['Keine Angabe', '0-18', '18-25', '26-45', '46- 55', '55+']
const survey = useSurveyState()

</script>

<template>
  <UFormField required label="Dein Alter">
    <URadioGroup orientation="horizontal" variant="table" v-model="survey.age" :items="age_values" />
  </UFormField>
  <UFormField class="mt-5" required label="Wie schätzt du dein IT-Wissen ein?" description="">
    <div class="flex items-center gap-3 mt-5">
      <span class="text-xs text-stone-500 whitespace-nowrap">Anfänger</span>
      <USlider v-model="survey.itKnowledge" />
      <span class="text-xs text-stone-500 whitespace-nowrap">Experte</span>
    </div>
  </UFormField>
  <UFormField class="mt-5" required label="Wie schätzt du deine Security Awareness ein?" description="">
    <div class="flex items-center gap-3 mt-5">
      <span class="text-xs text-stone-500 whitespace-nowrap">Niedrig</span>
      <USlider v-model="survey.securityAwareness" />
      <span class="text-xs text-stone-500 whitespace-nowrap">Hoch</span>
    </div>
  </UFormField>
</template>

<style scoped>

</style>
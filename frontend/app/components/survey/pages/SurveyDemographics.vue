<script setup lang="ts">
const age_values = ['Keine Angabe', '0-18', '18-25', '26-45', '46- 55', '55+']
const survey = useSurveyState()

</script>

<template>
  <UFormField required label="Dein Alter">
    <URadioGroup orientation="horizontal" variant="table" v-model="survey.age" :items="age_values" />
  </UFormField>
  <SurveyOrdinalQuestion class="mt-5" label="Wie schätzt du dein IT-Wissen ein?" low-label="Anfänger" high-label="Experte" v-model="survey.itKnowledge"/>
  <SurveyOrdinalQuestion class="mt-5" label="Wie schätzt du deine Security Awareness ein?" v-model="survey.securityAwareness"/>
</template>

<style scoped>

</style>
<script setup lang="ts">

import SurveyStart from "~/components/survey/pages/SurveyStart.vue";
import SurveyDemographics from "~/components/survey/pages/SurveyDemographics.vue";
import SurveyLeak from "~/components/survey/pages/SurveyLeak.vue";

const surveyGroup = [
    SurveyStart,
    SurveyDemographics,
    SurveyLeak
]

const currentIndex = ref(0);

function handleNext() {
  if (hasNext()) {
    currentIndex.value++
  }
}

function handlePrev() {
  if (hasPrev()) {
    currentIndex.value--
  }
}

function hasPrev() {
  return currentIndex.value > 0
}

function hasNext() {
  return currentIndex.value < surveyGroup.length - 1
}

</script>

<template>
  <Transition mode="out-in">
    <SurveyGroupBase :hasNext="hasNext()" :hasPrev="hasPrev()" @next="handleNext()" @prev="handlePrev()">
      <component :is="surveyGroup[currentIndex]" :key="currentIndex"/>
    </SurveyGroupBase>
  </Transition>
</template>

<style scoped>

</style>
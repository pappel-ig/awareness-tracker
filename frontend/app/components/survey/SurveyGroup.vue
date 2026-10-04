<script setup lang="ts">

import SurveyStart from "~/components/survey/pages/SurveyStart.vue";
import SurveyDemographics from "~/components/survey/pages/SurveyDemographics.vue";
import SurveyLeak from "~/components/survey/pages/SurveyLeak.vue";
import SurveyIp from "~/components/survey/pages/SurveyIp.vue";
import SurveyFingerprint from "~/components/survey/pages/SurveyFingerprint.vue";
import SurveyBehavior from "~/components/survey/pages/SurveyBehavior.vue";
import SurveyTracker from "~/components/survey/pages/SurveyTracker.vue";

const survey = useSurveyState()
const apiBase = useApiBase()
const route = useRoute()
const router = useRouter()
const banner = useBanner()

const surveyGroup = [
    SurveyStart,
    SurveyBehavior,
    SurveyDemographics,
    SurveyLeak,
    SurveyIp,
    SurveyFingerprint,
    SurveyTracker
]

const currentIndex = ref(0);
const pageValid = provideSurveyPageValid()

function handleNext() {
  if (hasNext() && pageValid.value) {
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

async function handleFinish() {
  const token = route.query.token
  $fetch("survey", {
    baseURL: apiBase,
    method: "POST",
    query: { token },
    headers: { "Content-Type": "application/json" },
    body: survey.value
  }).then(() => {
    router.push({
      name: 'end',
      query: {
        ...route.query
      }
    })
  }).catch(() => {
    banner.value = { title: 'Beim Absenden deiner Umfrage ist etwas schiefgelaufen :(', color: 'error' }
  })
}

</script>

<template>
  <Transition mode="out-in">
    <SurveyGroupBase :hasNext="hasNext()" :hasPrev="hasPrev()" :canNext="pageValid" @next="handleNext()" @prev="handlePrev()" @finish="handleFinish()">
      <component :is="surveyGroup[currentIndex]" :key="currentIndex"/>
    </SurveyGroupBase>
  </Transition>
</template>

<style scoped>

</style>

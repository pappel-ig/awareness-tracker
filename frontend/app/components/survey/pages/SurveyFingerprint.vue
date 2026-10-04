<script setup lang="ts">
import * as v from 'valibot'
import FingerprintJS from '@fingerprintjs/fingerprintjs'

const survey = useSurveyState()

useSurveyPageValidation(v.object({ fingerprintKnowledge: binaryAnswer, fingerprintScareFactor: ordinalAnswer }))

interface FingerprintInfo {
  visitorId: string
  confidence: number
  componentCount: number
}

const result = ref<FingerprintInfo | null>(null)
const loading = ref(true)

onMounted(async () => {
  try {
    const fp = await FingerprintJS.load()
    const fingerprint = await fp.get()
    result.value = {
      visitorId: fingerprint.visitorId,
      confidence: fingerprint.confidence.score,
      componentCount: Object.keys(fingerprint.components).length
    }
  } finally {
    loading.value = false
  }
})
</script>

<template>
  <p>
    Websites können deinen Browser auch ohne IP-Adresse und Cookies wiedererkennen. Dazu werden browserspezifische
    Merkmale sowie Eigenheiten deiner Hardware kombiniert und daraus ein einzigartiger digitaler Fingerabdruck erzeugt.
    Je nach Hardware und verwendetem Browser ist dieser mehr oder weniger eindeutig.
  </p>

  <div v-if="loading" class="mt-3 flex items-center gap-2 text-sm text-stone-500">
    <UIcon name="i-heroicons-arrow-path-20-solid" class="animate-spin" />
    Fingerabdruck wird berechnet...
  </div>

  <div v-else-if="result" class="mt-3 flex flex-wrap gap-0.5">
    <UBadge><b>Fingerabdruck-ID:</b> {{ result.visitorId }}</UBadge>
    <UBadge><b>Eindeutigkeit:</b> {{ Math.round(result.confidence * 100) }}%</UBadge>
    <UBadge><b>Erfasste Merkmale:</b> {{ result.componentCount }}</UBadge>
  </div>

  <USeparator class="mt-5" type="dashed" />

  <SurveyBinaryQuestion class="mt-5" label="Wusstest du, dass Websites einen solchen digitalen Fingerabdruck erzeugen können?" v-model="survey.fingerprintKnowledge"/>

  <SurveyOrdinalQuestion class="mt-5" label="Wie stark beunruhigt dich dieser Fingerabdruck?" low-label="wenig" high-label="stark" v-model="survey.fingerprintScareFactor"/>
</template>

<style scoped>

</style>

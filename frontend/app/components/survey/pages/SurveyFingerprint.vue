<script setup lang="ts">
import FingerprintJS from '@fingerprintjs/fingerprintjs'

const survey = useSurveyState()

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
    Websiten können deine Browsern auch ohne IP-Adresse und Cookies erkennen. Dabei werden browserspezifische Merkmale
    sowie auch Eigenarten bzgl. deiner verwendeten Hardware kombiniert. So können dich Websiten auch ganz ohne Cookies
    eindeutig identifizieren. Mithilfe von <span v-if="result">{{ result.componentCount }}</span> Merkmalen wird dann
    ein einzigartige Fingerabdruck erzeugt. Je nach Hardware und verwendeten Browser kann dieser eindeutiger sein.
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

  <UFormField class="mt-5">
    <UCheckbox v-model="survey.surveyFingerprintKnowledge" variant="card" label="Wusstest du das dein Browser/Hardware einen solchen digitalen Fingerabdruck haben?" orientation="horizontal"/>
  </UFormField>

  <UFormField class="mt-5" required label="Wie stark beunruhigt dich dieser Fingerabdruck?">
    <div class="flex items-center gap-3 mt-5">
      <span class="text-xs text-stone-500 whitespace-nowrap">Niedrig</span>
      <USlider v-model="survey.surveyFingerprintScareFactor" />
      <span class="text-xs text-stone-500 whitespace-nowrap">Hoch</span>
    </div>
  </UFormField>
</template>

<style scoped>

</style>

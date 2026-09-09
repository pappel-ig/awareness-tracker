<script setup lang="ts">
const survey = useSurveyState()
const apiBase = useApiBase()
const route = useRoute()

const token = route.query.token

interface IpResult {
  city: string | null
  country: string | null
  country_code: string | null
  asn: string | null
  isp: string | null
  ip: string | null
}

const { data: result } = await useFetch<IpResult>('information/ip', {
  baseURL: apiBase,
  method: 'GET',
  query: { token },
  headers: { 'Content-Type': 'application/json' }
})
</script>

<template>
  <p>
    Durch den Besuch dieser Umfrage übermittelst du deine IP-Adresse an den Web-Server. Mithilfe dieser lässt sich dein
    ungefährer Standort sowie dein Internet-Anbieter ermitteln:
  </p>

  <div class="mt-3 flex gap-0.5">
    <UBadge><b>IP:</b> {{ result?.ip || "nicht ermittelbar" }}</UBadge>
    <UBadge><b>Standort:</b> {{result?.country || "n/a"}}, {{result?.city || "n/a"}}</UBadge>
    <UBadge><b>Internet-Anbieter:</b> {{result?.isp || "nicht ermittelbar"}}</UBadge>
  </div>

  <USeparator class="mt-5" type="dashed" />

  <UFormField class="mt-5">
    <UCheckbox variant="card" label="Wusstest du das deine IP-Adresse ungefähre Standortdaten sowie dein Internet-Anbieter preisgibt?" orientation="horizontal" v-model="survey.surveyIpKnowledge"/>
  </UFormField>

  <UFormField class="mt-5" required label="Wie stark beunruhigen dich diese Daten?" description="">
    <div class="flex items-center gap-3 mt-5">
      <span class="text-xs text-stone-500 whitespace-nowrap">Niedrig</span>
      <USlider v-model="survey.surveyIpScareFactor" />
      <span class="text-xs text-stone-500 whitespace-nowrap">Hoch</span>
    </div>
  </UFormField>

</template>

<style scoped>

</style>
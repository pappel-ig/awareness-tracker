<script setup lang="ts">
import * as v from 'valibot'
const survey = useSurveyState()

useSurveyPageValidation(v.object({ ipKnowledge: binaryAnswer, ipScareFactor: ordinalAnswer }))
const apiBase = useApiBase()
const route = useRoute()

export interface IpResult {
  city: string | null
  country: string | null
  country_code: string | null
  asn: string | null
  isp: string | null
  ip: string | null
}

const token = route.query.token
const result = await $fetch<IpResult>('information/ip', {
  baseURL: apiBase,
  method: 'GET',
  query: { token },
  headers: { 'Content-Type': 'application/json' }
})

survey.value.meta.ip = result;
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

  <SurveyBinaryQuestion class="mt-5" label="Wusstest du das deine IP-Adresse ungefähre Standortdaten sowie dein Internet-Anbieter preisgibt?" v-model="survey.ipKnowledge"/>

  <SurveyOrdinalQuestion class="mt-5" label="Wie stark beunruhigen dich diese Daten?" v-model="survey.ipScareFactor"/>

</template>

<style scoped>

</style>
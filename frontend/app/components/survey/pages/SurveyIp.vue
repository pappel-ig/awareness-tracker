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
    Durch den Besuch dieser Umfrage übermittelst du deine IP-Adresse an den Webserver. Mithilfe der IP-Adresse lässt sich dein
    ungefährer Standort und dein Internet-Anbieter ermitteln:
  </p>

  <div class="mt-3 flex gap-0.5">
    <UBadge><b>IP:</b> {{ result?.ip || "nicht ermittelbar" }}</UBadge>
    <UBadge><b>Standort:</b> {{ [result?.city, result?.country].filter(Boolean).join(", ") || "nicht ermittelbar" }}</UBadge>
    <UBadge><b>Internet-Anbieter:</b> {{result?.isp || "nicht ermittelbar"}}</UBadge>
  </div>

  <USeparator class="mt-5" type="dashed" />

  <SurveyBinaryQuestion class="mt-5" label="Wusstest du, dass deine IP-Adresse deinen ungefähren Standort sowie deinen Internet-Anbieter verrät?" v-model="survey.ipKnowledge"/>

  <SurveyOrdinalQuestion class="mt-5" v-model="survey.ipScareFactor"/>

</template>

<style scoped>

</style>
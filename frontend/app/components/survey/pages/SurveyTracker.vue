<script setup lang="ts">
import * as v from 'valibot'
import type {TableColumn} from "#ui/components/Table.vue";
import {UBadge} from "#components";

const survey = useSurveyState()
const apiBase = useApiBase()
const route = useRoute()
const router = useRouter()
const token = route.query.token

interface TrackerResult {
  created_at: Date,
  headers: {
    "user-agent": string
  }
  remote_addr: string
}



if (!route.query.token) {
  router.replace({ path: '/start' })
}

const result = await $fetch<TrackerResult[]>(apiBase + "information/tracks", {
  method: "GET",
  query: {
    token
  },
  headers: {
    "Content-Type": "application/json"
  }
})
</script>

<template>
  <p>
    E-Mails erlauben das Bilder extern geladen werden können. So können beim Laden dieser externen Inhalte auch Informationen
    preisgegeben werden, welche auf den ersten Blick gar nicht bedacht werden. Solche Tracker nennt man Pixel Tracker.
    Manche E-Mail Clients oder Anbieter verbieten das Laden dieser Inhalte oder laden diese "Anonym".
    <br>
    In der E-Mail mit der Einladung ist ein solcher Pixel Tracker eingebaut. In der nachfolgenden Tabelle siehst du dabei
    wann du die E-Mail geöffnet hast bzw. wann der Tracking Pixel geladen wurde.
  </p>

  <UTable :data="result" class="flex-1" />

  <SurveyBinaryQuestion class="mt-5" label="Wusstest du das solche Informationen durch das Öffnen der E-Mail geteilt werden?" v-model="survey.trackingPixelKnowledge"/>
  <SurveyOrdinalQuestion class="mt-5" label="Wie stark beunruhigen dich diese Daten?" v-model="survey.leakScareFactor"/>
</template>

<style scoped>
</style>
<script setup lang="ts">
import {h} from 'vue'
import * as v from 'valibot'
import type {TableColumn} from "#ui/components/Table.vue";

const survey = useSurveyState()
const apiBase = useApiBase()
const route = useRoute()
const token = route.query.token
useSurveyPageValidation(v.object({ trackingPixelKnowledge: binaryAnswer, trackingPixelScareFactor: ordinalAnswer }))

export interface TrackerResult {
  created_at: Date,
  headers: {
    "user-agent": string
  }
  remote_addr: string
  remote_addr_info?: {
    isp?: string
    city?: string
  }
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

survey.value.meta.tracker = result;
survey.value.blockExternalData = result.length == 0

const columns: TableColumn<TrackerResult>[] = [
  {
    accessorKey: 'created_at',
    header: 'Datum',
    cell: ({ row }) => {
      return new Date(row.getValue('created_at')).toLocaleString('de-DE', {
        day: 'numeric',
        month: 'short',
        hour: '2-digit',
        minute: '2-digit',
        hour12: false
      })
    }
  },
  {
    id: 'connection',
    header: "Verbindung",
    cell: ({ row }) => {
      const info = row.original.remote_addr_info
      return h('div', [
        h('div', { class: 'font-medium' }, row.original.remote_addr),
        h('div', { class: 'text-sm text-muted' }, [info?.isp, info?.city].filter(Boolean).join(' - '))
      ])
    }
  },
  {
    accessorKey: 'headers.user-agent',
    header: "Browser/Gerät (User Agent)",
    meta: {
      class: {
        th: 'whitespace-normal break-words',
        td: 'whitespace-normal break-words'
      }
    }
  },
]

</script>

<template>
  <p>
    E-Mails können Bilder von externen Servern nachladen. So wird beim <i>Öffnen</i> der E-Mail das Bild geladen und dabei unter Umständen
    Daten an den Server übermittelt. Solche Tracker nennt man Tracking-Pixel.
    Manche E-Mail-Clients oder Anbieter blockieren das Laden dieser Inhalte oder laden sie über einen Proxy.
    <br>
    In der Einladungs-E-Mail ist ein solcher Tracking-Pixel eingebaut. In der nachfolgenden Tabelle siehst du,
    wann du die E-Mail geöffnet hast bzw. wann der Tracking-Pixel geladen wurde.
  </p>

  <UTable :data="result" :columns="columns" class="flex-1">
    <template #empty>
      <div class="flex flex-col items-center justify-center py-6 text-gray-500">
        <UIcon name="i-heroicons-face-smile" class="w-8 h-8 mb-2" />
        <span class="text-sm">
          Anscheinend hat dein E-Mail-Client beim Öffnen der E-Mail das Laden externer Inhalte blockiert.
        </span>
      </div>
    </template>
  </UTable>

  <USeparator class="mt-5" type="dashed" />

  <SurveyBinaryQuestion class="mt-5" label="Wusstest du, dass solche Informationen durch das Öffnen der E-Mail geteilt werden?" v-model="survey.trackingPixelKnowledge"/>
  <SurveyOrdinalQuestion class="mt-5" v-model="survey.trackingPixelScareFactor"/>
</template>

<style scoped>
</style>
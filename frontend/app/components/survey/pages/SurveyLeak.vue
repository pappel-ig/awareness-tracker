<script setup lang="ts">
import * as v from 'valibot'
import type {TableColumn} from "#ui/components/Table.vue";
import {UBadge} from "#components";

const survey = useSurveyState()

useSurveyPageValidation(v.object({ leakKnowledge: binaryAnswer, leakScareFactor: ordinalAnswer }))
const apiBase = useApiBase()
const route = useRoute()
const banner = useBanner()

const token = route.query.token

export interface LeakResult {
  breaches?: Breach[]
  leak_check: boolean
}

export interface Breach {
  AddedDate: Date,
  DataClasses: string[],
  Domain: string,
  Name: string
}

async function fetchLeaks(requestCheck: boolean) {
  try {
    if (requestCheck) await $fetch(apiBase + "participants/leaks", { method: "POST", query: { token } })
    for (let attempt = 0; ; attempt++) {
      const result = await $fetch<LeakResult>(apiBase + "information/leaks", { query: { token } })
      if (result.breaches) survey.value.meta.leak = [...new Set(result.breaches.flatMap(breach => breach.DataClasses))]
      if (!result.leak_check || result.breaches || attempt >= 30) return result
      await new Promise(resolve => setTimeout(resolve, 1000))
    }
  } catch {
    banner.value = { color: "error", title: "Die HaveIBeenPwned-Daten konnten leider nicht abgerufen werden."}
  }
}

const requestCheck = ref(false)
const { data: result, status, refresh } = useLazyAsyncData("survey-leaks", () => fetchLeaks(requestCheck.value))
const breaches = computed(() => result.value?.breaches ?? [])
const showAllBreaches = ref(false)

const columns: TableColumn<Breach>[] = [
  {
    accessorKey: 'Name',
    header: 'Datenleck',
    meta: {
      class: {
        th: 'w-[25%]',
        td: 'w-[25%] whitespace-normal break-words [overflow-wrap:anywhere] hyphens-auto'
      }
    },
    cell: ({ row }) => h('span', { lang: 'de' }, row.getValue<string>('Name')),
  },
  {
    accessorKey: 'DataClasses',
    header: 'Betroffene Daten',
    cell: ({ row }) => {
      return h('div', { class: 'flex flex-wrap gap-1' },
        row.getValue<string[]>('DataClasses').map(leak => {
          return h(UBadge, { class: 'capitalize', variant: 'outline', color: 'primary' }, () =>
            leak
          )
        })
      )
    }
  },
]
</script>

<template>
  <p>
    Im Internet existieren Datenlecks (Leaks), die unter Umständen persönliche Informationen wie Passwörter, Bankdaten oder auch Adressen
    enthalten.
    <span v-if="breaches.length > 0">Hier siehst du einen Teil der Informationen, die mit deiner E-Mail-Adresse verknüpft werden können:</span>
  </p>

  <UTable :data="showAllBreaches ? breaches : breaches.slice(0, 3)" :columns="columns" :loading="status === 'pending'" :ui="{ base: 'table-fixed w-full' }">
    <template #loading>
      <div class="py-6" />
    </template>
    <template v-if="result?.leak_check" #empty>
      <div class="flex flex-col items-center justify-center py-6 text-gray-500">
        <UIcon name="i-heroicons-face-smile" class="w-8 h-8 mb-2" />
        <span class="text-sm">
          Es wurden keine Datenlecks zu deiner E-Mail-Adresse gefunden. Das bedeutet jedoch nicht, dass keine Datenlecks vorhanden sind.
        </span>
      </div>
    </template>
    <template v-else #empty>
      <div class="flex flex-col items-center justify-center py-6 text-gray-500">
        <UIcon name="i-heroicons-face-frown" class="w-8 h-8 mb-2" />
        <span class="text-sm">
          Du hast bei der Anmeldung nicht zugestimmt, dass deine E-Mail-Adresse an HaveIBeenPwned.com übermittelt wird.
          Daher konnten keine Daten zu Datenlecks geladen werden. Du kannst das aber auch jetzt noch nachholen.
        </span>
        <UButton class="mt-2" variant="outline" color="neutral" @click="requestCheck = true; refresh()">Jetzt laden</UButton>
      </div>
    </template>
  </UTable>

  <div v-if="breaches.length > 3" class="mt-2 flex justify-center">
    <UButton color="neutral" variant="outline" @click="showAllBreaches = !showAllBreaches">
      {{ showAllBreaches ? 'Weniger anzeigen' : `Alle ${breaches.length} Einträge anzeigen` }}
    </UButton>
  </div>

  <USeparator v-if="breaches.length > 0" class="mt-5" type="dashed" />

  <SurveyBinaryQuestion class="mt-5" label="Wusstest du, dass solche Informationen im Internet existieren?" v-model="survey.leakKnowledge"/>
  <SurveyOrdinalQuestion class="mt-5" v-model="survey.leakScareFactor"/>
</template>

<style scoped>
</style>

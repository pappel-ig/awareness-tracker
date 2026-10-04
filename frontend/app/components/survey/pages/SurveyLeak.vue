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
  breaches: Breach[] | null
  leak_check: boolean
}

export interface Breach {
  AddedDate: Date,
  DataClasses: string[],
  Domain: string,
  Name: string
}

function fetchLeaks() {
  return $fetch<LeakResult>(apiBase + "information/leaks", {
    method: "GET",
    query: {
      token
    },
    headers: {
      "Content-Type": "application/json"
    }
  })
}

const { data: result, status } = useLazyAsyncData<LeakResult>("survey-leaks", fetchLeaks)
const polling = ref(false)
const loading = computed(() => status.value === 'pending' || polling.value)
const breaches = computed(() => result.value?.breaches ?? [])
const leakCheck = computed(() => result.value?.leak_check ?? false)

watch(() => result.value?.breaches, (value) => {
  if (value) {
    survey.value.meta.leak = [...new Set(value.flatMap(breach => breach.DataClasses))];
  }
}, { immediate: true })

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

const sleep =(ms: number) => new Promise(resolve => setTimeout(resolve, ms))
let unmounted = false
onBeforeUnmount(() => { unmounted = true })

async function updateLeakStatus() {
  if (polling.value) return
  polling.value = true
  try {
    await $fetch(apiBase + "participants/leaks", {
      method: "POST",
      query: {
        token
      },
      headers: {
        "Content-Type": "application/json"
      }
    })

    for (let attempt = 0; attempt < 30; attempt++) {
      await sleep(1000)
      if (unmounted) return
      result.value = await fetchLeaks()
      if (result.value.breaches != null) return
    }
  } catch (error) {
    banner.value = { color: "error", title: "Die HaveIBeenPwned-Daten konnten leider nicht abgerufen werden."}
  } finally {
    polling.value = false
  }
}

const showAllBreaches = ref(false)
const visibleBreaches = computed(() =>
  showAllBreaches.value ? breaches.value : breaches.value.slice(0, 3)
)
</script>

<template>
  <p>
    Im Internet existieren Datenlecks (Leaks), die unter Umständen persönliche Informationen wie Passwörter, Bankdaten oder auch Adressen
    enthalten.
    <span v-if="!loading && breaches.length > 0 && leakCheck">Hier siehst du einen Teil der Informationen, die mit deiner E-Mail-Adresse verknüpft werden können:</span>
  </p>

  <UTable :data="visibleBreaches" :columns="columns" :loading="loading" :ui="{ base: 'table-fixed w-full' }">
    <template v-if="leakCheck" #empty>
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
        <UButton class="mt-2" variant="outline" color="neutral" :loading="loading" @click="updateLeakStatus()">Jetzt laden</UButton>
      </div>
    </template>
  </UTable>

  <div v-if="breaches.length > 3 && leakCheck" class="mt-2 flex justify-center">
    <UButton color="neutral" variant="outline" @click="showAllBreaches = !showAllBreaches">
      {{ showAllBreaches ? 'Weniger anzeigen' : `Alle ${breaches.length} Einträge anzeigen` }}
    </UButton>
  </div>

  <USeparator v-if="breaches.length > 0 && leakCheck" class="mt-5" type="dashed" />

  <SurveyBinaryQuestion class="mt-5" label="Wusstest du, dass solche Informationen im Internet existieren?" v-model="survey.leakKnowledge"/>
  <SurveyOrdinalQuestion class="mt-5" v-model="survey.leakScareFactor"/>
</template>

<style scoped>
</style>

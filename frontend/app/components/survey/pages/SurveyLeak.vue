<script setup lang="ts">
import type {TableColumn} from "#ui/components/Table.vue";
import {UBadge} from "#components";

const survey = useSurveyState()
const apiBase = useApiBase()
const route = useRoute()
const router = useRouter()

const token = route.query.token

interface LeakResult {
  breaches: Breach[]
  leak_check: boolean
}

interface Breach {
  AddedDate: Date,
  DataClasses: string[],
  Domain: string,
  Name: string
}

if (!route.query.token) {
  router.replace({ path: '/start' })
}

const params = new URLSearchParams()
params.append("token", String(token ?? ""))
const result = await $fetch<LeakResult>(apiBase + "information/leaks", {
  method: "GET",
  query: {
    token
  },
  headers: {
    "Content-Type": "application/json"
  }
})

const columns: TableColumn<Breach>[] = [
  {
    accessorKey: 'Name',
    header: 'Leak',
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
    header: 'Daten',
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

const showAllBreaches = ref(false)
const visibleBreaches = computed(() =>
  showAllBreaches.value ? result.breaches : result.breaches.slice(0, 3)
)
</script>

<template>
  <p>
    Im Internet existieren Leaks die unter Umständen persönliche Informationen wie Passwörter, Bankdaten oder auch Adressen
    beinhalten.
    <span v-if="result.breaches.length > 0 && result.leak_check">Hier ist ein Teil deiner Informationen, die mit deiner E-Mail verknüpft werden können:</span>
    <span v-if="!(result.breaches.length > 0) && result.leak_check">Zu deiner E-Mail Adresse konnten keine bekannten Datenlecks gefunden werden!</span>
    <span v-if="!result.leak_check">Du hast bei der Anmeldung der Verarbeitung mit HaveIBeenPwned nicht zugestimmt... Du kannst <a href="https://haveibeenpwned.com/">hier</a> deine E-Mail manuell überprüfen!</span>

  </p>

  <UTable v-if="result.breaches.length > 0 && result.leak_check" :data="visibleBreaches" :columns="columns" :ui="{ base: 'table-fixed w-full' }"></UTable>

  <div v-if="result.breaches.length > 0 && result.leak_check && result.breaches.length > 3" class="mt-2 flex justify-center">
    <UButton color="neutral" variant="outline" @click="showAllBreaches = !showAllBreaches">
      {{ showAllBreaches ? 'Weniger anzeigen' : `Alle ${result.breaches.length} Einträge anzeigen` }}
    </UButton>
  </div>

  <USeparator v-if="result.breaches.length > 0 && result.leak_check" class="mt-5" size="sm" type="dashed" />

  <UFormField class="mt-5">
    <UCheckbox variant="card" label="Wusstest du das solche Informationen im Internet existieren?" orientation="horizontal" v-model="survey.surveyLeakKnowledge"/>
  </UFormField>

  <UFormField class="mt-5" required label="Wie stark beunruhigen dich diese Daten?" description="">
    <div class="flex items-center gap-3 mt-5">
      <span class="text-xs text-stone-500 whitespace-nowrap">Niedrig</span>
      <USlider v-model="survey.surveyLeakScareFactor" />
      <span class="text-xs text-stone-500 whitespace-nowrap">Hoch</span>
    </div>
  </UFormField>
</template>

<style scoped>
</style>
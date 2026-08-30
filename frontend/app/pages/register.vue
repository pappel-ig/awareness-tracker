<script setup lang="ts">
import * as v from 'valibot'
import type { FormSubmitEvent } from '@nuxt/ui'

const registerSchema = v.object({
  email: v.pipe(v.string(), v.email("Ungültige E-Mail")),
  processingDataOk: v.pipe(v.boolean(), v.literal(true, "Du musst der Verarbeitung deiner Daten zustimmen")),
  emailDataLeakCheckOk: v.boolean()
})

type Schema = v.InferOutput<typeof registerSchema>

const state = reactive({
  email: '',
  processingDataOk: false,
  emailDataLeakCheckOk: false
})

const apiBase = useApiBase()
const banner = useBanner()

const status = ref<'form' | 'ok'>('form')

async function onSubmit(event: FormSubmitEvent<Schema>) {
  try {
    const result = await $fetch<{ status: 'ok' }>(apiBase + "participants", {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: {
        "email": event.data.email,
        "leak_check": event.data.emailDataLeakCheckOk
      }
    })
    if (result.status === 'ok') {
      status.value = 'ok'
    }
  } catch {
    banner.value = { title: 'Da ist ein Fehler passiert. Bitte versuche es erneut.', color: 'error' }
  }
}

</script>

<template>
  <template v-if="status === 'form'">
    <p class="text-black text-md leading-relaxed mb-6">
      Vielen Dank das du bei der Umfrage mitmachen möchtest. Trage dazu bitte deine E-Mail in das unten stehende Formular ein.
      Du bekommst dann eine E-Mail an die hinterlegte Adresse, sobald sich genügend Leute registriert haben.
    </p>
    <UForm :schema="registerSchema" :state="state" class="space-y-4 mt-5" @submit="onSubmit">
      <UFormField label="Email" name="email">
        <UInput placeholder="mail@example.org" v-model="state.email" class="w-full" />
      </UFormField>
      <UFormField name="emailDataLeakCheckOk">
        <UCheckbox v-model="state.emailDataLeakCheckOk" label="E-Mail-Abgleich" description="Zwecks Überprüfung auf Datenlecks wird deine E-Mail Adresse einmalig an XposedOrNot übermittelt. Der Anbieter ist OpenSource und speichert die E-Mail Adresse nicht."/>
      </UFormField>
      <UFormField name="processingDataOk">
        <UCheckbox v-model="state.processingDataOk" description="Ich willige ein, dass meine Antworten sowie technische Informationen des Browser bzw. des E-Mail Clients für das Forschungsprojekt verarbeitet werden.">
          <template #label>
            Der Datenverarbeitung zustimmen <span class="text-red-500">*</span>
          </template>
        </UCheckbox>
      </UFormField>
      <UButton type="submit">Anmelden</UButton>
    </UForm>
  </template>
  <Status v-else description="Du hast dich erfolgreich für die Umfrage registriert. Du bekommst beim Start der Umfrage eine E-Mail. Vielen Dank das du bei der Umfrage mitmachst!"/>
</template>

<style scoped>

</style>
<script setup lang="ts">
import * as v from 'valibot'
import type { FormSubmitEvent } from '@nuxt/ui'
import type {Turnstile} from "#components";

const registerSchema = v.object({
  email: v.pipe(v.string(), v.email("Ungültige E-Mail-Adresse")),
  processingDataOk: v.pipe(v.boolean(), v.literal(true, "Du musst der Verarbeitung deiner Daten zustimmen")),
  emailDataLeakCheckOk: v.boolean(),
  turnstileToken: v.pipe(v.string(), v.minLength(1, "Bitte bestätige, dass du kein Bot bist"))
})

type Schema = v.InferOutput<typeof registerSchema>

const state = reactive({
  email: '',
  processingDataOk: false,
  emailDataLeakCheckOk: false,
  turnstileToken: ''
})

const apiBase = useApiBase()
const turnstileSiteKey = useTurnstileSiteKey()
const banner = useBanner()

const status = ref<'form' | 'ok'>('form')
const turnstile = ref<InstanceType<typeof Turnstile> | null>(null)

async function onSubmit(event: FormSubmitEvent<Schema>) {
  try {
    const result = await $fetch<{ status: 'ok' }>(apiBase + "participants", {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: {
        "email": event.data.email,
        "leak_check": event.data.emailDataLeakCheckOk,
        "turnstile_token": event.data.turnstileToken
      }
    })
    if (result.status === 'ok') {
      status.value = 'ok'
    }
  } catch (error: any) {
    if (error?.response?.status === 403) {
      banner.value = { title: 'Deine Anfrage wurde als Spam erkannt. Bitte lade die Seite neu und versuche es erneut.', color: 'error' }
    } else {
      banner.value = { title: 'Es ist ein Fehler aufgetreten. Bitte versuche es erneut.', color: 'error' }
    }
    state.turnstileToken = ''
    turnstile.value?.reset()
  }
}

function onTurnstileError() {
  state.turnstileToken = ''
  banner.value = { title: 'Spam-Schutz konnte nicht überprüft werden. Bitte lade die Seite neu und versuche es erneut.', color: 'error' }
}

</script>

<template>
  <template v-if="status === 'form'">
    <p class="text-black text-md leading-relaxed mb-6">
      Vielen Dank, dass du bei der Umfrage mitmachen möchtest. Trage dazu bitte deine E-Mail-Adresse in das unten stehende Formular ein.
      Du bekommst dann einen Link zu deiner individuellen Umfrage an deine E-Mail-Adresse.
    </p>
    <UForm :schema="registerSchema" :state="state" class="space-y-4 mt-5" @submit="onSubmit">
      <UFormField label="E-Mail" name="email">
        <UInput placeholder="mail@example.org" v-model="state.email" class="w-full" />
      </UFormField>
      <UFormField name="emailDataLeakCheckOk">
        <UCheckbox v-model="state.emailDataLeakCheckOk" label="Auf Datenlecks prüfen (optional)" description="Zwecks Überprüfung auf Datenlecks wird deine E-Mail-Adresse einmalig an HaveIBeenPwned übermittelt."/>
      </UFormField>
      <UFormField name="processingDataOk">
        <UCheckbox v-model="state.processingDataOk" description="Ich willige ein, dass meine Antworten sowie technische Informationen des Browsers bzw. des E-Mail-Clients für das Forschungsprojekt verarbeitet werden. Zwecks Spam-Schutz wird Cloudflare Turnstile verwendet.">
          <template #label>
            Der Datenverarbeitung zustimmen <span class="text-red-500">*</span>
          </template>
        </UCheckbox>
      </UFormField>
      <UFormField name="turnstileToken">
        <Turnstile
          ref="turnstile"
          :sitekey="turnstileSiteKey"
          @verified="(token) => state.turnstileToken = token"
          @expired="() => state.turnstileToken = ''"
          @error="onTurnstileError"
        />
      </UFormField>
      <UButton type="submit">Anmelden</UButton>
    </UForm>
  </template>
  <Status v-else description="Du hast dich erfolgreich für die Umfrage registriert. Du bekommst beim Start der Umfrage eine E-Mail. Vielen Dank, dass du an der Umfrage teilnimmst!"/>
</template>

<style scoped>

</style>
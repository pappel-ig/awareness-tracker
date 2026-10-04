<script setup lang="ts">
import * as v from 'valibot'
import type {BehaviorStatement} from "~/composables/useSurveyState";

const statements: { key: BehaviorStatement, label: string }[] = [
  { key: 'vpn', label: 'Ich benutze ein VPN' },
  { key: 'passwordReuse', label: 'Ich verwende das gleiche Passwort für mehrere Dienste' },
  { key: 'passwordChange', label: 'Ich ändere Passwörter regelmäßig' },
  { key: 'passwordManager', label: 'Ich benutze einen Passwort-Manager' },
  { key: 'passkeys', label: 'Ich benutze, wo möglich, Passkeys' },
  { key: 'twoFactor', label: 'Ich nutze Zwei-Faktor-Authentifizierung, wo es möglich ist' },
  { key: 'updates', label: 'Ich installiere Updates zeitnah' },
  { key: 'publicWifi', label: 'Ich verbinde mich mit öffentlichen WLANs' },
  { key: 'adBlocker', label: 'Ich benutze einen Werbe- bzw. Tracking-Blocker' },
]

const survey = useSurveyState()
useSurveyPageValidation(v.object(
    Object.fromEntries(statements.map(s => [s.key, binaryAnswer]))
))
</script>

<template>
  <p>
    Bitte gib an, welche der folgenden Aussagen auf dich zutreffen. Wenn du mit einer Aussage nichts anfangen
    kannst, wähle <i>Nein</i>.
  </p>

  <USeparator class="mt-5" type="dashed" />

  <div class="mt-5 flex flex-col gap-2">
    <SurveyBinaryQuestion
      v-for="statement in statements"
      :key="statement.key"
      :label="statement.label"
      v-model="survey[statement.key]"
    />
  </div>
</template>

<style scoped>

</style>

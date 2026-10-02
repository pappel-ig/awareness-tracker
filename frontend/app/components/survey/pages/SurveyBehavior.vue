<script setup lang="ts">
import type {BehaviorStatement} from "~/composables/useSurveyState";

const survey = useSurveyState()

const statements: { key: BehaviorStatement, label: string }[] = [
  { key: 'vpn', label: 'Ich benutze ein VPN' },
  { key: 'passwordReuse', label: 'Ich benutze ein einzelne Zugangsdaten wieder' },
  { key: 'passwordChange', label: 'Ich ändere Zugangsdaten regelmäßig' },
  { key: 'passwordManager', label: 'Ich benutze einen Passwort-Manager' },
  { key: 'passkeys', label: 'Ich benutze, wo möglich, Passkeys' },
  { key: 'twoFactor', label: 'Ich nutze Zwei-Faktor-Authentifizierung, wo es möglich ist' },
  { key: 'updates', label: 'Ich installiere Updates zeitnah' },
  { key: 'publicWifi', label: 'Ich verbinde mich mit öffentlichen WLAN-Netzen' },
  { key: 'adBlocker', label: 'Ich benutze einen Werbe- bzw. Tracking-Blocker' },
]
</script>

<template>
  <p>
    Nachfolgend findest du ein Aussagen, bitte kreuze die zutreffenden Aussagen an. Wenn du mit der Aussage nichts anfangen
    kannst, dann kannst du <i>Nein</i> ankreuzen.
  </p>

  <USeparator class="mt-5" type="dashed" />

  <div class="mt-5 flex flex-col gap-2">
    <SurveyBinaryQuestion
      v-for="statement in statements"
      :key="statement.key"
      :label="statement.label"
      v-model="survey.surveyBehavior[statement.key]"
    />
  </div>
</template>

<style scoped>

</style>

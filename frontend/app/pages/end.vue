<script setup lang="ts">
import type {SurveyState} from "~/composables/useSurveyState";

interface Recommendation {
  key: string
  severity: 'extreme' | 'high' | 'medium' | 'low'
  title: string
  when: ((s: SurveyState) => boolean)[]
  text: string
}

const severityMap = {
  extreme: 4,
  high: 3,
  medium: 2,
  low: 1
}

type Predicate = (s: SurveyState) => boolean
const and = (...ps: Predicate[]): Predicate => s => ps.every(p => p(s))
const not = (p: Predicate): Predicate => s => !p(s)

const passwordLeaked = (s: SurveyState) => s.meta.leak?.includes("Passwords") ?? false

const usesVpn = (s: SurveyState) => s.vpn ?? false
const reusesPasswords = (s: SurveyState) => s.passwordReuse ?? false
const changesPasswords = (s: SurveyState) => s.passwordChange ?? false
const usesPasswordManager = (s: SurveyState) => s.passwordManager ?? false
const usesPasskeys = (s: SurveyState) => s.passkeys ?? false
const usesTwoFactor = (s: SurveyState) => s.twoFactor ?? false
const updates = (s: SurveyState) => s.updates ?? false
const publicWifi = (s: SurveyState) => s.publicWifi ?? false
const adblocker = (s: SurveyState) => s.adBlocker ?? false

const recommendations: Recommendation[] = [
  {
    key: 'change-passwords',
    severity: 'extreme',
    title: 'Passwörter ändern',
    when: [
        and(
            not(changesPasswords),
            reusesPasswords,
            passwordLeaked
        )
    ],
    text: 'Deine Passwörter sind in öffentlichen Leaks vorhanden. Du solltest dringend deine Passwörter ändern! Da du Passwörter wiederverwendest und diese nicht regelmäßig änderst, steht die Chance hoch, dass sich derzeit Leute auch in andere Account einloggen können!'
  },
  {
    key: 'use-passwordmanager',
    severity: "low",
    title: "Passwort Manager verwenden",
    when: [
        not(usesPasswordManager)
    ],
    text: "Du könntest einen Passwort Manager ausprobieren! Die meisten Passwort Manager synchronisieren deine Passwörter automatisch an alle Geräte. Desweiteren bieten Passwort Manager auch eine Autofill Methode an!"
  },
  {
    key: 'use-2fa',
    severity: "low",
    title: "2-Faktor-Authentifizierung verwenden",
    when: [
      not(usesTwoFactor)
    ],
    "text": "2-Faktor-Authentifizierung verbessert die Sicherheit deiner Logins. Sie erhöht den Schutz vor Phishing und Datenlecks und bildet so eine zusätzliche Barriere."
  },
  {
    key: 'use-passkeys',
    severity: "low",
    title: 'Passkeys ausprobieren',
    when: [
      not(usesPasskeys)
    ],
    text: 'Passkeys erlauben eine sehr komfortable passwortfreie Anmeldung und bieten einen sicheren Schutz gegenüber Phishing.'
  },
  {
    key: 'public-wifi',
    severity: "low",
    title: 'Vorsicht in öffentlichen Netzwerken!',
    when: [
      publicWifi
    ],
    text: 'Im öffentlichen Netzwerken kann potenziell jeder mitlesen. Es ist gesund diese als "feindliche" Umgebung zu betrachten! Falls möglich könntest du dich mit einen VPN verbinden, dieser bietet einen gewissen Schutz vor dem mitlesen.'
  },
  {
    key: 'update',
    severity: "low",
    title: 'Updates regelmäßig durchführen',
    when: [
      not(updates)
    ],
    text: 'Regelmäßiges updaten der Geräte führt. Angreifer nutzen oft bereits bekannte Sicherheitslücken aus, die bereits in Updates behoben wurde. Halte daher deine Geräte immer auf den aktuellsten Stand.'
  }
]

const survey = useSurveyState()

if (import.meta.dev && useRoute().query.mock !== undefined) {
  const {surveyMock} = await import('~/mocks/surveyMock')
  Object.assign(survey.value, surveyMock)
}


const matched = computed(() => {
  const best = new Map<string, Recommendation>()
  for (const r of recommendations) {
    if (!r.when.some(fn => fn(survey.value))) continue
    const current = best.get(r.key)
    if (!current || r.severity > current.severity) best.set(r.key, r)
  }
  return [...best.values()].sort((a, b) => severityMap[b.severity] - severityMap[a.severity])
})

const recommendationSeverityMap: { [key: string]: "error" | "warning" | "info" | "neutral" | "primary" | "secondary" | "success" | undefined } = {
  extreme: "error",
  high: "warning",
  medium: "info",
  low: "neutral",
}
</script>

<template>
  <p>Vielen Dank das du bei der Umfrage mitgemacht hast! Nachfolgend findest du eine Auswertung deiner Antworten</p>

  <div v-if="matched.length" class="mt-5 flex flex-col gap-3">
    <UAlert
      v-for="recommendation in matched"
      :key="recommendation.title"
      :color="recommendationSeverityMap[recommendation.severity]"
      variant="subtle"
      icon="i-lucide-shield-alert"
      :title="recommendation.title"
      :description="recommendation.text"
    />
  </div>
  <p v-else class="mt-5">
    Auf Basis deiner Antworten haben wir keine speziellen Empfehlungen für dich. Weiter so!
  </p>
</template>

<style scoped>

</style>

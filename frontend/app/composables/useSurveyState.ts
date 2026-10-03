import type {IpResult} from "~/components/survey/pages/SurveyIp.vue";
import type {LeakResult} from "~/components/survey/pages/SurveyLeak.vue";
import type {TrackerResult} from "~/components/survey/pages/SurveyTracker.vue";

export type BehaviorStatement =
    | 'vpn'
    | 'passwordReuse'
    | 'passwordChange'
    | 'passwordManager'
    | 'passkeys'
    | 'twoFactor'
    | 'updates'
    | 'publicWifi'
    | 'adBlocker'

export type SurveyState = {
  meta: {
    ip: IpResult | null,
    leak: LeakResult | null,
    tracker: TrackerResult[] | null
  }
  age: string
  itKnowledge: number | null,
  securityAwareness: number | null,
  leakKnowledge: boolean | null,
  leakScareFactor: number | null,
  ipKnowledge: boolean | null,
  ipScareFactor: number | null,
  fingerprintKnowledge: boolean | null,
  fingerprintScareFactor: number | null,
  surveyBehavior: Record<BehaviorStatement, boolean | null>,
  trackingPixelKnowledge: boolean | null,
  trackingPixelScareFactor: number | null
}

export function useSurveyState() {
  return useState<SurveyState>('survey', () => ({
    meta: {
      ip: null,
      leak: null,
      tracker: null
    },
    age: 'Keine Angabe',
    itKnowledge: null,
    securityAwareness: null,
    leakKnowledge: null,
    leakScareFactor: null,
    ipKnowledge: null,
    ipScareFactor: null,
    fingerprintKnowledge: null,
    fingerprintScareFactor: null,
    surveyBehavior: {
      vpn: null,
      passwordReuse: null,
      passwordChange: null,
      passwordManager: null,
      passkeys: null,
      twoFactor: null,
      updates: null,
      publicWifi: null,
      adBlocker: null,
    },
    trackingPixelKnowledge: null,
    trackingPixelScareFactor: null
  }))
}

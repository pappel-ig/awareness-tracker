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
    | 'acceptCookies'

export type SurveyState = {
  age: string
  itKnowledge: number | null,
  securityAwareness: number | null,
  surveyLeakKnowledge: boolean | null,
  surveyLeakScareFactor: number | null,
  surveyIpKnowledge: boolean | null,
  surveyIpScareFactor: number | null,
  surveyFingerprintKnowledge: boolean | null,
  surveyFingerprintScareFactor: number | null,
  surveyBehavior: Record<BehaviorStatement, boolean | null>,
}

export function useSurveyState() {
  return useState<SurveyState>('survey', () => ({
    age: 'Keine Angabe',
    itKnowledge: null,
    securityAwareness: null,
    surveyLeakKnowledge: null,
    surveyLeakScareFactor: null,
    surveyIpKnowledge: null,
    surveyIpScareFactor: null,
    surveyFingerprintKnowledge: null,
    surveyFingerprintScareFactor: null,
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
      acceptCookies: null,
    },
  }))
}

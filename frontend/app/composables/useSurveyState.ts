export type SurveyState = {
  age: string
  itKnowledge: number
  securityAwareness: number
}

export function useSurveyState() {
  return useState<SurveyState>('survey', () => ({
    age: 'Keine Angabe',
    itKnowledge: 0,
    securityAwareness: 0,
  }))
}

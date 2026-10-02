import * as v from 'valibot'
import type {ComputedRef, InjectionKey, ShallowRef} from "vue";

type ActivePage = ShallowRef<ComputedRef<boolean> | null>

const activePageKey: InjectionKey<ActivePage> = Symbol('surveyActivePage')

export const ordinalAnswer = v.number()
export const binaryAnswer = v.boolean()

export function provideSurveyPageValid() {
  const activePage: ActivePage = shallowRef(null)
  provide(activePageKey, activePage)
  return computed(() => activePage.value?.value ?? true)
}

export function useSurveyPageValidation(schema: v.GenericSchema) {
  const survey = useSurveyState()
  const activePage = inject(activePageKey, null)

  const valid = computed(() => v.safeParse(schema, survey.value).success)

  if (activePage) {
    activePage.value = valid
    onUnmounted(() => {
      if (activePage.value === valid) {
        activePage.value = null
      }
    })
  }

  return valid
}

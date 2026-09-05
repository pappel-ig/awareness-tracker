<script setup lang="ts">
declare global {
  interface Window {
    turnstile?: {
      render: (container: string | HTMLElement, options: Record<string, unknown>) => string
      remove: (widgetId: string) => void
      reset: (widgetId: string) => void
    }
    __turnstileOnLoad__?: () => void
  }
}

const props = defineProps<{ sitekey: string }>()
const emit = defineEmits<{
  verified: [token: string]
  expired: []
  error: []
}>()

const el = ref<HTMLElement | null>(null)
let widgetId: string | undefined

function renderWidget() {
  if (!window.turnstile || !el.value) return
  widgetId = window.turnstile.render(el.value, {
    sitekey: props.sitekey,
    appearance: 'interaction-only',
    callback: (token: string) => emit('verified', token),
    'expired-callback': () => emit('expired'),
    'error-callback': () => emit('error')
  })
}

function loadScript(): Promise<void> {
  return new Promise((resolve) => {
    if (window.turnstile) {
      resolve()
      return
    }
    if (document.querySelector('script[data-turnstile]')) {
      window.__turnstileOnLoad__ = resolve
      return
    }
    window.__turnstileOnLoad__ = resolve
    const script = document.createElement('script')
    script.src = 'https://challenges.cloudflare.com/turnstile/v0/api.js?onload=__turnstileOnLoad__&render=explicit'
    script.async = true
    script.defer = true
    script.dataset.turnstile = 'true'
    document.head.appendChild(script)
  })
}

onMounted(async () => {
  await loadScript()
  renderWidget()
})

onBeforeUnmount(() => {
  if (widgetId && window.turnstile) {
    window.turnstile.remove(widgetId)
  }
})

function reset() {
  if (widgetId && window.turnstile) {
    window.turnstile.reset(widgetId)
  }
}

defineExpose({ reset })
</script>

<template>
  <div ref="el" />
</template>

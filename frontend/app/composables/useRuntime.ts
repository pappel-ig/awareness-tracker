type RuntimeEnv = {
  API_BASE?: string
  TURNSTILE_SITE_KEY?: string
}

function getRuntimeEnv(): RuntimeEnv | undefined {
  return typeof window !== 'undefined' ? (window as unknown as { __ENV__?: RuntimeEnv }).__ENV__ : undefined
}

export function useApiBase(): string {
  const config = useRuntimeConfig()
  return getRuntimeEnv()?.API_BASE || config.public.apiBase
}

export function useTurnstileSiteKey(): string {
  const config = useRuntimeConfig()
  return getRuntimeEnv()?.TURNSTILE_SITE_KEY || config.public.turnstileSiteKey
}

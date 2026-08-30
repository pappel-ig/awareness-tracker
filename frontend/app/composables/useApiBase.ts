export function useApiBase(): string {
  const config = useRuntimeConfig()
  const runtimeOverride = (typeof window !== 'undefined' ? (window as unknown as { __ENV__?: { API_BASE?: string } }).__ENV__?.API_BASE : undefined)
  return runtimeOverride || config.public.apiBase
}

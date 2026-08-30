export type BannerType = {
  color?: "primary" | "secondary" | "success" | "info" | "warning" | "error" | "neutral"
  title: string
}

export function useBanner() {
  return useState<BannerType | null>('banner', () => null)
}

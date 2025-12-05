import { browser } from '$app/environment'
import type { Color } from '$lib/types/color'

export const themes = ['dark', 'light', 'system'] as const
export type Theme = (typeof themes)[number]

export type UiState = {
  theme: Theme
  color: Color
  loading: boolean
  leftMenuOpen: boolean
  leftMenuWidth: number
  appliedTheme: Theme
}

let theme: Theme = $state('system')
let color: Color = $state('pink')
let loading = $state(true)
let leftMenuOpen = $state(true)
let leftMenuWidth = $state(326)

const appliedTheme: Theme = $derived.by(() => {
  if (browser) {
    if (theme === 'system') {
      return window.matchMedia('(prefers-color-scheme: dark)').matches
        ? 'dark'
        : 'light'
    }
    return theme
  }
  return 'dark'
})

export const useUiStore: () => UiState = () => {
  $effect(() => {
    if (browser && loading) {
      loading = false
    }
  })

  return {
    get theme() {
      return theme
    },
    set theme(value) {
      theme = value
    },
    get color() {
      return color
    },
    set color(value) {
      color = value
    },
    get loading() {
      return loading
    },
    get leftMenuOpen() {
      return leftMenuOpen
    },
    set leftMenuOpen(value) {
      leftMenuOpen = value
    },
    get leftMenuWidth() {
      return leftMenuWidth
    },
    set leftMenuWidth(value) {
      leftMenuWidth = value
    },
    get appliedTheme() {
      return appliedTheme
    },
  }
}

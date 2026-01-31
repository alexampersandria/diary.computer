import { browser } from '$app/environment'
import type { Color } from '$lib/types/color'
import type { DateObject } from '$lib/types/date'
import { formatDate, objectizeDate } from '$lib/utils/date'

export const themes = ['dark', 'light', 'system'] as const
export type Theme = (typeof themes)[number]

export type UiDate = {
  date: Date
  formatted: string
  object: DateObject
  update: () => void
}

export type UiState = {
  theme: Theme
  color: Color
  loading: boolean
  leftMenuOpen: boolean
  leftMenuWidth: number
  appliedTheme: Theme
  date: UiDate
}

let theme: Theme = $state('system')
let color: Color = $state('pink')
let loading = $state(true)
let leftMenuOpen = $state(true)
let leftMenuWidth = $state(326)

let dateValue = new Date()
const dateFormatted = $derived.by(() => formatDate(dateValue))
const dateObject = $derived.by(() => objectizeDate(dateValue))

// #TODO: implement automatic updates based on some timing function
const updateDate = () => {
  dateValue = new Date()
}

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

    get date() {
      return {
        date: dateValue,
        formatted: dateFormatted,
        object: dateObject,
        update: updateDate,
      }
    },
  }
}

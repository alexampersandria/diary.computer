<script lang="ts">
import { useUiStore, type Theme } from '$lib/store/uiStore.svelte'
import { Blend, Moon, Spotlight } from 'lucide-svelte'

let uiStore = useUiStore()

const setTheme = (theme: Theme) => {
  uiStore.theme = theme
}
</script>

<div class="theme-picker button-icon-group">
  <button
    class="plain"
    class:active={uiStore.theme === 'system'}
    title="System"
    aria-label="Automatic Theme"
    onclick={() => {
      setTheme('system')
    }}>
    <Blend />
  </button>
  <button
    class="plain"
    class:active={uiStore.theme === 'dark'}
    title="Dark"
    aria-label="Dark Theme"
    onclick={() => {
      setTheme('dark')
    }}>
    <Moon />
  </button>
  <button
    class="plain"
    class:active={uiStore.theme === 'light'}
    title="Light"
    aria-label="Light Theme"
    onclick={() => {
      setTheme('light')
    }}>
    <Spotlight />
  </button>
</div>

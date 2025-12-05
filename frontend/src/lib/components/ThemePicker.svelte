<script lang="ts">
import { useUiStore, type Theme } from '$lib/store/uiStore.svelte'
import Select from './Select.svelte'

let uiStore = useUiStore()

const options: { label: string; value: Theme }[] = [
  { label: 'Light', value: 'light' },
  { label: 'Dark', value: 'dark' },
  { label: 'Adapt to system', value: 'system' },
]
</script>

<div class="theme-picker">
  <Select bind:value={uiStore.theme} {options} aria-label="Select Theme" />
</div>

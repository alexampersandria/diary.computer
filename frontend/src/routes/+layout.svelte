<script lang="ts">
import 'modern-normalize/modern-normalize.css'

import '$lib/assets/main.scss'

import Preloader from '$lib/components/Preloader.svelte'
import { initializeStores } from '$lib/store/index.svelte'
import { useUiStore } from '$lib/store/uiStore.svelte'

initializeStores()

let uiStore = useUiStore()

let { children } = $props()

const title = $derived('diary.computer')
</script>

<svelte:head>
  <title>{title}</title>
</svelte:head>

<div id="root" class="theme-{uiStore.appliedTheme}">
  {#if uiStore.loading}
    <Preloader />
  {:else}
    {@render children()}
  {/if}
</div>

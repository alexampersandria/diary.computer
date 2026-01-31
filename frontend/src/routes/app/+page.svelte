<script lang="ts">
import EntryPreview from '$lib/components/EntryPreview.svelte'
import { useDataStore } from '$lib/store/dataStore.svelte'
import { useUiStore } from '$lib/store/uiStore.svelte'
import { useUserStore } from '$lib/store/userStore.svelte'
import { CalendarDays } from 'lucide-svelte'

let userStore = useUserStore()
let dataStore = useDataStore()
let uiStore = useUiStore()

let today = $derived.by(() => {
  return dataStore.getEntry(uiStore.date.formatted)
})
</script>

<div class="app-page home-page">
  <div class="container">
    {#if userStore.userDetails !== null}
      <div class="today">
        <div class="app-page-title">
          <CalendarDays />
          Today's Entry
        </div>
        <EntryPreview
          date={today ? today.date : uiStore.date.formatted}
          entry={today} />
      </div>
    {/if}
  </div>
</div>

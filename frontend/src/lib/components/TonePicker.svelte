<script lang="ts">
import { useUiStore, type Tone } from '$lib/store/uiStore.svelte'
import { Coffee, Snowflake } from 'lucide-svelte'

let uiStore = useUiStore()

const setTone = (tone: Tone) => {
  uiStore.tone = tone
}
</script>

<div class="tone-picker button-icon-group">
  <button
    class="plain"
    class:active={uiStore.tone === 'cold'}
    title="Cold"
    aria-label="Cold Tone"
    onclick={() => {
      setTone('cold')
    }}>
    <Snowflake />
  </button>
  <button
    class="plain"
    class:active={uiStore.tone === 'warm'}
    title="Warm"
    aria-label="Warm Tone"
    onclick={() => {
      setTone('warm')
    }}>
    <Coffee />
  </button>
</div>

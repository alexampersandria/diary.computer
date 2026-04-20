<script lang="ts">
import PastSevenDays from '$lib/assemblies/PastSevenDays.svelte'
import Today from '$lib/assemblies/Today.svelte'
</script>

<div class="app-page home-page">
  <div class="container">
    <div class="home-elements">
      <Today />
      <PastSevenDays />
    </div>
  </div>
</div>

<style lang="scss">
.home-elements {
  display: flex;
  flex-direction: column;
  gap: var(--padding-l);
}
</style>

<script lang="ts">
import PastSevenDays from '$lib/assemblies/PastSevenDays.svelte'
import Today from '$lib/assemblies/Today.svelte'
import { useUserStore } from '$lib/store/userStore.svelte'
import { currentDate, fullDate } from '$lib/utils/log'

const userStore = useUserStore()

const today = currentDate()
</script>

<div class="home-page">
  {#if userStore.userDetails}
    <div class="welcome-back">
      <div class="container fill">
        <h1>
          Welcome back
          {userStore.userDetails.name}
        </h1>
        <div class="today">
          Today is
          {fullDate(today)}
        </div>
      </div>
    </div>
  {/if}
  <div class="app-page">
    <div class="container">
      <div class="home-elements">
        <Today />
        <PastSevenDays />
      </div>
    </div>
  </div>
</div>

<style lang="scss">
.welcome-back {
  display: flex;
  flex-direction: column;
  justify-content: center;
  padding: var(--padding-xl) var(--padding-m);
  min-height: 25svh;
  background-color: var(--background-accent);
  border-bottom: var(--border-width) solid var(--border-color);

  h1 {
    margin: 0;
  }
}

.home-elements {
  display: flex;
  flex-direction: column;
  gap: var(--padding-l);
}
</style>

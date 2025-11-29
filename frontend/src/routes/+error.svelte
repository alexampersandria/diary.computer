<script lang="ts">
import { page } from '$app/state'
import NewIssue from '$lib/assemblies/NewIssue.svelte'
import Message from '$lib/components/Message.svelte'
import { useUserStore } from '$lib/store/userStore.svelte'
import { ArrowLeft } from 'lucide-svelte'

let userStore = useUserStore()
</script>

<div class="error">
  <div class="container">
    <Message type="error" size="small">
      Error {page.status}: {page.error?.message || 'Unknown error'}
    </Message>
    <br />
    <div class="muted small">
      If you believe this to be an error <NewIssue />
      <br />
      {#if userStore.userDetails}
        <a href="/app">
          <ArrowLeft />
          Dashboard
        </a>
      {:else}
        <a href="/">
          <ArrowLeft />
          Landing page
        </a>
      {/if}
    </div>
  </div>
</div>

<style lang="scss">
.error {
  height: 100vh;
  width: 100vw;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>

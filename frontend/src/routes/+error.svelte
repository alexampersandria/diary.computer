<script lang="ts">
import { page } from '$app/state'
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
      If you believe this to be an error <a
        href="https://github.com/alexampersandria/diary.computer/issues/new"
        target="_blank"
        rel="noopener noreferrer">create an issue</a>
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

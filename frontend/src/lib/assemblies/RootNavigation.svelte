<script lang="ts">
import Button from '$lib/components/Button.svelte'
import Logo from '$lib/components/Logo.svelte'
import Modal from '$lib/components/Modal.svelte'
import ThemeToggle from '$lib/components/ThemeToggle.svelte'
import {
  ArrowRight,
  Book,
  EllipsisVertical,
  Github,
  LogIn,
} from 'lucide-svelte'
import Auth from './Auth.svelte'
import { beforeNavigate } from '$app/navigation'
import { useUserStore } from '$lib/store/userStore.svelte'

let userStore = useUserStore()

let authModal = $state(false)
let authMode = $state<'login' | 'register'>('register')

const openAuthModal = (mode: 'login' | 'register' = 'login') => {
  authModal = true
  authMode = mode
}

let rightMenuOpen = $state(false)
const toggleRightMenu = () => {
  rightMenuOpen = !rightMenuOpen
}

beforeNavigate(nav => {
  if (nav.to?.route.id === '/login') {
    authMode = 'register'
    authModal = true
    nav.cancel()
  }
})
</script>

<Modal bind:open={authModal}>
  <Auth mode={authMode} />
</Modal>

<div class="navigation">
  <div class="action-elements">
    <a href="/"><Logo /></a>

    <div class="mobile-only toggle-right-menu">
      <ThemeToggle />
      <Button type="ghost" onclick={toggleRightMenu}>
        <EllipsisVertical />
      </Button>
    </div>
  </div>
  <div class="nav-elements-wrapper">
    <div class="nav-elements" class:open={rightMenuOpen}>
      <Button
        type="ghost"
        href="https://github.com/alexampersandria/diary.computer"
        target="_blank">
        <Github />
        GitHub
      </Button>

      <Button type="ghost" href="/docs">
        <Book />
        Docs
      </Button>

      {#if userStore.sessionId === null}
        <Button type="ghost" onclick={() => openAuthModal('login')}>
          <LogIn />
          Log in
        </Button>
      {:else}
        <Button type="ghost" href="/app">
          <ArrowRight />
          Go to app
        </Button>
      {/if}

      <div class="desktop-only">
        <ThemeToggle />
      </div>
    </div>
  </div>
</div>

<style lang="scss">
.navigation {
  position: sticky;
  z-index: 2;
  top: 0;
  width: 100%;
  height: 4rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-inline: var(--padding-xl);

  .action-elements,
  .nav-elements {
    display: flex;
    align-items: center;
    gap: var(--padding-s);
  }
}
</style>

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
import { onClickOutside, watch } from 'runed'

let userStore = useUserStore()

let authModal = $state(false)
let authMode = $state<'login' | 'register'>('register')

const openAuthModal = (mode: 'login' | 'register' = 'login') => {
  authModal = true
  authMode = mode
}

let navElements = $state<HTMLElement>()
let rightMenuOpen = $state(false)

const toggleRightMenu = () => {
  rightMenuOpen = !rightMenuOpen
}

const handleOutsideClick = onClickOutside(
  () => navElements,
  () => {
    if (rightMenuOpen) {
      rightMenuOpen = false
    }
  },
  { immediate: false },
)

watch(
  () => rightMenuOpen,
  () => {
    if (rightMenuOpen) {
      handleOutsideClick.start()
    } else {
      handleOutsideClick.stop()
    }
  },
)

beforeNavigate(nav => {
  rightMenuOpen = false
  if (nav.to?.route.id === '/login') {
    authMode = 'register'
    authModal = true
    nav.cancel()
  }
})
</script>

enabled {handleOutsideClick.enabled}

open: {rightMenuOpen}

<Modal bind:open={authModal}>
  <Auth mode={authMode} />
</Modal>

<div class="navigation">
  <div class="action-elements">
    <a class="logo-link" href="/"><Logo /></a>

    <div class="mobile-only toggle-right-menu">
      <ThemeToggle />
      <Button type="ghost" onclick={toggleRightMenu}>
        <EllipsisVertical />
      </Button>
    </div>
  </div>
  <div class="nav-elements-wrapper">
    <div
      class="nav-elements"
      class:open={rightMenuOpen}
      bind:this={navElements}>
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

  @media (max-width: 768px) {
    & {
      padding-inline: var(--padding-m);

      .action-elements {
        width: 100%;
        display: flex;
        justify-content: space-between;
        position: relative;
        z-index: 11;
      }

      .nav-elements-wrapper {
        overflow: hidden;
        position: absolute;
        display: flex;
        justify-content: flex-end;
        z-index: 10;
        top: 0;
        right: 0;
        padding: var(--padding-m);
        padding-top: 4rem;

        &:has(.open) {
          z-index: 12;
        }

        &:not(:has(.open)) {
          pointer-events: none;
        }

        .nav-elements {
          display: flex;
          flex-direction: column;
          align-items: flex-end;
          flex: 0 0 auto;
          padding: var(--padding-m);
          opacity: 0;
          transition: opacity var(--animation-length-s) var(--better-ease-out);
          background-color: var(--background-overlay-light);
          border-radius: var(--radius-s);
          box-shadow: var(--overlay-inset-shadow);
          backdrop-filter: blur(4px);
          --button-background-hover: transparent;
          --button-background-active: transparent;
          transform: translateY(calc(-0.5 * var(--padding-m)));
          transition:
            opacity var(--animation-length-s) var(--better-ease-out),
            transform var(--animation-length-s) var(--better-ease-out);

          :global(.button) {
            width: 100%;
            justify-content: flex-end;
          }

          &.open {
            opacity: 1;
            transform: translateY(0);
          }
        }
      }
    }
  }
}
</style>

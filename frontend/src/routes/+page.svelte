<script lang="ts">
import Auth from '$lib/assemblies/Auth.svelte'
import Button from '$lib/components/Button.svelte'
import Logo from '$lib/components/Logo.svelte'
import Modal from '$lib/components/Modal.svelte'
import ThemeToggle from '$lib/components/ThemeToggle.svelte'
import { useUserStore } from '$lib/store/userStore.svelte'
import { ArrowRight, Book, Github, LogIn, UserPlus } from 'lucide-svelte'

let userStore = useUserStore()

let authModal = $state(false)
let authMode = $state<'login' | 'register'>('register')
const openAuthModal = (mode: 'login' | 'register' = 'login') => {
  authModal = true
  authMode = mode
}
</script>

<svelte:head>
  <title>diary.computer — Your life, documented</title>
</svelte:head>

<div class="landing-page">
  <Modal bind:open={authModal}>
    <Auth mode={authMode} />
  </Modal>

  <div class="navigation">
    <div class="container">
      <div class="left muted">
        <a href="/"><Logo /></a>
      </div>
      <div class="right">
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

        <ThemeToggle />
      </div>
    </div>
  </div>

  <div class="container">
    <div class="header">
      <div class="text">
        <div class="fade-in fade-in-0 header-title instrument">
          Your life, documented
        </div>
        <div class="fade-in fade-in-1 muted small">
          Capture your life, day by day, and gain insights into how you live
        </div>
      </div>

      <div class="actions">
        <div class="fade-in fade-in-2 call-to-action">
          {#if userStore.sessionId === null}
            <Button type="primary" onclick={() => openAuthModal('register')}>
              <UserPlus />
              Sign up
            </Button>
          {:else}
            <Button type="primary" href="/app">
              <ArrowRight />
              Go to app
            </Button>
          {/if}
        </div>

        <div class="fade-in fade-in-3">
          <Button
            type="ghost"
            href="https://github.com/alexampersandria/diary.computer"
            target="_blank">
            <Github />
            GitHub
          </Button>
        </div>
      </div>
    </div>
  </div>
</div>

<!-- svelte-ignore css_unused_selector -->
<style lang="scss">
.landing-page {
  background: linear-gradient(
    135deg,
    rgba(255, 182, 232, 0.05) 0%,
    rgba(173, 216, 230, 0.07) 25%,
    rgba(226, 236, 249, 0.05) 50%,
    rgba(240, 170, 245, 0.07) 75%,
    rgba(176, 184, 230, 0.05) 100%
  );
  min-height: 100vh;

  .navigation {
    position: fixed;
    width: 100%;

    .container {
      display: flex;
      justify-content: space-between;
      align-items: center;
      min-height: 4rem;
    }

    .left,
    .right {
      display: flex;
      align-items: center;
      gap: var(--padding-s);
    }
  }

  .header {
    display: flex;
    flex-direction: column;
    justify-content: center;
    min-height: var(--block-size-s);
    height: 95dvh;
    gap: var(--padding-l);

    .text {
      .header-title {
        font-size: var(--font-size-xl);
        font-weight: 600;
      }
    }

    .actions {
      display: flex;
      gap: var(--padding-s);

      .call-to-action {
        position: relative;
        z-index: 1;

        &:after {
          position: absolute;
          top: 0;
          left: 0;
          z-index: -1;
          content: '';
          display: block;
          width: 100%;
          height: 100%;
          background: linear-gradient(
            45deg,
            var(--color-pink-60),
            var(--color-blue-60)
          );
          background-size: 200% 200%;
          background-position: 33% 50%;
          filter: blur(12px);
          transform: translateY(6px);
          transition:
            transform var(--animation-length-m) var(--better-ease-out),
            filter var(--animation-length-m) var(--better-ease-out),
            opacity var(--animation-length-xl) var(--better-ease-out),
            background-position var(--animation-length-xl)
              var(--better-ease-out);
          opacity: 0.5;
        }

        &:hover {
          &:after {
            filter: blur(18px);
            opacity: 0.75;
            background-position: 66% 50%;
          }
        }
      }
    }
  }

  .fade-in {
    animation: fade-in var(--animation-length-l) var(--better-ease-out);
    animation-fill-mode: backwards;

    @for $i from 0 through 99 {
      &.fade-in-#{$i} {
        animation-delay: calc(
          $i * var(--animation-length-s) + var(--animation-length-l)
        );
      }
    }
  }

  @keyframes fade-in {
    0% {
      opacity: 0;
      transform: translateY(0.2rem);
      filter: blur(4px);
    }
    100% {
      opacity: 1;
      transform: translateY(0);
      filter: blur(0);
    }
  }
}
</style>

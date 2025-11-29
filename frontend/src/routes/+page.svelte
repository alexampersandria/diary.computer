<script lang="ts">
import Auth from '$lib/assemblies/Auth.svelte'
import Button from '$lib/components/Button.svelte'
import Logo from '$lib/components/Logo.svelte'
import Modal from '$lib/components/Modal.svelte'
import ThemeToggle from '$lib/components/ThemeToggle.svelte'
import { useUserStore } from '$lib/store/userStore.svelte'
import {
  ArrowRight,
  Book,
  ChartArea,
  FileScan,
  Github,
  LogIn,
  PanelRightClose,
  PanelRightOpen,
  TrainFront,
  UserPlus,
} from 'lucide-svelte'

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
      <div class="action-elements muted">
        <a href="/"><Logo /></a>

        <div class="mobile-only toggle-right-menu">
          <Button type="ghost" onclick={toggleRightMenu}>
            {#if rightMenuOpen}
              <PanelRightClose />
            {:else}
              <PanelRightOpen />
            {/if}
          </Button>
        </div>
      </div>
      <div class="nav-elements-wrapper">
        <div class="nav-elements" class:open={rightMenuOpen}>
          <div class="mobile-only nav-title">diary.computer</div>

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
  </div>

  <div class="container landing-page-section">
    <div class="header">
      <div class="text">
        <div class="fade-in fade-in-0 header-title instrument">
          Your diary(.computer)
        </div>
        <div class="fade-in fade-in-1 muted small">
          Journaling with insights, simple, and open source
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

  <div class="container landing-page-section">
    <div class="more-info fade-in fade-in-4">
      <div class="info-section">
        <div class="info-section-title">
          <div class="icon">
            <FileScan />
          </div>
          Open Source
        </div>
        <div class="info-section-text">
          diary.computer is open source, you can self host your own instance or
          contribute to the project yourself on <a
            href="https://github.com/alexampersandria/diary.computer"
            target="_blank">
            GitHub
          </a>
        </div>
      </div>

      <div class="info-section fade-in fade-in-5">
        <div class="info-section-title">
          <div class="icon">
            <TrainFront />
          </div>
          Focus on performance
        </div>
        <div class="info-section-text">
          Entirely written in SvelteKit using bun and Rust for the backend,
          diary.computer is optimized for speed and efficiency to give you the
          best experience possible
        </div>
      </div>

      <div class="info-section fade-in fade-in-6">
        <div class="info-section-title">
          <div class="icon">
            <ChartArea />
          </div>
          Stats and insights
        </div>
        <div class="info-section-text">
          Designed to be simple and easy to use, and opinionated as to how you
          use your diary
        </div>
        <div class="info-section-text">
          All entries have a mood value with optional tags and text entries,
          making it easy to capture your day in just a few seconds and provides
          meaningful insights
        </div>
      </div>
    </div>
  </div>
</div>

<!-- svelte-ignore css_unused_selector -->
<style lang="scss">
@use '../lib/assets/scss/generics';

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
    height: 4rem;

    .container {
      display: flex;
      justify-content: space-between;
      align-items: center;
      min-height: 4rem;
    }

    .action-elements,
    .nav-elements {
      display: flex;
      align-items: center;
      gap: var(--padding-s);
    }
  }

  @media (max-width: 768px) {
    .navigation {
      position: relative;

      .action-elements {
        width: 100%;
        display: flex;
        justify-content: space-between;

        .toggle-right-menu {
          z-index: 11;
        }
      }

      .nav-elements-wrapper {
        overflow: hidden;
        position: absolute;
        z-index: 10;
        top: 0;
        right: 0;
        width: 100vw;
        height: 100vh;

        &:not(:has(.open)) {
          pointer-events: none;
        }

        .nav-elements {
          .nav-title {
            padding: var(--button-padding);
            font-weight: 600;
          }

          display: flex;
          flex-direction: column;
          align-items: flex-start;
          width: 100%;
          height: 100%;
          background-color: var(--background-overlay);
          padding: var(--padding-m);

          backdrop-filter: blur(0px);
          opacity: 0;
          transition:
            right var(--animation-length-s) var(--better-ease-out),
            backdrop-filter var(--animation-length-s) var(--better-ease-out),
            opacity var(--animation-length-s) var(--better-ease-out);

          &.open {
            right: 0;
            backdrop-filter: blur(4px);
            opacity: 1;
          }
        }
      }
    }
  }

  .landing-page-section {
    padding-block: var(--padding-l);
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  .header {
    display: flex;
    flex-direction: column;
    justify-content: center;
    min-height: var(--block-size-s);
    gap: var(--padding-l);
    padding-top: var(--padding-xl);

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

  .more-info {
    display: flex;
    flex-direction: column;
    gap: var(--padding-l);

    .info-section {
      display: flex;
      flex-direction: column;
      gap: var(--padding-s);

      padding-left: var(--padding-m);
      border-left: 4px solid var(--background-accent);
      transition: border-color var(--animation-length-s) var(--better-ease-out);

      .info-section-title {
        font-size: var(--font-size-l);
        font-weight: 600;
        display: flex;
        align-items: center;
        gap: var(--padding-s);
      }

      .info-section-text {
        @extend .muted;
        @extend .small;
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

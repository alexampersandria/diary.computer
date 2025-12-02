<script lang="ts">
import RootNavigation from '$lib/assemblies/RootNavigation.svelte'
import Button from '$lib/components/Button.svelte'
import Footer from '$lib/components/Footer.svelte'
import Logo from '$lib/components/Logo.svelte'
import { useUserStore } from '$lib/store/userStore.svelte'
import {
  ArrowRight,
  ChartArea,
  FileScan,
  Github,
  TrainFront,
  UserPlus,
} from 'lucide-svelte'
import { ScrollState } from 'runed'

let userStore = useUserStore()

let pageElement = $state<HTMLElement>()
let pageScrolState = new ScrollState({
  element: () => pageElement,
})
let scrolled = $derived.by(() => {
  return pageScrolState.y > 50
})
</script>

<svelte:head>
  <title>Your new diary(.computer)</title>
</svelte:head>

<div class="landing-page" bind:this={pageElement} class:scrolled>
  <RootNavigation />

  <div class="container landing-page-section">
    <div class="header centered">
      <div class="text">
        <div class="header-logo fade-in long-fade-in">
          <Logo />
        </div>
        <h1 class="fade-in fade-in-0 header-title">
          Your new diary(.computer)
        </h1>
        <div class="fade-in fade-in-1 header-subtitle muted">
          Your private space for daily reflection, capture your thoughts and
          gain insights that reveal patterns about yourself
        </div>
      </div>

      <div class="actions">
        <div class="fade-in fade-in-2 call-to-action">
          {#if userStore.sessionId === null}
            <Button type="primary" href="/login">
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

  <div class="landing-page-section image-preview fade-in fade-in-5">
    <img
      src="/img/landing-page-preview.webp"
      alt="screenshot of diary.computer app" />
  </div>

  <div class="container landing-page-section">
    <div class="more-info fade-in fade-in-6">
      <div class="info-section">
        <div class="info-section-title">
          <div class="icon">
            <FileScan />
          </div>
          Open Source
        </div>
        <div class="info-section-text muted">
          diary.computer is open source, you can self host your own instance or
          contribute to the project yourself on <a
            href="https://github.com/alexampersandria/diary.computer"
            target="_blank">
            GitHub
          </a>
        </div>
      </div>

      <div class="info-section fade-in fade-in-7">
        <div class="info-section-title">
          <div class="icon">
            <TrainFront />
          </div>
          Focus on performance
        </div>
        <div class="info-section-text muted">
          Entirely written in SvelteKit using bun and Rust for the backend,
          diary.computer is optimized for speed and efficiency to give you the
          best experience possible
        </div>
      </div>

      <div class="info-section fade-in fade-in-8">
        <div class="info-section-title">
          <div class="icon">
            <ChartArea />
          </div>
          Stats and insights
        </div>
        <div class="info-section-text muted">
          Designed to be simple and easy to use, and opinionated as to how you
          use your diary
        </div>
        <div class="info-section-text muted">
          All entries have a mood value with optional tags and text entries,
          making it easy to capture your day in just a few seconds and provides
          meaningful insights
        </div>
      </div>
    </div>
  </div>

  <Footer />
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
  background-attachment: fixed;
  background-size: cover;
  height: 100vh;
  overflow-x: hidden;
  overflow-y: auto;

  &:after {
    content: '';
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
    background: linear-gradient(
      to top,
      var(--background-primary) 5%,
      transparent 30%
    );
    z-index: 100;
    opacity: 1;
    transition: opacity var(--animation-length-l) var(--better-ease-out);
  }

  &.scrolled {
    &:after {
      opacity: 0;
    }
  }

  .landing-page-section {
    padding-block: var(--padding-l);
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  .image-preview {
    overflow: hidden;
    display: flex;
    justify-content: center;
    align-items: flex-start;

    img {
      min-width: 42rem;
      max-width: min(100%, 100rem);
    }
  }

  .header {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: clamp(var(--block-size-s), 50vh, 50vh);
    gap: var(--padding-l);
    padding-top: var(--padding-xl);

    .header-logo {
      font-size: 2rem;
      padding-bottom: var(--padding-l);
    }

    .text {
      .header-title {
        margin: 0;
      }

      .header-subtitle {
        max-width: var(--block-size-m);
        font-size: var(--font-size-xl);
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
    }
  }

  @media (max-width: 768px) {
    .navigation {
      position: relative;
      padding-inline: var(--padding-m);

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
          display: flex;
          flex-direction: column;
          align-items: flex-end;
          width: 100%;
          height: 100%;
          background-color: var(--background-overlay);
          padding: var(--padding-m);
          padding-top: 4rem;

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

    .header {
      padding-top: var(--padding-m);

      .text {
        .header-title {
          font-size: var(--font-size-xl);
        }

        .header-subtitle {
          font-size: var(--font-size-m);
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

    &.long-fade-in {
      animation: long-fade-in var(--animation-length-xxl)
        cubic-bezier(0, 0.13, 0, 0.98);
      animation-fill-mode: backwards;
      animation-delay: var(--animation-length-xl);
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

  @keyframes long-fade-in {
    0% {
      opacity: 0;
      transform: translateY(-6rem);
      filter: blur(8px);
    }
    100% {
      opacity: 1;
      transform: translateY(0);
      filter: blur(0);
    }
  }
}
</style>

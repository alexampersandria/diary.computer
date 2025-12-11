<script lang="ts">
import { page } from '$app/state'
import { active } from '$lib/actions/active.svelte'
import {
  getRoutes,
  componentNameFromRoute,
  titleFromRoute,
} from '$lib/utils/routes.svelte'
import Logo from '$lib/components/Logo.svelte'
import { Menu, X } from 'lucide-svelte'
import ThemeToggle from '$lib/components/ThemeToggle.svelte'
import { beforeNavigate } from '$app/navigation'
import { generateTableOfContents } from '$lib/utils/toc'
import type { TableOfContents as TableOfContentsType } from '$lib/utils/toc'
import { afterNavigate } from '$app/navigation'
import TableOfContents from '$lib/components/TableOfContents.svelte'
import Footer from '$lib/components/Footer.svelte'

const design = getRoutes(/\/docs\/design\/[^/]+\//)
const components = getRoutes(/\/docs\/components\/[^/]+\//)
const assemblies = getRoutes(/\/docs\/assemblies\/[^/]+\//)
const types = getRoutes(/\/docs\/types\/[^/]+\//)
const api = getRoutes(/\/docs\/api\/[^/]+\//)

let { children } = $props()

let sidebarOpen = $state(false)
let content: HTMLElement | undefined = $state()

beforeNavigate(nav => {
  // close sidebar and scroll to top when navigating to a new page
  sidebarOpen = false
  if (content) {
    content.scrollTop = 0
  }

  if (nav.to?.route.id?.startsWith('/app')) {
    // prevent navigation to /app routes from docs
    // this is being triggered from a doc example and should not direct the user to the app
    nav.cancel()
  }
})

const sidebarToggle = () => {
  sidebarOpen = !sidebarOpen
}

const SidebarIcon = $derived.by(() => {
  if (sidebarOpen) {
    return X
  } else {
    return Menu
  }
})

const title = $derived.by(() => {
  let pre = [...page.url.pathname.split('/').slice(2)]
    .filter(Boolean)
    .map(part =>
      part.replace(/-|_/g, ' ').replace(/\b\w/g, l => l.toUpperCase()),
    )
    .join('/')
  if (pre === '') {
    pre = 'Readme'
  }
  return pre + ' — diary.computer Documentation'
})

let toc: TableOfContentsType | undefined = $state()
afterNavigate(() => {
  toc = generateTableOfContents(content)
})
</script>

<svelte:head>
  <title>{title}</title>
</svelte:head>

<div class="docs">
  <div class="docs-menu">
    <div class="docs-sidebar-toggle">
      <button onclick={sidebarToggle}>
        <SidebarIcon />
      </button>
    </div>
    <div class="docs-home">
      <div class="docs-route-link">
        <a href="/">
          <Logo />
        </a>
      </div>
    </div>
    <div class="docs-icon-actions">
      <ThemeToggle />
    </div>
  </div>

  <div class="docs-navigation-backdrop"></div>
  <div class="docs-navigation" class:sidebarOpen>
    <div class="docs-routes">
      <div class="docs-route-title">Get Started</div>
      <div class="docs-route-link">
        <a href="/docs/" use:active>Readme</a>
        <a
          href="https://github.com/alexampersandria/diary.computer"
          target="_blank">GitHub</a>
      </div>
    </div>

    <div class="docs-routes">
      <div class="docs-route-title">API</div>
      {#each api as route}
        <div class="docs-route-link">
          <a href={route} use:active>
            {titleFromRoute(route)}
          </a>
        </div>
      {/each}
    </div>

    <div class="docs-routes">
      <div class="docs-route-title">Design</div>
      {#each design as route}
        <div class="docs-route-link">
          <a href={route} use:active>
            {titleFromRoute(route)}
          </a>
        </div>
      {/each}
    </div>

    <div class="docs-routes">
      <div class="docs-route-title">Components</div>
      {#each components as route}
        <div class="docs-route-link">
          <a href={route} use:active>
            {componentNameFromRoute(route)}
          </a>
        </div>
      {/each}
    </div>

    <div class="docs-routes">
      <div class="docs-route-title">Assemblies</div>
      {#each assemblies as route}
        <div class="docs-route-link">
          <a href={route} use:active>
            {componentNameFromRoute(route)}
          </a>
        </div>
      {/each}
    </div>
    <div class="docs-routes">
      <div class="docs-route-title">Types</div>
      {#each types as route}
        <div class="docs-route-link">
          <a href={route} use:active>
            {componentNameFromRoute(route)}
          </a>
        </div>
      {/each}
    </div>
  </div>

  <div class="docs-content" bind:this={content}>
    <div class="docs-content-flex">
      {#if toc}
        <div class="toc">
          <TableOfContents items={toc.items} />
        </div>
      {/if}

      <div class="container">
        {@render children()}
      </div>
    </div>

    <div class="footer">
      <Footer />
    </div>
  </div>
</div>

<style lang="scss">
.docs {
  display: grid;
  grid-template-columns: 16rem 1fr;
  grid-template-rows: 4rem 1fr;
  grid-column-gap: var(--border-width);
  grid-row-gap: var(--border-width);
  height: 100vh;
  overflow: hidden;

  .footer {
    display: flex;
    border-top: var(--border-width) solid var(--border-color);
    padding: var(--padding-l);

    :global(.footer) {
      padding: 0;
    }
  }

  background-color: var(--border-color);
  min-height: 100vh;

  .docs-menu {
    grid-area: 1 / 1 / 2 / 3;
  }

  .docs-navigation {
    grid-area: 2 / 1 / 3 / 2;
  }

  .docs-content {
    grid-area: 2 / 2 / 3 / 3;
  }

  .docs-menu {
    padding: var(--padding-m);
  }

  .docs-menu {
    display: flex;
    align-items: center;
    background-color: var(--background-primary);

    .docs-home {
      a {
        color: var(--text-primary);
      }
    }

    .docs-sidebar-toggle {
      button {
        background-color: transparent;
        border: none;
        font-size: 1.5rem;
        cursor: pointer;
        color: var(--text-muted);

        &:hover {
          color: var(--text-primary);
        }

        &:active {
          color: var(--text-muted);
        }
      }
    }

    .docs-icon-actions {
      display: flex;
      align-items: center;
      gap: var(--padding-s);
      margin-left: auto;
    }

    .docs-sidebar-toggle {
      display: none;
    }
  }

  .docs-content {
    padding-bottom: var(--padding-xl);

    .toc {
      display: none;
      max-width: var(--container-width);
      padding-inline: var(--padding-m);
      margin: 0 auto;
    }

    @media (min-width: 1500px) {
      .docs-content-flex:has(.toc) {
        display: flex;
        flex-direction: row-reverse;
        align-items: flex-start;
        justify-content: space-between;
        gap: var(--padding-l);
        padding: 0;

        .container {
          flex: 1;
          padding-block: var(--padding-l);
        }

        .toc {
          display: block;
          position: sticky;
          top: 0;
          max-height: calc(100vh - (4rem));
          overflow-y: auto;
          width: calc(100vw - var(--container-width) - 16rem);
          max-width: var(--block-size-xs);
          padding-block: var(--padding-l);
        }
      }
    }
  }

  .docs-navigation,
  .docs-content {
    background-color: var(--background-primary);
    height: 100%;
    overflow: auto;
    padding-bottom: max(var(--padding-xl), 5vh);
  }

  .docs-navigation {
    .docs-routes {
      display: flex;
      flex-direction: column;
      padding-top: var(--padding-m);

      .docs-route-link,
      .docs-route-title {
        flex: 1;
      }

      .docs-route-link a,
      .docs-route-title {
        font-size: var(--font-size-s);
        display: block;
        padding: calc(var(--padding-xs) / 2) var(--padding-m);
      }

      .docs-route-title {
        font-weight: 600;
      }
    }
  }

  a {
    color: var(--text-dimmed);

    &:hover {
      color: var(--text-muted);
    }

    &:global(.active) {
      color: var(--text-primary);
    }
  }
}

@media screen and (max-width: 768px) {
  .docs {
    grid-template-columns: 0 1fr;
    grid-column-gap: 0;

    .footer {
      padding-inline: var(--padding-s);
      margin-top: var(--padding-l);
    }

    .docs-navigation-backdrop {
      position: fixed;
      top: calc(4rem + var(--border-width));
      left: 0;
      width: 100%;
      height: 100vh;
      background-color: transparent;
      transition:
        background-color var(--animation-length-s) var(--better-ease-out),
        backdrop-filter var(--animation-length-l) var(--better-ease-out);
      pointer-events: none;
      z-index: 1311;

      &:has(+ .sidebarOpen) {
        background-color: var(--background-overlay);
        backdrop-filter: blur(4px);
      }
    }

    .docs-navigation {
      position: fixed;
      top: calc(4rem + var(--border-width));
      border-right: var(--border-width) solid var(--border-color);
      left: calc(-100% - var(--border-width));
      width: 100%;
      z-index: 1312;

      transition: left var(--animation-length-l);

      &.sidebarOpen {
        left: 0;
      }

      .docs-routes {
        .docs-route-link a,
        .docs-route-title {
          font-size: var(--font-size-m);
        }
      }
    }

    .docs-menu {
      justify-content: space-between;

      .docs-home {
        flex: 1;
      }

      .docs-sidebar-toggle {
        display: block;
      }
    }
  }
}
</style>

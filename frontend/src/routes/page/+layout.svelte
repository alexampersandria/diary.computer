<script lang="ts">
import { afterNavigate } from '$app/navigation'
import ArrowLink from '$lib/components/ArrowLink.svelte'
import Logo from '$lib/components/Logo.svelte'
import TableOfContents from '$lib/components/TableOfContents.svelte'
import ThemeToggle from '$lib/components/ThemeToggle.svelte'
import { generateTableOfContents } from '$lib/utils/toc'
import type { TableOfContents as TableOfContentsType } from '$lib/utils/toc'

let { children } = $props()

const rootTitle = 'diary.computer'
const title = $derived.by(() => {
  if (toc?.title) {
    return `${toc.title} - ${rootTitle}`
  } else {
    return rootTitle
  }
})

let root: HTMLElement | undefined = $state()

let toc: TableOfContentsType | undefined = $state()
afterNavigate(() => {
  toc = generateTableOfContents(root)
})
</script>

<svelte:head>
  <title>{title}</title>
</svelte:head>

<div class="root-page" bind:this={root}>
  <div class="container">
    <div class="nav">
      <ArrowLink href="/">Home</ArrowLink>

      <ThemeToggle />
    </div>

    <div class="content">
      {#if toc}
        <div class="toc">
          <TableOfContents items={toc.items} />
        </div>
      {/if}

      {@render children()}
    </div>
  </div>

  <div class="logo">
    <Logo />
  </div>
</div>

<style lang="scss">
.root-page {
  padding-block: var(--padding-l);
  min-height: 100vh;
  background-color: var(--background-primary);
  background: linear-gradient(
    to bottom,
    var(--background-accent) 0%,
    var(--background-secondary) 100vh
  );
  background-attachment: fixed;

  .nav {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .logo {
    display: flex;
    justify-content: center;
    align-items: center;
    height: min(20vh, 16rem);
  }

  @media (min-width: 1500px) {
    .toc {
      position: fixed;
      top: 0;
      right: 0;
      padding: var(--padding-l);
      overflow-y: auto;
      width: calc(50vw - var(--container-width) / 2);
      max-height: 100vh;
    }
  }
}
</style>

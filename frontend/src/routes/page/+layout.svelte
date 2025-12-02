<script lang="ts">
import { afterNavigate } from '$app/navigation'
import RootNavigation from '$lib/assemblies/RootNavigation.svelte'
import Footer from '$lib/components/Footer.svelte'
import TableOfContents from '$lib/components/TableOfContents.svelte'
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

let root = $state<HTMLElement>()

let toc: TableOfContentsType | undefined = $state()
afterNavigate(() => {
  toc = generateTableOfContents(root)
  if (root) {
    root.scrollTop = 0
  }
})
</script>

<svelte:head>
  <title>{title}</title>
</svelte:head>

<div class="root-page" bind:this={root}>
  <RootNavigation />

  <div class="container">
    <div class="content">
      {#if toc}
        <div class="toc">
          <div class="toc-inner">
            <TableOfContents items={toc.items} />
          </div>
        </div>
      {/if}

      <div class="page-content">
        {@render children()}
      </div>
    </div>
  </div>

  <Footer />
</div>

<style lang="scss">
.root-page {
  height: 100vh;
  overflow-y: auto;
  background-color: var(--background-primary);
  background: linear-gradient(
    to bottom,
    var(--background-accent) 0%,
    var(--background-secondary) 100vh
  );
  background-attachment: fixed;

  .content {
    padding-top: var(--padding-l);
  }

  .page-content :global(a) {
    color: var(--text-primary);
    text-decoration: underline;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;

    &:hover {
      color: var(--text-primary);
    }
  }

  .toc {
    display: none;
  }

  @media (min-width: 1500px) {
    .toc {
      display: block;
      position: fixed;
      top: 0;
      right: 0;
      padding: var(--padding-l);
      overflow-y: auto;
      width: calc(50vw - var(--container-width) / 2);
      max-height: 100vh;

      .toc-inner {
        padding-top: var(--padding-xl);
      }
    }
  }
}
</style>

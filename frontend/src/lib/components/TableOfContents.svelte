<script lang="ts">
import type { TableOfContentsProps } from '$lib/types/components/tableofcontents'
import TableOfContentsItem from './TableOfContentsItem.svelte'

let { items }: TableOfContentsProps = $props()
</script>

<div class="table-of-contents">
  <div class="title">On this page</div>

  {#each items as item}
    <TableOfContentsItem {item} />
  {/each}
</div>

<style>
.table-of-contents {
  display: flex;
  flex-direction: column;

  .title {
    font-weight: 600;
  }
}
</style>

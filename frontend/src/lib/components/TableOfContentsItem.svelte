<script lang="ts">
import type { TableOfContentsItemProps } from '$lib/types/components/tableofcontents'
import TableOfContentsItem from './TableOfContentsItem.svelte'

let { item }: TableOfContentsItemProps = $props()
</script>

<a class="table-of-contents-item level-{item.level}" href="#{item.id}">
  {item.title}
</a>

{#each item.children as child}
  <TableOfContentsItem item={child} />
{/each}

<style lang="scss">
.table-of-contents-item {
  color: var(--text-muted);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;

  &:hover {
    color: var(--text-primary);
  }

  @for $i from 1 through 6 {
    &.level-#{$i} {
      padding-left: #{$i - 1}ch;
    }
  }
}
</style>

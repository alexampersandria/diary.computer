<script lang="ts">
import Button from '$lib/components/Button.svelte'
import EntryPreview from '$lib/components/EntryPreview.svelte'
import Spinner from '$lib/components/Spinner.svelte'
import type { EntriesListProps } from '$lib/types/assemblies/entrieslist'
import { BookDashed } from 'lucide-svelte'

let { entries, pagination, loading, onloadmore }: EntriesListProps = $props()

let loadingMore = $state(false)
const clickMore = async () => {
  if (onloadmore) {
    loadingMore = true
    await onloadmore()
    loadingMore = false
  }
}
</script>

<div class="entries-list">
  {#if loading && !loadingMore}
    <div class="loading">
      <Spinner />
    </div>
  {:else if !entries || entries.length === 0}
    <div class="no-entries">
      <div class="missing-icon">
        <BookDashed />
      </div>
      <div class="text">No entries to show</div>
    </div>
  {:else}
    {#if pagination}
      <div class="count">
        showing {entries.length} of {pagination.total_count} entries
      </div>
    {/if}

    <div class="entries">
      {#each entries as entry}
        <div class="entry-item">
          <EntryPreview date={entry.date} {entry} small />
        </div>
      {/each}
    </div>

    {#if pagination && onloadmore}
      <div class="more">
        <Button
          fullwidth
          onclick={clickMore}
          loading={loadingMore}
          disabled={entries.length >= pagination.total_count}>
          Load more
        </Button>
      </div>
    {/if}
  {/if}
</div>

<style lang="scss">
.entries-list {
  display: flex;
  flex-direction: column;
  gap: var(--padding-m);

  .loading {
    padding: var(--padding-l) 0;
    display: flex;
    justify-content: center;
  }

  .no-entries {
    padding: var(--padding-l) 0;
    color: var(--text-muted);
    display: flex;
    flex-direction: column;
    align-items: center;

    .missing-icon {
      font-size: var(--font-size-xl);
    }
  }

  .count {
    font-size: var(--font-size-s);
    color: var(--text-muted);
    width: 100%;
    flex-basis: 100%;
    flex-grow: 1;
    flex-shrink: 0;
  }

  .entries {
    display: flex;
    flex-wrap: wrap;
    gap: var(--padding-s);

    .entry-item {
      flex: 1 1 100%;
      max-width: 100%;
    }
  }
}
</style>

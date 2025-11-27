<script lang="ts">
import { useDataStore } from '$lib/store/dataStore.svelte'
import type { EntryPreviewProps } from '$lib/types/components/entrypreview'
import { fullDate } from '$lib/utils/log'
import { Smile } from 'lucide-svelte'

let dataStore = useDataStore()

let { date, entry, small = false }: EntryPreviewProps = $props()

let tagList = $derived.by(() => {
  let tags =
    entry?.selected_tags.map(tagId => dataStore.getTag(tagId)?.name) || []
  tags.sort((a, b) => a!.localeCompare(b!))

  return tags.join(', ')
})

let mood = $derived.by(() => {
  return entry?.mood || null
})
</script>

{#key date}
  <a href={`/app/entry/${date}`} class="entry-preview" class:small>
    <div class="text">
      <div class="date">
        <div class="desktop-only">
          {fullDate(date)}
        </div>
        <div class="mobile-only">
          {date}
        </div>
      </div>
      {#if !entry}
        <div class="no-entry">No entry yet</div>
      {:else if tagList}
        <div class="tags">
          {tagList}
        </div>
      {/if}
    </div>

    <div class="mood mood-{mood || 'null'}">
      {#if mood}
        {mood}
      {:else}
        <Smile />
      {/if}
    </div>
  </a>
{/key}

<style lang="scss">
.entry-preview {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: var(--background-secondary);
  border: var(--border-width) solid var(--border-color);
  border-radius: var(--radius-xs);
  gap: var(--padding-m);
  color: var(--text-primary);
  height: var(--entry-preview-height);
  padding: var(--padding-s);
  flex: 1 0 auto;
  width: 100%;

  &.small {
    height: var(--entry-preview-height-small);
    padding: 0;

    .text {
      .no-entry,
      .tags {
        font-size: var(--font-size-xs);
      }
    }

    .mood {
      font-size: var(--font-size-s);
      width: calc(var(--entry-preview-height-small));
      border-top-left-radius: 0;
      border-bottom-left-radius: 0;
    }
  }

  &:active {
    transform: var(--click-transform);
    background-color: var(--background-primary);
  }

  &:hover {
    background-color: var(--background-accent);

    .text {
      .no-entry,
      .tags {
        color: var(--text-primary);
      }
    }

    .mood {
      &.mood-null {
        background-color: var(--background-secondary);
        color: var(--text-primary);
      }
    }
  }

  .text {
    display: flex;
    flex-direction: column;
    justify-content: left;
    text-align: left;
    flex-shrink: 1;
    overflow: hidden;

    padding-left: var(--padding-s);

    .no-entry,
    .tags {
      color: var(--text-muted);
      font-size: var(--font-size-s);
    }

    .date,
    .tags {
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
    }

    .date > div {
      overflow: hidden;
      text-overflow: ellipsis;
    }
  }

  .mood {
    font-size: var(--font-size-l);
    background-color: var(--background-primary);
    border-radius: var(--radius-s);
    flex-grow: 0;
    flex-shrink: 0;
    height: 100%;
    width: calc(var(--entry-preview-height) - var(--padding-s) * 2);
    display: flex;
    align-items: center;
    justify-content: center;

    @for $i from 1 through 5 {
      &.mood-#{$i} {
        background-color: var(--mood-value-#{$i}-background);
        color: var(--mood-value-#{$i}-color);
      }
    }

    &.mood-null {
      color: var(--text-muted);
    }
  }
}
</style>

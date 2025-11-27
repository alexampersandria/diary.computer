<script lang="ts">
import Entry from '$lib/assemblies/Entry.svelte'
import Message from '$lib/components/Message.svelte'
import Spinner from '$lib/components/Spinner.svelte'
import { useDataStore } from '$lib/store/dataStore.svelte'
import type { EditEntry, Entry as EntryType, NewEntry } from '$lib/types/log'
import { isValidDate } from '$lib/utils/log'
import { takeAtLeast } from '$lib/utils/takeAtLeast'
import { watch } from 'runed'
import type { PageProps } from './$types'

let { data }: PageProps = $props()

let dataStore = useDataStore()

let entry: EntryType | null | undefined = $state(
  dataStore.getEntry(data.date) || undefined,
)

const getData = () => {
  if (!valid) {
    entry = undefined
    return
  }
  setTimeout(async () => {
    // first fetch the entry from the backend to ensure we have the latest data
    // this is not done automatically in the dataStore to avoid excessive requests
    await takeAtLeast(dataStore.fetchEntry(data.date))
    entry = dataStore.getEntry(data.date) || null
  }, 0)
}

watch(
  () => data.date,
  () => {
    getData()
  },
)

const onCreate = async (newEntry: NewEntry) => {
  const created = await dataStore.createEntry({
    date: data.date,
    entry: newEntry.entry,
    mood: newEntry.mood,
    selected_tags: newEntry.selected_tags,
  })
  if (created) {
    entry = created
  }
  return created
}

const onUpdate = async (updatedEntry: EditEntry) => {
  const updated = await dataStore.updateEntry({
    id: updatedEntry.id,
    date: data.date,
    entry: updatedEntry.entry,
    mood: updatedEntry.mood,
    selected_tags: updatedEntry.selected_tags,
  })
  if (updated) {
    entry = updated
  } else {
    // if update fails, it could be because the entry was deleted elsewhere so refresh data
    getData()
  }
  return updated
}

const onDelete = async (entryId: string) => {
  const success = await dataStore.deleteEntry(entryId)
  if (success) {
    getData()
  }
  return success
}

let valid = $derived.by(() => {
  return isValidDate(data.date)
})
</script>

<div class="app-page entry-page">
  <div class="container">
    {#if valid}
      {#if entry !== undefined}
        {#key entry ? entry.id : data.date}
          <Entry
            mode={entry ? 'view' : 'create'}
            id={entry ? entry.id : undefined}
            date={data.date}
            entry={entry ? entry.entry : undefined}
            mood={entry ? entry.mood : undefined}
            selectedTagIds={entry ? entry.selected_tags : undefined}
            categories={dataStore.categories}
            {onCreate}
            {onUpdate}
            {onDelete}
            onAddTag={async tag => {
              return dataStore.createTag({
                name: tag.name,
                color: tag.color,
                category_id: tag.category_id,
              })
            }}
            onEditTag={async tag => {
              return dataStore.updateTag({
                id: tag.id,
                name: tag.name,
                color: tag.color,
              })
            }}
            onRemoveTag={async tagId => {
              return dataStore.deleteTag(tagId)
            }}
            onAddCategory={async category => {
              return dataStore.createCategory({
                name: category.name,
              })
            }}
            onEditCategory={async category => {
              return dataStore.updateCategory({
                id: category.id,
                name: category.name,
              })
            }}
            onDeleteCategory={async categoryId => {
              return dataStore.deleteCategory(categoryId)
            }} />
        {/key}
      {:else}
        <div class="loading">
          <Spinner />
        </div>
      {/if}
    {:else}
      <div class="container">
        <div class="invalid-date">
          <Message type="error">Error: Invalid date</Message>
          <div class="muted small">
            The date "{data.date}" is invalid
            <br />
            All dates must be in the format "YYYY-MM-DD"
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>

<style lang="scss">
.entry-page {
  display: flex;
  justify-content: center;

  .invalid-date {
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: var(--padding-s);
  }

  .loading {
    display: flex;
    justify-content: center;
    align-items: center;
    padding: var(--padding-l);
  }
}
</style>

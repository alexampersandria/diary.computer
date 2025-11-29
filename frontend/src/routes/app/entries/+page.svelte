<script lang="ts">
import { getEntries, type FetchEntriesOptions } from '$lib/utils/api'
import type { Entry } from '$lib/types/log'
import { FilterX, ScrollText, SlidersHorizontal } from 'lucide-svelte'
import { onMount } from 'svelte'
import { useUserStore } from '$lib/store/userStore.svelte'
import { useDataStore } from '$lib/store/dataStore.svelte'
import Button from '$lib/components/Button.svelte'
import Input from '$lib/components/Input.svelte'
import Select from '$lib/components/Select.svelte'
import { goto } from '$app/navigation'
import type { PaginationObject } from '$lib/types/paginated'
import Checkbox from '$lib/components/Checkbox.svelte'
import Label from '$lib/components/Label.svelte'
import { takeAtLeast } from '$lib/utils/takeAtLeast'
import Calendar from '$lib/components/Calendar.svelte'
import Modal from '$lib/components/Modal.svelte'
import EntriesList from '$lib/assemblies/EntriesList.svelte'

let {
  data,
}: {
  data: {
    search?: string
  }
} = $props()

let search = $derived.by(() => {
  const urlSearchParams = new URLSearchParams(data?.search || '')

  return {
    from_date: urlSearchParams.get('from_date') || undefined,
    to_date: urlSearchParams.get('to_date') || undefined,
    tags: urlSearchParams.get('tags')?.split(',') || undefined,
    from_mood: urlSearchParams.get('from_mood') || undefined,
    to_mood: urlSearchParams.get('to_mood') || undefined,
    order: urlSearchParams.get('order') || undefined,
  } as FetchEntriesOptions
})

let list: Entry[] = $state([])
let pagination: PaginationObject = $state({
  limit: 0,
  offset: 0,
  total_count: 0,
})
let loading = $state(false)
let showFilters = $state(false)

let userStore = useUserStore()
let dataStore = useDataStore()

onMount(() => {
  options = { ...options, ...search }
  getData()
})

let options: FetchEntriesOptions = $state({
  from_date: undefined,
  to_date: undefined,
  tags: undefined,
  from_mood: undefined,
  to_mood: undefined,
  order: 'date_desc',
  limit: 20,
  offset: 0,
})

const reset = () => {
  options = {
    from_date: undefined,
    to_date: undefined,
    tags: undefined,
    from_mood: undefined,
    to_mood: undefined,
    order: 'date_desc',
    limit: 20,
    offset: 0,
  }
}

const getData = async (more = false) => {
  if (userStore.sessionId) {
    if (more) {
      options.offset = pagination.offset + pagination.limit
    } else {
      options.offset = 0
    }

    loading = true

    const params = new URLSearchParams()
    if (options.from_date) params.append('from_date', options.from_date)
    if (options.to_date) params.append('to_date', options.to_date)
    if (options.tags) {
      const tagString = options.tags.join(',')
      if (tagString) {
        params.append('tags', tagString)
      }
    }
    if (options.from_mood) params.append('from_mood', `${options.from_mood}`)
    if (options.to_mood) params.append('to_mood', `${options.to_mood}`)
    if (options.order && options.order !== 'date_desc')
      params.append('order', options.order)
    if (options.limit !== undefined && options.limit !== 20)
      params.append('limit', `${options.limit}`)

    goto(`/app/entries/?${params.toString()}`, { replaceState: true })

    const res = await takeAtLeast(getEntries(userStore.sessionId, options))
    if (res) {
      if (more) {
        list = [...list, ...res.data]
      } else {
        list = res.data
      }
      pagination = res.pagination
    } else {
      list = []
      console.error('Failed to fetch entries')
    }

    loading = false
  }
}

const toggleTag = (tagId: string) => {
  if (!options.tags) {
    options.tags = []
  }
  const index = options.tags.indexOf(tagId)
  if (index === -1) {
    options.tags.push(tagId)
  } else {
    options.tags.splice(index, 1)
  }
}

const filtersApplied = $derived.by(() => {
  return (
    options.from_date !== undefined ||
    options.to_date !== undefined ||
    (options.tags !== undefined && options.tags.length > 0) ||
    options.from_mood !== undefined ||
    options.to_mood !== undefined ||
    (options.order !== undefined && options.order !== 'date_desc') ||
    (options.limit !== undefined && options.limit !== 20)
  )
})
</script>

<div class="app-page entries-page">
  <div class="container">
    <div class="app-page-title">
      <ScrollText />
      Entries

      <div class="toggle-filters">
        <Button onclick={() => (showFilters = true)}>
          <SlidersHorizontal />
        </Button>
      </div>
    </div>

    <EntriesList
      entries={list}
      {pagination}
      {loading}
      onloadmore={() => getData(true)} />
  </div>
</div>

<Modal bind:open={showFilters} size="m" onclose={() => getData()}>
  <div class="filters">
    <div class="filters-title">
      Filter entries

      <div class="clear-filters">
        <Button onclick={reset} disabled={!filtersApplied}>
          <FilterX />
        </Button>
      </div>
    </div>

    <div class="rangepicker">
      <Label>Date range</Label>
      <Calendar
        mode="rangepicker"
        bind:from={options.from_date}
        bind:to={options.to_date} />
    </div>

    <div>
      <Label>Tags</Label>
      <div class="tag-options">
        {#each dataStore.getTags() || [] as tag}
          <div class="tag-option color-{tag.color}-text">
            <Checkbox
              id={tag.id}
              value={options.tags?.includes(tag.id) || false}
              onchange={() => toggleTag(tag.id)} />
            <Label for={tag.id}>
              {tag.category.name}/{tag.name}
            </Label>
          </div>
        {/each}
      </div>
    </div>

    <div class="form-field inline">
      <Label for="from-mood">Mood</Label>
      <div class="inputs">
        <Input id="from-mood" type="number" bind:value={options.from_mood} />
        to
        <Input id="to-mood" type="number" bind:value={options.to_mood} />
      </div>
    </div>

    <div class="form-field inline">
      <Label for="limit">Results per page</Label>
      <Select
        bind:value={options.limit}
        options={[
          { label: '10', value: 10 },
          { label: '20', value: 20 },
          { label: '50', value: 50 },
          { label: '100', value: 100 },
          { label: 'all', value: 0 },
        ]} />
    </div>

    <div class="form-field inline">
      <Label for="order">Sort</Label>
      <Select
        bind:value={options.order}
        options={[
          { label: 'Date Descending', value: 'date_desc' },
          { label: 'Date Ascending', value: 'date_asc' },
          { label: 'Mood Descending', value: 'mood_desc' },
          { label: 'Mood Ascending', value: 'mood_asc' },
        ]} />
    </div>
  </div>
</Modal>

<style lang="scss">
.entries-page {
  .toggle-filters {
    margin-left: auto;
  }
}

.filters {
  padding-top: var(--padding-m);
  display: flex;
  flex-wrap: wrap;
  gap: var(--padding-s);
  margin-bottom: var(--padding-m);

  .filters-title {
    font-weight: 600;
    font-size: var(--font-size-xl);
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
  }

  .rangepicker {
    flex: 1 1 100%;
  }

  .tag-options {
    display: flex;
    flex-wrap: wrap;
    gap: var(--padding-s);
  }
}
</style>

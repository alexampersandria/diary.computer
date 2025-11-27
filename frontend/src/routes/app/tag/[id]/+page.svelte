<script lang="ts">
import { useDataStore } from '$lib/store/dataStore.svelte'
import { Tag, Tags } from 'lucide-svelte'
import type { PageProps } from './$types'
import Message from '$lib/components/Message.svelte'
import type { Paginated } from '$lib/types/paginated'
import type { Entry } from '$lib/types/log'
import { useUserStore } from '$lib/store/userStore.svelte'
import {
  DEFAULT_TAKEATLEAST_DURATION,
  takeAtLeast,
} from '$lib/utils/takeAtLeast'
import { getEntries } from '$lib/utils/api'
import EntriesList from '$lib/assemblies/EntriesList.svelte'
import { onMount } from 'svelte'

let { data: pageData }: PageProps = $props()

let dataStore = useDataStore()
let userStore = useUserStore()

let tag = $derived.by(() => {
  return dataStore.getTag(pageData.id)
})

let data: Paginated<Entry> | undefined = $state()
let loading = $state(false)
let limit = 20

const getData = async () => {
  if (userStore.sessionId && tag) {
    if (!data) {
      loading = true
      const res = await takeAtLeast(
        getEntries(userStore.sessionId, { limit, tags: [tag.id] }),
      )
      if (res) {
        data = res
      }
      loading = false
    }
  }
}

const onloadmore = async () => {
  if (userStore.sessionId && tag) {
    if (data) {
      const offset = data.pagination.offset + data.pagination.limit
      const res = await takeAtLeast(
        getEntries(userStore.sessionId, { limit, tags: [tag.id], offset }),
        DEFAULT_TAKEATLEAST_DURATION / 2,
      )
      if (res) {
        data.pagination = res.pagination
        data.data = [...data.data, ...res.data]
      }
    } else {
      await getData()
    }
  }
}

onMount(() => {
  getData()
})
</script>

<div class="app-page tag-page">
  <div class="container">
    {#if tag}
      <div class="app-page-title">
        <Tag />
        {tag.name}
      </div>

      <EntriesList
        entries={data?.data}
        pagination={data?.pagination}
        {loading}
        {onloadmore} />
    {:else}
      <div class="no-tag">
        <Message type="error">Error: Tag not found</Message>
        <div class="muted small">
          If you believe this to be an error <a
            href="http://github.com/alexampersandria/ephemeride/issues/new"
            target="_blank"
            rel="noopener noreferrer">create an issue</a>
          <br />
          <a href="/app/tags"><Tags /> See list of all tags</a>
        </div>
      </div>
    {/if}
  </div>
</div>

<style lang="scss">
.tag-page {
  .no-tag {
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: var(--padding-s);
  }
}
</style>

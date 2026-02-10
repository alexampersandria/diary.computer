<script lang="ts">
import { useDataStore } from '$lib/store/dataStore.svelte'
import { ChevronLeft, ChevronRight, Tag, Tags } from 'lucide-svelte'
import type { PageProps } from './$types'
import Message from '$lib/components/Message.svelte'
import type { Paginated } from '$lib/types/paginated'
import type { Entry } from '$lib/types/log'
import { useUserStore } from '$lib/store/userStore.svelte'
import { takeAtLeast } from '$lib/utils/takeAtLeast'
import { getEntries } from '$lib/utils/api'
import EntriesList from '$lib/assemblies/EntriesList.svelte'
import { onMount } from 'svelte'
import NewIssue from '$lib/components/NewIssue.svelte'
import { currentDateObject, yearDateRange } from '$lib/utils/log'
import type { HeatmapDataPoint } from '$lib/types/components/heatmap'
import Heatmap from '$lib/components/Heatmap.svelte'
import Button from '$lib/components/Button.svelte'

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

let yearlyDataYear = $state(currentDateObject().year)
let yearlyData: HeatmapDataPoint[] | undefined = $state()

const getHeatmapData = async (
  year = yearlyDataYear,
  minDuration: number | undefined = undefined,
) => {
  if (userStore.sessionId && tag) {
    const paginatedEntries = await takeAtLeast(
      getEntries(userStore.sessionId, {
        from_date: yearDateRange(year).firstDate,
        to_date: yearDateRange(year).lastDate,
        limit: 366,
        tags: [tag.id],
      }),
      minDuration !== undefined ? minDuration : 750, // longer because the animation is pretty :)
    )

    if (paginatedEntries) {
      const entries = paginatedEntries.data
      yearlyData = entries.map(entry => {
        return {
          date: entry.date,
          value: entry.mood,
        }
      })
    }
  }
}

const navigateYear = async (year: number) => {
  yearlyDataYear = year
  getHeatmapData(year, 0)
}

onMount(() => {
  getData()
  getHeatmapData(yearlyDataYear)
})
</script>

<div class="app-page tag-page">
  <div class="container">
    {#if tag}
      <div class="app-page-title">
        <div class="icon color-{tag.color}-text">
          <Tag />
        </div>
        <div class="ellipsis">
          {tag.category.name} /
          {tag.name}
        </div>
      </div>

      <div class="sections">
        <div class="section heatmap">
          <div class="section-title">
            {yearlyDataYear} Heatmap
            <div class="navigation">
              <Button
                type="ghost"
                onclick={() => navigateYear(yearlyDataYear - 1)}
                aria-label="Previous Year">
                <ChevronLeft />
              </Button>

              <Button
                type="ghost"
                onclick={() => navigateYear(yearlyDataYear + 1)}
                aria-label="Next Year">
                <ChevronRight />
              </Button>
            </div>
          </div>

          <Heatmap
            year={yearlyDataYear}
            data={yearlyData}
            loading={yearlyData === undefined} />
        </div>

        <div class="section entries">
          <div class="section-title">Entries</div>

          <EntriesList
            entries={data?.data}
            pagination={data?.pagination}
            {loading}
            {onloadmore} />
        </div>
      </div>
    {:else}
      <div class="no-tag">
        <Message type="error">Error: Tag not found</Message>
        <div class="muted small">
          If you believe this to be an error <NewIssue />
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

  .icon {
    display: flex;
  }

  .sections {
    display: flex;
    flex-direction: column;
    gap: var(--padding-l);
    padding-top: var(--padding-m);

    .section {
      display: flex;
      flex-direction: column;
      gap: var(--padding-s);

      .section-title {
        display: flex;
        justify-content: space-between;
        align-items: center;

        .navigation {
          display: flex;
          gap: var(--padding-xs);
        }
      }
    }
  }
}
</style>

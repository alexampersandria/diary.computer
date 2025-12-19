<script lang="ts">
import Button from '$lib/components/Button.svelte'
import Heatmap from '$lib/components/Heatmap.svelte'
import Table from '$lib/components/Table.svelte'
import { useDataStore } from '$lib/store/dataStore.svelte'
import { useUserStore } from '$lib/store/userStore.svelte'
import type {
  MoodStats,
  TagMoodStats,
  TagStats,
  WeekdayStats,
} from '$lib/types/api/stats'
import type { HeatmapDataPoint } from '$lib/types/components/heatmap'
import {
  getEntries,
  getMoodStats,
  getTagStats,
  getWeekdayStats,
} from '$lib/utils/api'
import { formatKey } from '$lib/utils/formatKey'
import { currentDateObject, yearDateRange } from '$lib/utils/log'
import { formatNumber } from '$lib/utils/numbers'
import { takeAtLeast } from '$lib/utils/takeAtLeast'
import { ChartLine, ChevronLeft, ChevronRight } from 'lucide-svelte'
import { onMount } from 'svelte'

let userStore = useUserStore()
let dataStore = useDataStore()

let yearlyDataYear = $state(currentDateObject().year)
let yearlyData: HeatmapDataPoint[] | undefined = $state(undefined)

const getHeatmapData = async (
  year = yearlyDataYear,
  minDuration: number | undefined = undefined,
) => {
  if (userStore.sessionId) {
    const paginatedEntries = await takeAtLeast(
      getEntries(userStore.sessionId, {
        from_date: yearDateRange(year).firstDate,
        to_date: yearDateRange(year).lastDate,
        limit: 366,
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

let moodData: MoodStats | undefined = $state(undefined)
let tagData: TagStats | undefined = $state(undefined)
let weekdayData: WeekdayStats | undefined = $state(undefined)

const getMoodData = async () => {
  if (userStore.sessionId) {
    const moodStats = await getMoodStats(userStore.sessionId)

    if (moodStats) {
      moodData = moodStats
    }
  }
}

const getTagData = async () => {
  if (userStore.sessionId) {
    const tagStats = await takeAtLeast(getTagStats(userStore.sessionId))

    if (tagStats) {
      tagData = tagStats
    }
  }
}

const getWeekdayData = async () => {
  if (userStore.sessionId) {
    const weekdayStats = await takeAtLeast(getWeekdayStats(userStore.sessionId))

    if (weekdayStats) {
      weekdayData = weekdayStats
    }
  }
}

onMount(() => {
  getHeatmapData()
  getMoodData()
  getTagData()
  getWeekdayData()
})

const navigateYear = async (year: number) => {
  yearlyDataYear = year
  getHeatmapData(year, 0)
}

const formatTag = (tagStat: TagMoodStats) => {
  const tag = dataStore.getTag(tagStat.tag_id)
  if (!tag) {
    return 'Unknown'
  }
  return `${tag.category.name}/${tag.name}`
}
</script>

<div class="app-page stats-page">
  <div class="container">
    <div class="app-page-title">
      <ChartLine />
      Stats
    </div>

    <div class="stats-sections">
      <div class="section overall">
        <table class="values">
          <tbody>
            <tr class="entry-count">
              <td class="label">Total Entries</td>
              <td class="value">
                {#if moodData}
                  {formatNumber(moodData.entry_count)}
                {:else}
                  loading...
                {/if}
              </td>
            </tr>
            <tr class="average-mood">
              <td class="label">Average Mood</td>
              <td class="value">
                {#if moodData}
                  {formatNumber(moodData.average_mood)}
                {:else}
                  loading...
                {/if}
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="section heatmap-current-year">
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

      <div class="section tag-stats">
        <div class="section-title">Tag Stats</div>

        <Table
          fields={[
            { key: 'tag', label: 'Tag', sortable: true },
            { key: 'entry_count', label: 'Entry Count', sortable: true },
            { key: 'average_mood', label: 'Average Mood', sortable: true },
          ]}
          data={tagData?.map(tag => ({
            tag: formatTag(tag),
            entry_count: tag.entry_count,
            average_mood: tag.average_mood,
            median_mood: tag.median_mood,
          }))}
          compareAverageTo={moodData?.average_mood}
          sortedBy="entry_count" />
      </div>

      <div class="section weekday-stats">
        <div class="section-title">Weekday Stats</div>

        <Table
          data={weekdayData
            ? Object.entries(weekdayData).map(([key, value]) => ({
                weekday: formatKey(key),
                entry_count: value.entry_count,
                average_mood: value.average_mood,
              }))
            : undefined}
          compareAverageTo={moodData?.average_mood} />
      </div>
    </div>
  </div>
</div>

<style lang="scss">
.stats-page {
  .stats-sections {
    display: flex;
    flex-direction: column;
    gap: var(--padding-l);
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

  .overall {
    .values {
      width: max-content;

      .label {
        font-weight: 600;
        padding-right: var(--padding-m);
      }

      .value {
        text-align: right;
        min-width: 5ch;
      }
    }
  }
}
</style>

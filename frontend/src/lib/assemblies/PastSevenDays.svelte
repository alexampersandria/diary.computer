<script lang="ts">
import { useUserStore } from '$lib/store/userStore.svelte'
import type { Entry } from '$lib/types/log'
import { getEntries } from '$lib/utils/api'
import { currentDate, timestampToDate } from '$lib/utils/log'
import { CalendarDays, Smile } from 'lucide-svelte'
import { onMount } from 'svelte'

let userStore = useUserStore()

let pastSevenDays: Array<Entry> | undefined = $state()

const getPastSevenDaysData = async () => {
  if (userStore.sessionId) {
    const today = currentDate()
    const sevenDaysAgo = new Date(today)
    sevenDaysAgo.setDate(sevenDaysAgo.getDate() - 7)
    const data = await getEntries(userStore.sessionId, {
      from_date: timestampToDate(sevenDaysAgo.getTime()),
      to_date: today,
    })

    if (data) {
      pastSevenDays = data.data
    }
  }
}

onMount(() => {
  getPastSevenDaysData()
})

let pastSevenDates = $derived.by(() => {
  const today = currentDate()
  const array = [today]
  for (let i = 1; i < 7; i++) {
    const date = new Date(today)
    date.setDate(date.getDate() - i)
    array.push(timestampToDate(date.getTime()))
  }
  return array
})
</script>

{#if userStore.userDetails !== null}
  <div class="past-seven-days">
    <div class="app-page-title">
      <CalendarDays />
      Past 7 days
    </div>

    <div class="dates">
      {#each pastSevenDates as date}
        {@const entry = pastSevenDays?.find(e => e.date === date)}
        {@const hasEntry = entry !== undefined}
        {@const weekDay = new Date(date).toLocaleString('en-US', {
          weekday: 'short',
        })}
        <a href={`/app/entry/${date}`} class="date" class:has-entry={hasEntry}>
          <div class="weekday">
            {weekDay}
          </div>
          <div class="mood mood-{entry?.mood || 'null'}">
            {#if hasEntry}
              {entry.mood}
            {:else}
              <Smile />
            {/if}
          </div>
        </a>
      {/each}
    </div>
  </div>
{/if}

<style lang="scss">
.past-seven-days {
  container-type: inline-size;
  container-name: past-seven-days;

  .dates {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    background-color: var(--background-secondary);
    border: var(--border-width) solid var(--border-color);
    border-radius: var(--radius-s);
    padding: var(--padding-s);
    gap: var(--padding-xs);

    @container past-seven-days (max-width: 600px) {
      grid-template-columns: 1fr;
    }

    .date {
      flex: 1 0 auto;
      display: flex;
      flex-direction: column;
      justify-content: stretch;
      align-items: stretch;
      gap: var(--padding-xs);
      color: var(--text-primary);
      border-radius: var(--radius-s);
      overflow: hidden;

      &:hover {
        .weekday {
          color: var(--text-primary);
        }

        .mood {
          &.mood-null {
            color: var(--text-primary);
          }
        }
      }

      &.has-entry {
        .weekday {
          color: var(--text-primary);
        }
      }

      .weekday {
        flex: 1 0 auto;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        text-align: center;
        color: var(--text-muted);
      }

      .mood {
        flex: 1 0 auto;
        font-size: var(--font-size-l);
        border-radius: var(--radius-s);
        background-color: var(--background-primary);
        display: flex;
        align-items: center;
        justify-content: center;
        height: 4rem;

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
  }
}
</style>

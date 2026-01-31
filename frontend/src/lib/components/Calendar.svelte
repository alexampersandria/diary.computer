<script lang="ts">
import type {
  CalendarProps,
  RangepickerValue,
} from '$lib/types/components/calendar'
import {
  calendarDaysInMonth,
  dateClosestToRangeEdge,
  isDateInRange,
  monthDateRange,
  monthValuesShort,
  sortDateRange,
  yearValues,
} from '$lib/utils/log'
import { ChevronLeft, ChevronRight } from 'lucide-svelte'
import { useDataStore } from '$lib/store/dataStore.svelte'
import { page } from '$app/state'
import { onMount } from 'svelte'
import type { Range } from '$lib/types/range'
import { watch } from 'runed'
import Select from './Select.svelte'
import { useUiStore } from '$lib/store/uiStore.svelte'

let dataStore = useDataStore()
let uiStore = useUiStore()

let { month: defaultMonth, year: defaultYear } = uiStore.date.object
let {
  mode = 'navigation',
  value = $bindable(undefined),
  from = $bindable(undefined),
  to = $bindable(undefined),
  month = defaultMonth,
  year = defaultYear,
  usedata = false,
  onchange,
}: CalendarProps = $props()

let calendarYearValues = $state(yearValues)

let daysInMonth = $derived.by(() => {
  return calendarDaysInMonth(year, month)
})

const getData = () => {
  if (usedata) {
    const { firstDate, lastDate } = monthDateRange(year, month)
    dataStore.fetchEntries({
      from_date: firstDate,
      to_date: lastDate,
    })
  }
}

onMount(() => {
  if (usedata && mode === 'navigation') {
    month = dataStore.calendarPosition.month
    year = dataStore.calendarPosition.year
  }

  getData()
})

const navigate = (increment: number) => {
  if (increment !== 0) {
    month += increment
    if (month < 1) {
      month = 12
      year -= 1
    } else if (month > 12) {
      month = 1
      year += 1
    }
  } else {
    const { month: newMonth, year: newYear } = uiStore.date.object
    month = newMonth
    year = newYear
  }
  // getData will be called by the watcher
}

const formatDay = (day: number): string => {
  const paddedMonth = String(month).padStart(2, '0')
  const paddedDay = String(day).padStart(2, '0')
  return `${year}-${paddedMonth}-${paddedDay}`
}

const entryLink = (day: number): string => {
  return `/app/entry/${formatDay(day)}/`
}

const dayMood = (day: number | null) => {
  if (usedata) {
    return dataStore.getEntry(formatDay(day || 0))?.mood || null
  }
}

const isToday = (day: number | null) => {
  const today = uiStore.date.formatted
  return today === formatDay(day || 0)
}

const isFuture = (day: number | null) => {
  const today = uiStore.date.formatted
  return new Date(formatDay(day || 0)) > new Date(today)
}

const isActive = (day: number | null) => {
  if (day === null) {
    return false
  }
  const url = page.url.pathname
  return url === entryLink(day)
}

const isSelected = (day: number | null) => {
  if (day === null || !value || mode === 'navigation') {
    return false
  }

  const formattedDay = formatDay(day)
  if (value === formattedDay || from === formattedDay || to === formattedDay) {
    return true
  } else {
    return false
  }
}

const isFirstSelected = (day: number | null) => {
  if (
    day === null ||
    !value ||
    mode !== 'rangepicker' ||
    typeof value === 'string'
  ) {
    return false
  }
  if (!isSelected(day)) {
    return false
  }
  if (!value.from || !value.to) {
    return false
  }
  const formattedDay = formatDay(day)
  return value.from === formattedDay
}

const isLastSelected = (day: number | null) => {
  if (
    day === null ||
    !value ||
    mode !== 'rangepicker' ||
    typeof value === 'string'
  ) {
    return false
  }
  if (!isSelected(day)) {
    return false
  }
  if (!value.from || !value.to) {
    return false
  }
  const formattedDay = formatDay(day)
  return value.to === formattedDay
}

const isBetweenSelected = (day: number | null) => {
  if (
    day === null ||
    !value ||
    mode !== 'rangepicker' ||
    typeof value === 'string'
  ) {
    return false
  }
  if (isSelected(day)) {
    return false
  }
  const formattedDay = formatDay(day)
  if (value.from && value.to) {
    return isDateInRange(formattedDay, value as Range<string>)
  }
  return false
}

const clickDay = (day: number) => {
  const formattedDay = formatDay(day)
  if (mode === 'datepicker') {
    if (value === formattedDay) {
      value = undefined
    } else {
      value = formattedDay
    }
    from = undefined
    to = undefined
  } else if (mode === 'rangepicker') {
    if (value === undefined) {
      value = {
        from: formattedDay,
        to: undefined,
      }
    } else if (typeof value === 'object') {
      if (value.from === formattedDay) {
        value.from = undefined
      } else if (value.to === formattedDay) {
        value.to = undefined
      } else if (value.from === undefined) {
        value.from = formattedDay
      } else if (value.to === undefined) {
        value.to = formattedDay
      } else if (value.from && value.to) {
        const edge = dateClosestToRangeEdge(
          formattedDay,
          value as Range<string>,
        )
        if (edge === 'from') {
          value.from = formattedDay
        } else {
          value.to = formattedDay
        }
      }

      if (value.from && value.to) {
        value = sortDateRange(value as Range<string>)
      }
    }

    from = (value as RangepickerValue).from
    to = (value as RangepickerValue).to
  }
  if (onchange) {
    onchange()
  }
}

watch([() => to, () => from], () => {
  // skip assignment if mode isn't rangepicker
  if (mode !== 'rangepicker') {
    return
  }
  if (to && from) {
    value = { from, to }
  } else if (from && !to) {
    value = { from, to: undefined }
  } else if (!from && to) {
    value = { from: undefined, to }
  } else {
    value = undefined
  }
})

watch(
  () => value,
  () => {
    // skip assignment if value hasn't changed so we don't create an infinite loop though i don't think it would anyway
    if (typeof value === 'object') {
      if (value.from === from && value.to === to) {
        return
      }
    }
    if (value === undefined) {
      from = undefined
      to = undefined
    } else if (typeof value === 'string') {
      from = undefined
      to = undefined
    } else if (typeof value === 'object') {
      from = value.from
      to = value.to
    }
  },
)

watch(
  () => [month, year],
  (_to, from) => {
    // if year not in calendarYearValues, add it as well as any missing years in between
    // otherwise the year would overflow to the first option in the select
    // which is like 1925 (current year - 100) which is just odd
    if (!calendarYearValues.find(y => y.value === year)) {
      let lastYearInCalendarValues = Math.max(
        ...calendarYearValues.map(y => y.value as number),
      )
      let missingYears = []
      for (let y = lastYearInCalendarValues + 1; y <= year; y++) {
        missingYears.push({ label: `${y}`, value: y })
      }
      calendarYearValues = [...calendarYearValues, ...missingYears]
    }
    // only get data if month or year changed
    // if from is undefined it is likely the initial run so we skip it
    // as getData is already called onMount
    if (from) {
      if (usedata && mode === 'navigation') {
        dataStore.calendarPosition = { year, month }
      }
      getData()
    }
  },
)
</script>

<div class="calendar">
  <div class="navigation">
    <button class="month-navigation" onclick={() => navigate(-1)}>
      <ChevronLeft />
    </button>

    <div class="month-year">
      <div class="month">
        <Select bind:value={month} options={monthValuesShort} />
      </div>
      <div class="year">
        <Select bind:value={year} options={calendarYearValues} />
      </div>
    </div>

    <button class="month-navigation" onclick={() => navigate(1)}>
      <ChevronRight />
    </button>
  </div>

  <div class="days">
    <div class="row weekdays">
      <div class="day">M</div>
      <div class="day">T</div>
      <div class="day">W</div>
      <div class="day">T</div>
      <div class="day">F</div>
      <div class="day">S</div>
      <div class="day">S</div>
    </div>

    {#each daysInMonth as week}
      <div class="row week">
        {#each week as day}
          <div
            class="day mood-{dayMood(day)}"
            class:today={isToday(day)}
            class:future={isFuture(day)}
            class:selected={isSelected(day)}
            class:between-selected={isBetweenSelected(day)}
            class:first-selected={isFirstSelected(day)}
            class:last-selected={isLastSelected(day)}
            class:active={isActive(day)}
            class:empty={day === null}>
            {#if day}
              {#if mode === 'navigation'}
                <a href={entryLink(day)} class="day-button">{day}</a>
              {:else}
                <button onclick={() => clickDay(day)} class="day-button">
                  {day}
                </button>
              {/if}
            {/if}
          </div>
        {/each}
      </div>
    {/each}
  </div>
</div>

<style lang="scss">
.calendar {
  width: 100%;

  .navigation {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--padding-s) 0;

    .month-navigation {
      background-color: transparent;
      border: none;
      color: var(--text-muted);
      cursor: pointer;
      padding: var(--padding-xs);
      border-radius: var(--button-radius);

      &:hover {
        background-color: var(--background-accent);
        color: var(--text-primary);
      }

      &:active {
        background-color: var(--background-primary);
        color: var(--text-primary);
        transform: var(--click-transform);
      }
    }

    .month-year {
      display: flex;
      flex-shrink: 1;
      gap: var(--padding-xs);
      align-items: center;

      .month {
        flex-shrink: 1;
      }
    }
  }

  .days {
    display: table;
    width: 100%;

    .row {
      display: table-row;

      &.week {
        height: calc(var(--button-height) + var(--calendar-button-spacing) * 2);
      }

      &.weekdays {
        color: var(--text-muted);
        font-size: var(--font-size-xs);

        .day {
          padding-bottom: var(--padding-s);
        }
      }

      .day {
        display: table-cell;
        text-align: center;
        width: calc(100% / 7);
        padding: var(--calendar-button-spacing);

        &.today {
          position: relative;

          .day-button {
            color: var(--text-primary);
          }

          &:after {
            content: '';
            display: block;
            width: 0.25rem;
            height: 0.25rem;
            background-color: var(--color-base-100);
            border-radius: 50%;
            position: absolute;
            top: 0.33rem;
            right: 0.33rem;
          }
        }

        &.future {
          .day-button {
            color: var(--text-dimmed);
          }
        }

        &.active {
          .day-button {
            font-weight: 600;
            color: var(--text-primary);
          }
        }

        @for $i from 1 through 5 {
          &.mood-#{$i} {
            .day-button {
              background-color: var(--mood-value-#{$i}-background);
              color: var(--mood-value-#{$i}-color);
            }

            .day-button:hover {
              background-color: var(--mood-value-#{$i}-background-hover);
              color: var(--mood-value-#{$i}-color);
            }

            .day-button:active {
              background-color: var(--mood-value-#{$i}-background-active);
              color: var(--mood-value-#{$i}-color);
            }
          }
        }

        .day-button {
          width: 100%;
          color: var(--text-muted);
          border: none;
          border-radius: var(--button-radius);
          height: var(--button-height);
          display: inline-block;
          line-height: var(--button-height);
          font-size: var(--font-size-s);
          background-color: transparent;
        }

        a.day-button,
        button.day-button {
          &:hover {
            background-color: var(--background-accent);
            color: var(--text-primary);
            cursor: pointer;
          }

          &:active {
            background-color: var(--background-primary);
            color: var(--text-primary);
            transform: var(--click-transform);
          }
        }

        &.selected {
          &:after {
            background-color: var(--calendar-selected-color);
          }

          a.day-button,
          button.day-button {
            background-color: var(--calendar-selected-background);
            color: var(--calendar-selected-color);

            &:hover {
              background-color: var(--calendar-selected-background);
              color: var(--calendar-selected-color);
            }
          }

          &.first-selected {
            padding-right: 0;

            .day-button {
              border-top-right-radius: 0;
              border-bottom-right-radius: 0;
            }
          }

          &.last-selected {
            padding-left: 0;

            .day-button {
              border-top-left-radius: 0;
              border-bottom-left-radius: 0;
            }
          }
        }

        &.between-selected {
          &:not(:last-child, :first-child) {
            padding-inline: 0;

            .day-button {
              border-radius: 0;
            }
          }
          &:first-child {
            padding-right: 0;

            .day-button {
              border-top-right-radius: 0;
              border-bottom-right-radius: 0;
            }
          }
          &:last-child {
            padding-left: 0;

            .day-button {
              border-top-left-radius: 0;
              border-bottom-left-radius: 0;
            }
          }

          &:after {
            background-color: var(--calendar-between-selected-color);
          }

          a.day-button,
          button.day-button {
            background-color: var(--calendar-between-selected-background);
            color: var(--calendar-between-selected-color);

            &:hover {
              background-color: var(--calendar-between-selected-background);
              color: var(--calendar-between-selected-color);
            }
          }
        }
      }
    }
  }

  &:has(a.day-button:hover, button.day-button:hover) {
    .day {
      @for $i from 1 through 5 {
        &.mood-#{$i} {
          &:not(.selected, .between-selected) {
            .day-button {
              &:not(:hover) {
                background-color: var(--mood-value-#{$i}-background-muted);
              }
            }
          }
        }
      }
    }
  }
}
</style>

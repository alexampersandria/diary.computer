<script lang="ts">
import type { TableField, TableProps } from '$lib/types/components/table'
import { formatKey } from '$lib/utils/formatKey'
import {
  ArrowDown,
  ArrowUp,
  ArrowUpDown,
  SquareArrowOutUpRight,
} from 'lucide-svelte'
import Spinner from './Spinner.svelte'
import { formatNumber } from '$lib/utils/numbers'

let {
  fields,
  data,
  sortedBy = $bindable(),
  sortDirection = $bindable('desc'),
  compareAverageTo,
}: TableProps = $props()

const switchDirection = () => {
  if (sortDirection === 'asc') {
    sortDirection = 'desc'
  } else {
    sortDirection = 'asc'
  }
}

const sort = (fieldKey: string) => {
  if (sortedBy === fieldKey) {
    switchDirection()
  } else {
    sortedBy = fieldKey
    sortDirection = 'desc'
  }
}

const sortedData = $derived.by(() => {
  if (!data) {
    return undefined
  }

  if (!sortedBy) {
    return data
  }

  return data.slice().sort((a, b) => {
    const aValue = a[sortedBy as string]
    const bValue = b[sortedBy as string]

    if (aValue < bValue) {
      return sortDirection === 'asc' ? -1 : 1
    } else if (aValue > bValue) {
      return sortDirection === 'asc' ? 1 : -1
    } else {
      return 0
    }
  })
})

const derivedFields: TableField[] = $derived.by(() => {
  if (fields && fields.length > 0) {
    return fields
  }

  if (!data || data.length === 0) {
    return []
  }

  return Object.keys(data[0]).map(key => ({
    key,
    label: formatKey(key),
    sortable: false,
  }))
})

// needs to be any, ignore ts error
// eslint-disable-next-line @typescript-eslint/no-explicit-any
const delta = (key: string, value: any): number | null => {
  if (compareAverageTo === undefined) {
    return null
  }

  let delta: number | null = null
  if (
    (key.startsWith('average') || key.startsWith('avg')) &&
    typeof value === 'number'
  ) {
    delta = value - compareAverageTo
  }
  return delta
}

const formatDelta = (delta: number): string => {
  let formatted = formatNumber(delta, { decimals: true })
  if (delta === 0) {
    // fixed response for zero values
    return `±0.00`
  } else if (delta > 0) {
    // prepend plus sign for positive numbers
    return `+${formatted}`
  } else {
    // negative number
    // already has minus sign
    return `${formatted}`
  }
}

type DeltaExtremes = {
  min: number
  max: number
}
const deltaExtremes = $derived.by((): DeltaExtremes | null => {
  if (compareAverageTo === undefined || !data) {
    return null
  }

  let min = Infinity
  let max = -Infinity

  data.forEach(row => {
    Object.keys(row).forEach(key => {
      if (
        (key.startsWith('average') || key.startsWith('avg')) &&
        typeof row[key] === 'number'
      ) {
        const value = row[key] as number
        const deltaValue = value - compareAverageTo
        if (deltaValue < min) {
          min = deltaValue
        }
        if (deltaValue > max) {
          max = deltaValue
        }
      }
    })
  })

  if (min === Infinity || max === -Infinity) {
    return null
  }

  return { min, max }
})

/**
 * Get the position of the delta value within the extremes range
 * Returns a number between -1 and 1, or null if extremes are not defined
 * Negative values are from 0 to deltaExtremes.min
 * Positive values are from 0 to deltaExtremes.max
 * @param delta
 */
const deltaPosition = (delta: number): number | null => {
  if (deltaExtremes === null) {
    return null
  }

  const range = deltaExtremes.max - deltaExtremes.min
  if (range === 0) {
    return 0
  }

  if (delta < 0) {
    return delta / Math.abs(deltaExtremes.min)
  } else {
    return delta / deltaExtremes.max
  }
}

/**
 * returns css style string for delta color based on position
 * using color-mix
 */
const deltaStyle = (position: number | null): string => {
  if (position === null) {
    return ''
  }

  const neutral = 'var(--color-neutral)'
  const positive = 'var(--color-positive)'
  const slightlyPositive = 'var(--color-slightly-positive)'
  const negative = 'var(--color-negative)'
  const slightlyNegative = 'var(--color-slightly-negative)'

  if (position < 0) {
    if (position > -0.5) {
      const weight = Math.min(Math.abs(position) * 200, 100)
      return `color: color-mix(in srgb, ${slightlyNegative} ${weight}%, ${neutral} ${
        100 - weight
      }%);`
    } else {
      const weight = Math.min((Math.abs(position) - 0.5) * 200, 100)
      return `color: color-mix(in srgb, ${negative} ${weight}%, ${slightlyNegative} ${
        100 - weight
      }%);`
    }
  } else {
    if (position < 0.5) {
      const weight = Math.min(position * 200, 100)
      return `color: color-mix(in srgb, ${slightlyPositive} ${weight}%, ${neutral} ${
        100 - weight
      }%);`
    } else {
      const weight = Math.min((position - 0.5) * 200, 100)
      return `color: color-mix(in srgb, ${positive} ${weight}%, ${slightlyPositive} ${
        100 - weight
      }%);`
    }
  }
}
</script>

<table class="table">
  <thead class="fields">
    <tr>
      {#each derivedFields as field}
        <th class="field" data-key={field.key} title={field.label}>
          <div class="field-container">
            <div class="label">
              {field.label}
            </div>
            {#if field.sortable}
              <button
                class="sort"
                class:active={sortedBy === field.key}
                aria-label="Sort by {field.label}"
                onclick={() => {
                  sort(field.key)
                }}>
                {#if sortedBy === field.key}
                  {#if sortDirection === 'asc'}
                    <ArrowUp />
                  {:else}
                    <ArrowDown />
                  {/if}
                {:else}
                  <ArrowUpDown />
                {/if}
              </button>
            {/if}
          </div>
        </th>
      {/each}
    </tr>
  </thead>

  <tbody class="data">
    {#if sortedData === undefined}
      <tr class="loading-row">
        <td class="cell loading-cell" colspan={derivedFields.length}>
          <div class="loading-element">
            <Spinner />
          </div>
        </td>
      </tr>
    {:else if sortedData.length === 0}
      <tr>
        <td class="cell" colspan={derivedFields.length}>
          No data available.
        </td>
      </tr>
    {:else}
      {#each sortedData as row, rowIndex}
        <tr class="row" data-index={rowIndex}>
          {#each derivedFields as field}
            {@const cellValue = row[field.key]}
            {@const dataType = typeof cellValue}
            {@const valueDelta = delta(field.key, cellValue)}
            <td
              class="cell"
              data-type={dataType}
              data-key={field.key}
              data-value={cellValue}
              data-delta={valueDelta}>
              <div class="cell-content">
                <div class="value">
                  {#if dataType === 'number'}
                    {#if valueDelta !== null}
                      {formatNumber(cellValue, { decimals: true })}
                    {:else}
                      {formatNumber(cellValue)}
                    {/if}
                  {:else if dataType === 'object'}
                    {#if cellValue.href && cellValue.label}
                      <div class="flex-between">
                        <div class="label">
                          {cellValue.label}
                        </div>
                        <div class="link dimmed">
                          <a href={cellValue.href}>
                            <SquareArrowOutUpRight />
                          </a>
                        </div>
                      </div>
                    {:else}
                      <code>{JSON.stringify(cellValue)}</code>
                    {/if}
                  {:else}
                    {cellValue}
                  {/if}
                </div>
                {#if valueDelta !== null}
                  {@const formattedDelta = formatDelta(valueDelta)}
                  {@const position = deltaPosition(valueDelta)}
                  <div
                    class="delta"
                    style={deltaStyle(position)}
                    aria-label={`delta of ${formattedDelta}, value of ${cellValue} compared to average ${compareAverageTo}`}>
                    {formattedDelta}
                  </div>
                {/if}
              </div>
            </td>
          {/each}
        </tr>
      {/each}
    {/if}
  </tbody>
</table>

<style lang="scss">
.table {
  width: 100%;
  --table-radius: var(--radius-m);
  border-spacing: 0;
  border-radius: var(--table-radius);
  overflow: hidden;
  border: var(--border-width) solid var(--border-color);

  .field {
    background-color: var(--background-secondary);
  }

  .field,
  .cell {
    padding: var(--padding-xs) var(--padding-m);
    text-align: left;

    &:not(:first-child) {
      border-left: var(--border-width) solid var(--border-color);
    }
  }

  tbody {
    tr {
      .cell {
        border-top: var(--border-width) solid var(--border-color);
      }
    }
  }

  .cell {
    &[data-type='number'] {
      text-align: right;

      .cell-content {
        justify-content: flex-end;
      }
    }

    .cell-content {
      display: flex;
      align-items: center;
      gap: var(--padding-xs);

      .delta {
        font-size: var(--font-size-xs);
        color: var(--text-muted);
        min-width: 5ch;
        font-family: 'Fira Code', monospace;
      }

      .value {
        width: 100%;
      }
    }
  }

  .field {
    .field-container {
      &:has(.sort) {
        position: relative;

        .label {
          width: calc(100% - (1em + (var(--padding-s) * 2)));
        }
      }

      .label {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }
    }
  }

  .sort {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: var(--font-size-m);
    padding: var(--padding-xxs) var(--padding-s);
    position: absolute;
    top: 0;
    right: 0;
    width: 100%;
    text-align: right;

    &.active,
    &:hover {
      color: var(--text-primary);
    }
  }

  .loading-row {
    .loading-cell {
      .loading-element {
        display: flex;
        justify-content: center;
        align-items: center;
        padding: var(--padding-l);
      }
    }
  }

  thead {
    tr:first-child {
      .field {
        &:first-child {
          border-top-left-radius: var(--table-radius);
        }
        &:last-child {
          border-top-right-radius: var(--table-radius);
        }
      }
    }
  }
}
</style>

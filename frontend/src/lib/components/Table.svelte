<script lang="ts">
import type { TableField, TableProps } from '$lib/types/components/table'
import { formatKey } from '$lib/utils/formatKey'
import { ArrowDown, ArrowUp, ArrowUpDown } from 'lucide-svelte'
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
</script>

<table class="table">
  <thead class="fields">
    <tr>
      {#each derivedFields as field}
        <th class="field" data-key={field.key}>
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
                  {:else}
                    {cellValue}
                  {/if}
                </div>
                {#if valueDelta !== null}
                  {@const formattedDelta = formatDelta(valueDelta)}
                  <div
                    class="delta"
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
  border-collapse: collapse;
  --table-radius: var(--radius-m);

  .field {
    background-color: var(--background-secondary);
  }

  .field,
  .cell {
    padding: var(--padding-xs) var(--padding-m);
    border: var(--border-width) solid var(--border-color);
    text-align: left;
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
    }
  }

  .field {
    .field-container {
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: var(--padding-xs);
    }
  }

  .sort {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: var(--font-size-m);
    padding: var(--padding-xxs) var(--padding-s);

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

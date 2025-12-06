<script lang="ts">
import type { ColorPickerProps } from '$lib/types/components/colorpicker'
import { colorPriority, colors as defaultColors } from '$lib/types/color'

let {
  colors = [...defaultColors],
  value = $bindable(),
  onChange,
  fullwidth,
  inputstate = $bindable('untouched'),
  id,
  'aria-label': ariaLabel = 'Color picker',
}: ColorPickerProps = $props()

const onclick = (selectedColor: string) => {
  value = selectedColor
  inputstate = 'touched'
  if (onChange) {
    onChange(selectedColor)
  }
}

const sortedColors = $derived.by(() => {
  return [...colors].sort((a, b) => {
    return colorPriority[b] - colorPriority[a]
  })
})
</script>

<div
  class="color-picker"
  class:fullwidth
  role="radiogroup"
  {id}
  aria-label={ariaLabel}
  aria-invalid={inputstate === 'invalid'}>
  {#each sortedColors as option}
    <div class="option color-{option}" class:selected={value === option}>
      <button
        class="option-inner plain"
        onclick={() => onclick(option)}
        role="radio"
        aria-checked={value === option}
        aria-label={`Select color ${option}`}>
      </button>
    </div>
  {/each}
</div>

<!-- svelte-ignore css_unused_selector -->
<style lang="scss">
@use '../assets/scss/color';

.color-picker {
  display: flex;
  flex-wrap: wrap;
  padding: var(--input-padding);
  padding-inline: 0;
  gap: var(--padding-xs);

  &.fullwidth {
    width: 100%;
    justify-content: space-between;
  }

  .option {
    // #TODO: add colors to color variables
    --color-picker-option-color: var(--color-base-40);
    display: flex;
    align-items: center;
    justify-content: center;

    .option-inner {
      position: relative;
      width: var(--font-size-xl);
      aspect-ratio: 1 / 1;
      border-radius: var(--radius-xxs);
      background-color: var(--color-picker-option-color);
      transition: box-shadow var(--animation-length-s) var(--better-ease-out);

      box-shadow:
        0 0 0 0 var(--background-primary),
        0 0 0 0 var(--color-picker-option-color);

      &:after {
        content: '';
        position: absolute;
        bottom: var(--padding-xxs);
        right: var(--padding-xxs);
        width: 4px;
        aspect-ratio: 1 / 1;
        border-radius: 9999px;
        background-color: transparent;
        transition: background-color var(--animation-length-m)
          var(--better-ease-out);
      }
    }

    &.selected {
      .option-inner {
        box-shadow:
          0 0 0 var(--focus-shadow-offset) var(--background-primary),
          0 0 0 calc(var(--focus-shadow-offset) + var(--focus-shadow-offset))
            var(--color-picker-option-color);

        &:after {
          background-color: var(--color-base-100);
        }
      }
    }

    @each $color in color.$colors {
      &.color-#{$color} {
        --color-picker-option-color: var(--color-#{$color}-70);
      }
    }
  }
}
</style>

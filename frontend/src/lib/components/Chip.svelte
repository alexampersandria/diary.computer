<script lang="ts">
import type { ChipProps } from '$lib/types/components/chip'
import { onMount } from 'svelte'

let {
  children,
  color = 'base',
  solid = false,
  outline = false,
}: ChipProps = $props()

let chip: HTMLDivElement | null = null
let isSingleCharacter = $state(false)

const evalSingleCharacter = () => {
  if (!chip) return
  const content = chip.textContent?.trim() || ''
  const hasOnlyIcon =
    content.length === 0 &&
    chip.children.length === 1 &&
    chip.children[0].tagName === 'svg' &&
    chip.children[0].classList.contains('lucide')
  isSingleCharacter = content.length === 1 || hasOnlyIcon
}

onMount(() => {
  evalSingleCharacter()

  const observer = new MutationObserver(() => {
    evalSingleCharacter()
  })

  observer.observe(chip!, {
    childList: true,
    subtree: true,
    characterData: true,
  })
})
</script>

<div
  class="chip {color}"
  class:solid
  class:outline
  class:single-character={isSingleCharacter}
  bind:this={chip}>
  {@render children()}
</div>

<!-- svelte-ignore css_unused_selector -->
<style lang="scss">
@use '../assets/scss/color';

.chip {
  vertical-align: middle;
  border-radius: 9999px;
  font-size: var(--font-size-xs);
  padding: 0 var(--padding-s);
  flex-shrink: 0;
  text-align: center;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  box-shadow:
    0 0 0 0 var(--background-primary),
    0 0 0 0 var(--background-primary);
  height: var(--chip-height);
  line-height: var(--chip-height);
  min-width: var(--chip-min-width);

  :global(.lucide) {
    min-width: 1em;
    min-height: 1em;
  }

  &.single-character {
    display: inline-flex;
    gap: var(--padding-xxs);
    align-items: center;
    justify-content: center;

    min-width: var(--chip-height);
    width: var(--chip-height);
  }

  @each $color in color.$colors {
    &.#{$color} {
      background-color: var(--chip-#{$color}-background);
      color: var(--chip-#{$color}-color);

      &.outline {
        box-shadow:
          0 0 0 var(--focus-shadow-offset) var(--background-primary),
          0
            0
            0
            calc(var(--focus-shadow-offset) + var(--focus-shadow-offset))
            var(--chip-#{$color}-background);
      }

      &.solid {
        background-color: var(--chip-solid-#{$color}-background);
        color: var(--chip-solid-#{$color}-color);

        &.outline {
          box-shadow:
            0 0 0 var(--focus-shadow-offset) var(--background-primary),
            0
              0
              0
              calc(var(--focus-shadow-offset) + var(--focus-shadow-offset))
              var(--chip-solid-#{$color}-background);
        }
      }
    }
  }
}

:global(button:has(.chip)) {
  border: none;
  background: none;
  padding: 0;
  margin: 0;
  font: inherit;
  color: inherit;
  cursor: pointer;
}

:global(.button:has(.chip)) {
  .chip {
    height: var(--chip-inside-button-height);
    min-width: var(--chip-inside-button-min-width);

    &.single-character {
      min-width: var(--chip-inside-button-height);
      width: var(--chip-inside-button-height);
    }
  }
}

:global(a:has(.chip)),
:global(button:not(.button):has(.chip)) {
  line-height: 1;
  border-radius: 9999px;

  @each $color in color.$colors {
    &:hover .chip.#{$color} {
      background-color: var(--chip-#{$color}-background-hover);

      &.solid {
        background-color: var(--chip-solid-#{$color}-background-hover);
      }
    }

    &:active .chip.#{$color} {
      background-color: var(--chip-#{$color}-background-active);
      transform: var(--click-transform);

      &.solid {
        background-color: var(--chip-solid-#{$color}-background-active);
      }
    }
  }

  &:focus-visible {
    .chip {
      &.outline {
        box-shadow:
          0 0 0 0 var(--background-primary),
          0 0 0 0 var(--background-primary);
      }
    }
  }
}
</style>

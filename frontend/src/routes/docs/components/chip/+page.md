<script>
import Chip from '$lib/components/Chip.svelte';
import Button from '$lib/components/Button.svelte';
import DocsExample from '$lib/components/utils/DocsExample.svelte';

import { Pencil } from 'lucide-svelte'

let count = $state(1)

const incrementCount = () => {
  count++
  if (count < 99) {
    setTimeout(incrementCount, Math.max(10, count * 10 - 800))
  }
}

setTimeout(incrementCount, 500)
</script>

# Chip

Small rounded elements used to display tags, labels, status indicators, or other compact information.

## Usage

Used to display compact information like tags, categories, status indicators, or notifications. Chips automatically adjust their sizing - single characters become circular, while longer content remains pill-shaped.

### Basic Usage

<DocsExample>
  Messages
  <Chip>
    { count }
  </Chip>
</DocsExample>

```svelte
<script>
let count = $state(1)
const incrementCount = () => {
  count++
  if (count < 99) {
    setTimeout(incrementCount, Math.max(10, count * 10 - 800))
  }
}
incrementCount()
</script>

<template>
  Messages
  <Chip>
    {count}
  </Chip>
</template>
```

### Interactable Chip

Chips inside an `a` tag, or other interactable elements like the [Button](/docs/components/button) component or `button` tag, have styling to show that they are interactable. This is useful for things like tags that are interactable, but not a replacement for the Button component and should not be used as such.

<DocsExample>
  <a href="#interactable-chip">
    <Chip>new</Chip>
  </a>
  <button>
    <Chip color="red">delete</Chip>
  </button>
  <Button>
    Unread <Chip solid>13</Chip>
  </Button>
</DocsExample>

```svelte
<a href="#interactable-chip">
  <Chip>new</Chip>
</a>
<button>
  <Chip color="red">delete</Chip>
</button>
<Button>
  Unread <Chip solid>13</Chip>
</Button>
```

### Single Character

Single character chips automatically become circular:

<DocsExample>
  <Chip>1</Chip>
  <Chip>A</Chip>
  <Chip>!</Chip>
</DocsExample>

```svelte
<Chip>1</Chip>
<Chip>A</Chip>
<Chip>!</Chip>
```

### Multi-character

Longer content maintains the pill shape:

<DocsExample>
  <Chip>New</Chip>
  <Chip>Featured</Chip>
  <Chip>Premium</Chip>
</DocsExample>

```svelte
<Chip>New</Chip>
<Chip>Featured</Chip>
<Chip>Premium</Chip>
```

## Color

Chips support various color options for different semantic meanings:

<DocsExample>
  <Chip>
    base
  </Chip>
  <Chip color="blue">
    blue
  </Chip>
  <Chip color="green">
    green
  </Chip>
  <Chip color="red">
    red
  </Chip>
  <Chip color="yellow">
    yellow
  </Chip>
  <Chip color="purple">
    purple
  </Chip>
  <Chip color="pink">
    pink
  </Chip>
</DocsExample>

```svelte
<Chip>base</Chip>
<Chip color="blue">blue</Chip>
<Chip color="green">green</Chip>
<Chip color="red">red</Chip>
<Chip color="yellow">yellow</Chip>
<Chip color="purple">purple</Chip>
<Chip color="pink">pink</Chip>
```

## Solid

Chip with solid backgrounds and light text, used for selected state or for chips that need to stand out visually

<DocsExample>
  <Chip solid>Solid</Chip>
  <Chip solid color="blue">Blue Solid</Chip>
  <Chip solid color="green">Green Solid</Chip>
  <Chip solid color="red">Red Solid</Chip>
  <Chip solid color="yellow">Yellow Solid</Chip>
  <Chip solid color="purple">Purple Solid</Chip>
  <Chip solid color="pink">Pink Solid</Chip>
</DocsExample>

```svelte
<Chip solid>Solid</Chip>
<Chip solid color="blue">Blue Solid</Chip>
<Chip solid color="green">Green Solid</Chip>
<Chip solid color="red">Red Solid</Chip>
<Chip solid color="yellow">Yellow Solid</Chip>
<Chip solid color="purple">Purple Solid</Chip>
<Chip solid color="pink">Pink Solid</Chip>
```

## Outline

Chip with an outline, _also_ used for selected state or for chips that need to stand out visually

<DocsExample>
  <Chip outline>Outline</Chip>
  <Chip outline solid>Outline</Chip>
  <Chip color="blue" outline>Outline</Chip>
  <Chip color="pink" outline>Outline</Chip>
  <Chip color="red" outline solid>Outline</Chip>
  <Chip color="yellow" outline solid>Outline</Chip>
</DocsExample>

```svelte
<Chip outline>Outline</Chip>
<Chip outline solid>Outline</Chip>
<Chip color="blue" outline>Outline</Chip>
<Chip color="pink" outline>Outline</Chip>
<Chip color="red" outline solid>Outline</Chip>
<Chip color="yellow" outline solid>Outline</Chip>
```

## Icon Content

Icons can be used inside the chip as well, usefull for tiny buttons.

<DocsExample>
  <button>
    <Chip>
      <Pencil />
    </Chip>
  </button>
</DocsExample>

```svelte
<button>
  <Chip>
    <Pencil />
  </Chip>
</button>
```

## Types

### Props

| Name     | Type      | Required | Default   | Description                                                              |
| -------- | --------- | :------: | --------- | ------------------------------------------------------------------------ |
| children | `Snippet` |    ✅    |           | Content to display inside the chip.                                      |
| color    | `Color`   |          | `'base'`  | Color theme of the chip.                                                 |
| solid    | `boolean` |          | `'balse'` | Whether the chip should be solid or not                                  |
| outline  | `boolean` |          | `false`   | If the chip should have an outline, used to indicate selected value etc. |

## References

- [Color](/docs/design/color)

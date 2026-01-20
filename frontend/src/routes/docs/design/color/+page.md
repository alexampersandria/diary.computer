<script>
import DocsColorRow from '$lib/components/utils/DocsColorRow.svelte'
import Alert from '$lib/components/Alert.svelte'
</script>

# Color

<Alert>
  The component for previewing the colors are broken when built for some reason, they only work properly when running the dev server yourself. You can track the issue here <a href="https://github.com/alexampersandria/diary.computer/issues/31" target="_blank">#31</a>.
</Alert>

The color palette used throughout diary.computer's design system. Each color comes in multiple shades, ranging from light to dark. From `00` to `100` in increments of `10`.

## Base

Base also includes `05` and `95` shades for extra light and extra dark variants and thus has two more steps than the other colors.

<DocsColorRow color="base" />

## Blue

<DocsColorRow color="blue" />

## Green

<DocsColorRow color="green" />

## Lime

<DocsColorRow color="lime" />

## Yellow

<DocsColorRow color="yellow" />

## Orange

<DocsColorRow color="orange" />

## Red

<DocsColorRow color="red" />

## Purple

<DocsColorRow color="purple" />

## Pink

<DocsColorRow color="pink" />

<script>
import DocsExample from '$lib/components/utils/DocsExample.svelte'
import EntryPreview from '$lib/components/EntryPreview.svelte'

let sampleEntry = {
  id: '1',
  user_id: '1',
  date: '2025-11-27',
  created_at: Date.now(),
  mood: 4,
  entry: 'Had a great day working on the project!',
  selected_tags: []
}

let emptyEntry = null
</script>

# Entry Preview

A preview card component that displays a summary of a journal entry, including the date, mood indicator, and associated tags. Used in lists and calendars to provide a quick overview of entries.

## Usage

Entry Preview displays a clickable card that links to the full entry view.

### With Entry

When an entry exists for a date, it shows the date, tags, and mood.

<DocsExample>
  <EntryPreview date="2025-11-27" entry={sampleEntry} />
</DocsExample>

```svelte
<script>
let entry = {
  id: '1',
  user_id: '1',
  date: '2025-11-27',
  created_at: Date.now(),
  mood: 4,
  entry: 'Had a great day working on the project!',
  selected_tags: [],
}
</script>

<EntryPreview date="2025-11-27" {entry} />
```

### Without Entry

When no entry exists for a date, it displays "No entry yet".

<DocsExample>
  <EntryPreview date="2025-11-26" entry={null} />
</DocsExample>

```svelte
<EntryPreview date="2025-11-26" entry={null} />
```

### Small Variant

A compact version for use in tight spaces like calendars.

<DocsExample>
  <EntryPreview date="2025-11-27" entry={sampleEntry} small />
</DocsExample>

```svelte
<EntryPreview date="2025-11-27" {entry} small />
```

### Without Tags

When an entry exists but has no tags selected.

<DocsExample>
  <EntryPreview date="2025-11-25" entry={{...sampleEntry, selected_tags: []}} />
</DocsExample>

```svelte
<script>
let entry = {
  id: '1',
  user_id: '1',
  date: '2025-11-25',
  created_at: Date.now(),
  mood: 3,
  entry: 'A simple entry',
  selected_tags: [],
}
</script>

<EntryPreview date="2025-11-25" {entry} />
```

## Mood Display

The mood indicator on the right side displays the mood value (1-5) with color-coded backgrounds:

<DocsExample column>
  <EntryPreview date="2025-11-20" entry={{...sampleEntry, mood: 1}} />
  <EntryPreview date="2025-11-21" entry={{...sampleEntry, mood: 2}} />
  <EntryPreview date="2025-11-22" entry={{...sampleEntry, mood: 3}} />
  <EntryPreview date="2025-11-23" entry={{...sampleEntry, mood: 4}} />
  <EntryPreview date="2025-11-24" entry={{...sampleEntry, mood: 5}} />
</DocsExample>

```svelte
<EntryPreview date="2025-11-20" entry={{ ...entry, mood: 1 }} />
<EntryPreview date="2025-11-21" entry={{ ...entry, mood: 2 }} />
<EntryPreview date="2025-11-22" entry={{ ...entry, mood: 3 }} />
<EntryPreview date="2025-11-23" entry={{ ...entry, mood: 4 }} />
<EntryPreview date="2025-11-24" entry={{ ...entry, mood: 5 }} />
```

## Types

### Props

| Name  | Type            | Required | Default | Description                                            |
| ----- | --------------- | :------: | ------- | ------------------------------------------------------ |
| date  | `string`        |    ✅    |         | The date string for the entry (YYYY-MM-DD format).     |
| entry | `Entry \| null` |          | `null`  | The entry data to display, or null if no entry exists. |
| small | `boolean`       |          | `false` | Whether to use the compact variant.                    |

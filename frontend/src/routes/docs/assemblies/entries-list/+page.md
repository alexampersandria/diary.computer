<script>
import DocsExample from '$lib/components/utils/DocsExample.svelte'
import EntriesList from '$lib/assemblies/EntriesList.svelte'

const entries = [
  {
    id: '1',
    user_id: 'me',
    date: '1998-01-01',
    created_at: 1234567890,
    mood: 3,
    entry: undefined,
    selected_tags: []
  },
  {
    id: '2',
    user_id: 'me',
    date: '1998-01-02',
    created_at: 1234567890,
    mood: 5,
    entry: undefined,
    selected_tags: []
  },
]

const pagination = {
  size: 20,
  offset: 0,
  total_count: 2
}
</script>

# Entries List

List of [EntryPreview](/docs/components/entry-preview) components, representing the entries in an assembly with a load more button

## Usage

<DocsExample>
  <EntriesList {entries} {pagination} onloadmore={() => {}} />
</DocsExample>

```svelte
<script>
const entries = [
  {
    id: '1',
    user_id: 'me',
    date: '1998-01-01',
    created_at: 1234567890,
    mood: 3,
    entry: undefined,
    selected_tags: [],
  },
  {
    id: '2',
    user_id: 'me',
    date: '1998-01-02',
    created_at: 1234567890,
    mood: 5,
    entry: undefined,
    selected_tags: [],
  },
]
</script>

<EntriesList {entries} {pagination} onloadmore={() => {}} />
```

## Types

### Props

| Name       | Type                  | Required | Default | Description                            |
| ---------- | --------------------- | :------: | ------- | -------------------------------------- |
| entries    | `Entry[]`             |          |         | List of entry objects to be displayed  |
| pagination | `PaginationObject`    |          |         | Pagination details for the entries     |
| loading    | `boolean`             |          | `false` | Whether the list is in a loading state |
| onloadmore | `() => Promise<void>` |          |         | Callback function                      |

## References

- [EntryPreview Assembly](/docs/components/entry-preview)

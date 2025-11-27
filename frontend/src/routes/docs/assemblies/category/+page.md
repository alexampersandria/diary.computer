<script>
import DocsExample from '$lib/components/utils/DocsExample.svelte'
import Category from '$lib/assemblies/Category.svelte'

let tags = $state([
  {
    id: '1',
    name: 'Work',
    color: 'base'
  },
  {
    id: '2',
    name: 'Movie',
    color: 'pink'
  },
  {
    id: '3',
    name: 'Exercise',
    color: 'yellow'
  },
  {
    id: '4',
    name: 'Read',
    color: 'green'
  },
  {
    id: '5',
    name: 'Shopping',
    color: 'blue'
  },
  {
    id: '6',
    name: 'Gaming',
    color: 'red'
  },
])

let selectedTagIds = $state([])
</script>

# Category

A component for organizing and displaying tags within a category. Categories can be viewed, edited, or used for tag selection with support for adding, editing, and removing tags

## Usage

The Category component provides multiple modes for different use cases: viewing tags, selecting tags, and managing tags

### Basic View Mode

By default, the category displays in view mode where tags are clickable links

<DocsExample>
  <Category id="1" name="Activities" tags={tags} />
</DocsExample>

```svelte
<script>
let tags = $state([
  {
    id: '1',
    name: 'Work',
    color: 'base',
  },
  // ...
  {
    id: '6',
    name: 'Gaming',
    color: 'red',
  },
])
</script>

<Category id="1" name="Activities" {tags} />
```

### Select Mode

In select mode, users can select and deselect tags. Selected tags are displayed with an outline and solid background

<DocsExample>
  <Category id="1" name="Activities" mode="select" tags={tags} bind:selectedTagIds />
</DocsExample>
<DocsExample column>
  <p style="margin: 0;">Selected: <code>{selectedTagIds.join(', ') || 'none'}</code></p>
</DocsExample>

```svelte
<script>
let tags = $state([
  {
    id: '1',
    name: 'Work',
    color: 'base',
  },
  // ...
])
let selectedTagIds = $state([])
</script>

<Category id="1" name="Activities" mode="select" {tags} bind:selectedTagIds />

Selected: <code>{selectedTagIds.join(', ') || 'none'}</code>
```

### Edit Mode

Edit mode allows adding and editing tags within the category. Click on a tag to edit it, or use the "Add tag" button

<DocsExample>
  <Category id="1" name="Activities" mode="edit" tags={tags} />
</DocsExample>

```svelte
<Category id="1" name="Activities" mode="edit" {tags} />
```

### Empty Category

When a category has no tags, it displays a "No tags" message

<DocsExample>
  <Category id="2" name="Empty Category" tags={[]}  />
</DocsExample>

```svelte
<Category id="2" name="Empty Category" tags={[]} />
```

## Types

### Props

| Name           | Type                                             | Required | Default  | Description                                        |
| -------------- | ------------------------------------------------ | :------: | -------- | -------------------------------------------------- |
| id             | `string`                                         |    ✅    |          | Unique identifier for the category.                |
| name           | `string`                                         |    ✅    |          | Display name of the category.                      |
| tags           | `Tag[]`                                          |          | `[]`     | Array of tags belonging to this category.          |
| selectedTagIds | `string[]`                                       |          | `[]`     | Array of selected tag IDs (bindable).              |
| mode           | `'view' \| 'edit' \| 'select' \| 'select-edit'`  |          | `'view'` | Display mode of the category.                      |
| onAddTag       | `(tag: NewTag) => Promise<Tag \| null>`          |          |          | Callback when a new tag is added.                  |
| onRemoveTag    | `(id: string) => Promise<boolean \| null>`       |          |          | Callback when a tag is removed.                    |
| onSelectTag    | `(tag: Tag, selected: boolean) => Promise<void>` |          |          | Callback when a tag is selected or deselected.     |
| onEditTag      | `(tag: Tag) => Promise<Tag \| null>`             |          |          | Callback when a tag is edited.                     |
| onEditCategory | `() => void`                                     |          |          | Callback when the category edit button is clicked. |

## References

- [Chip Component](/docs/components/chip)

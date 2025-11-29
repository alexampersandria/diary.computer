<script lang="ts">
import Alert from '$lib/components/Alert.svelte'
import Button from '$lib/components/Button.svelte'
import Chip from '$lib/components/Chip.svelte'
import Input from '$lib/components/Input.svelte'
import Modal from '$lib/components/Modal.svelte'
import type { CategoryProps } from '$lib/types/assemblies/category'
import { type Color } from '$lib/types/color'
import type { InputState } from '$lib/types/input'
import {
  Pencil,
  PencilOff,
  Plus,
  Save,
  Settings,
  TagIcon,
  Trash,
  X,
} from 'lucide-svelte'
import type { NewTag, Tag } from '$lib/types/log'
import ColorPicker from '$lib/components/ColorPicker.svelte'
import Label from '$lib/components/Label.svelte'
import { takeAtLeast } from '$lib/utils/takeAtLeast'

let {
  id,
  name,
  tags = [],
  selectedTagIds = $bindable([]),
  mode = 'view',
  onAddTag,
  onRemoveTag,
  onSelectTag,
  onEditTag,
  onEditCategory,
}: CategoryProps = $props()

const clickTag = (tag: Tag) => {
  if (mode === 'select') {
    if (selectedTagIds.includes(tag.id)) {
      selectedTagIds = selectedTagIds.filter(id => id !== tag.id)
    } else {
      selectedTagIds = [...selectedTagIds, tag.id]
    }

    if (onSelectTag) {
      onSelectTag(tag, selectedTagIds.includes(tag.id))
    }
  } else if (mode === 'edit' || mode === 'select-edit') {
    openEditTag(tag.id)
  }
}

const onedit = () => {
  if (mode === 'select') {
    mode = 'select-edit'
  } else if (mode === 'select-edit') {
    mode = 'select'
  }
}

const onclear = () => {
  // remove only tags in category from selectedTagIds
  selectedTagIds = selectedTagIds.filter(id => !tags.find(tag => tag.id === id))
}

let tagDetails: {
  mode: 'add' | 'edit'
  open: boolean
  id?: string
  name: {
    value?: string
    inputstate: InputState
  }
  color: {
    value?: Color
    inputstate: InputState
  }
  errors: string[]
  loading: boolean
  deleteLoading: boolean
} = $state({
  mode: 'add',
  open: false,
  id: undefined,
  name: {
    value: undefined,
    inputstate: 'untouched',
  },
  color: {
    value: undefined,
    inputstate: 'untouched',
  },
  errors: [] as string[],
  loading: false,
  deleteLoading: false,
})

const openAddTag = () => {
  tagDetails.open = true
  tagDetails.mode = 'add'
  tagDetails.id = undefined
  tagDetails.name.value = undefined
  tagDetails.name.inputstate = 'untouched'
  tagDetails.color.value = undefined
  tagDetails.color.inputstate = 'untouched'
  tagDetails.errors = []
  tagDetails.loading = false
  tagDetails.deleteLoading = false
}

const openEditTag = (tagId: string) => {
  const tag = tags.find(tag => tag.id === tagId)
  if (tag) {
    tagDetails.open = true
    tagDetails.errors = []
    tagDetails.loading = false
    tagDetails.deleteLoading = false
    tagDetails.mode = 'edit'
    tagDetails.id = tag.id
    tagDetails.name.value = tag.name
    tagDetails.name.inputstate = 'touched'
    tagDetails.color.value = tag.color
    tagDetails.color.inputstate = 'touched'
  }
}

const closeTagDetails = () => {
  tagDetails.open = false
}

const validateTagDetails = (requireUntouched = true) => {
  tagDetails.errors = []
  if (tagDetails.name.value) {
    tagDetails.name.value = tagDetails.name.value.trim()
  }

  if (
    !tagDetails.name.value &&
    (tagDetails.name.inputstate !== 'untouched' || !requireUntouched)
  ) {
    tagDetails.name.inputstate = 'invalid'
    tagDetails.errors.push('Name is required')
  }

  if (
    !tagDetails.color.value &&
    (tagDetails.color.inputstate !== 'untouched' || !requireUntouched)
  ) {
    tagDetails.color.inputstate = 'invalid'
    tagDetails.errors.push('Color is required')
  }

  if (
    tags.find(
      tag =>
        tag.name.toLowerCase() === tagDetails.name.value?.toLowerCase() &&
        tag.id !== tagDetails.id,
    )
  ) {
    tagDetails.name.inputstate = 'invalid'
    tagDetails.errors.push('Tag name must be unique')
  }
}

const submitAddTag = async () => {
  validateTagDetails(false)

  if (
    tagDetails.name.inputstate !== 'touched' ||
    tagDetails.color.inputstate !== 'touched' ||
    !tagDetails.color.value ||
    !tagDetails.name.value
  ) {
    return
  }

  const newTag: NewTag = {
    name: tagDetails.name.value,
    color: tagDetails.color.value as Color,
    category_id: id,
  }

  if (onAddTag) {
    tagDetails.loading = true
    const res = await takeAtLeast(onAddTag(newTag))
    if (res) {
      closeTagDetails()
    } else {
      tagDetails.loading = false
      tagDetails.errors = ['Failed to add tag']
    }
  } else {
    closeTagDetails()
  }
}

const submitEditTag = async () => {
  validateTagDetails(false)

  if (
    tagDetails.name.inputstate !== 'touched' ||
    tagDetails.color.inputstate !== 'touched' ||
    !tagDetails.color.value ||
    !tagDetails.name.value ||
    !tagDetails.id
  ) {
    return
  }

  const tagIndex = tags.findIndex(tag => tag.id === tagDetails.id)
  if (tagIndex !== -1) {
    const editedTag = {
      ...tags[tagIndex],
      name: tagDetails.name.value,
      color: tagDetails.color.value as Color,
    }

    if (onEditTag) {
      tagDetails.loading = true
      const res = await takeAtLeast(onEditTag(editedTag))
      if (res) {
        closeTagDetails()
      } else {
        tagDetails.errors = ['Failed to edit tag']
      }
      tagDetails.loading = false
    } else {
      closeTagDetails()
    }
  } else {
    closeTagDetails()
  }
}

const deleteEditTag = async () => {
  if (tagDetails.id) {
    selectedTagIds = selectedTagIds.filter(id => id !== tagDetails.id)
    if (onRemoveTag) {
      tagDetails.deleteLoading = true
      const res = await takeAtLeast(onRemoveTag(tagDetails.id))
      if (res) {
        closeTagDetails()
      } else {
        tagDetails.errors = ['Failed to delete tag']
      }
      tagDetails.deleteLoading = false
    } else {
      closeTagDetails()
    }
  }
}

const validAddTag = $derived.by(() => {
  return (
    tagDetails.name.inputstate !== 'invalid' &&
    tagDetails.color.inputstate !== 'invalid'
  )
})

const onClickEditCategory = () => {
  if (onEditCategory) {
    onEditCategory()
  }
}
</script>

<div class="category">
  <div class="category-info">
    {#if (mode === 'edit' || mode === 'select' || mode === 'select-edit') && onEditCategory}
      <button onclick={onClickEditCategory} aria-label="Edit {name} category">
        <Chip>
          <Settings />
        </Chip>
      </button>
    {/if}

    <div class="category-name">
      {name}
    </div>

    {#if mode === 'select' || mode === 'edit' || mode === 'select-edit'}
      <div class="category-actions">
        {#if selectedTagIds.some(id => tags.find(tag => tag.id === id))}
          <button onclick={onclear} aria-label="Clear selected {name} tags">
            <Chip color="red">
              <X />
            </Chip>
          </button>
        {/if}

        {#if mode === 'edit' || mode === 'select-edit'}
          <button onclick={openAddTag} aria-label="Add tag">
            <Chip>
              <div class="add-tag-inner">
                <Plus />
                Add tag
              </div>
            </Chip>
          </button>

          <Modal bind:open={tagDetails.open}>
            <div class="tag-details">
              <div class="tag-details-title">
                <TagIcon />
                {tagDetails.mode === 'add' ? 'Add Tag' : 'Edit Tag'}
              </div>

              <div class="tag-details-inputs">
                <div class="form-field inline color-picker">
                  <Label>Color</Label>
                  <ColorPicker
                    bind:value={tagDetails.color.value}
                    bind:inputstate={tagDetails.color.inputstate}
                    onChange={() => {
                      validateTagDetails()
                    }} />
                </div>

                <div class="form-field inline tag-name">
                  <Input
                    name="category-tag-details-name"
                    aria-label="Tag name"
                    placeholder="Tag name"
                    fullwidth
                    live
                    required
                    bind:value={tagDetails.name.value}
                    bind:inputstate={tagDetails.name.inputstate}
                    onenter={tagDetails.mode === 'add'
                      ? submitAddTag
                      : submitEditTag}
                    onchange={() => {
                      validateTagDetails()
                    }} />
                </div>
              </div>

              {#if tagDetails.errors.length > 0}
                <Alert type="error" size="small">
                  <b>Error</b>
                  <ul class="muted">
                    {#each tagDetails.errors as error}
                      <li>{error}</li>
                    {/each}
                  </ul>
                </Alert>
              {/if}

              <div class="tag-details-actions">
                {#if tagDetails.mode === 'add'}
                  <Button
                    type="primary"
                    onclick={submitAddTag}
                    loading={tagDetails.loading}
                    disabled={!validAddTag}>
                    <Plus /> Add
                  </Button>
                {:else if tagDetails.mode === 'edit'}
                  <Button
                    type="destructive"
                    onclick={deleteEditTag}
                    disabled={tagDetails.loading}
                    loading={tagDetails.deleteLoading}>
                    <Trash />
                    Delete tag
                  </Button>

                  <Button
                    type="primary"
                    onclick={submitEditTag}
                    loading={tagDetails.loading}
                    disabled={!validAddTag || tagDetails.deleteLoading}>
                    <Save />
                    Save changes
                  </Button>
                {/if}
              </div>
            </div>
          </Modal>
        {/if}

        {#if mode === 'select-edit' || mode === 'select'}
          <button onclick={onedit} aria-label="Edit {name} category">
            <Chip outline={mode === 'select-edit'}>
              {#if mode === 'select'}
                <Pencil />
              {:else if mode === 'select-edit'}
                <PencilOff />
              {/if}
            </Chip>
          </button>
        {/if}
      </div>
    {/if}
  </div>

  <div class="category-tags">
    {#if tags.length === 0}
      <div class="category-no-tags dimmed">
        <TagIcon />
        No tags
      </div>
    {:else}
      {#each tags as tag}
        <svelte:element
          this={mode === 'view' ? 'a' : 'button'}
          href={mode === 'view' ? `/app/tag/${tag.id}` : undefined}
          class="tag-wrapper-button plain"
          onclick={() => clickTag(tag)}
          role={mode === 'select' ? undefined : 'option'}
          aria-selected={selectedTagIds.includes(tag.id)}
          aria-label={mode === 'select'
            ? selectedTagIds.includes(tag.id)
              ? `Deselect tag: ${tag.name}`
              : `Select tag: ${tag.name}`
            : tag.name}>
          <Chip
            outline={selectedTagIds.includes(tag.id)}
            color={tag.color}
            solid={selectedTagIds.includes(tag.id)}>
            {tag.name}
          </Chip>
        </svelte:element>
      {/each}
    {/if}
  </div>
</div>

<style lang="scss">
.category {
  display: flex;
  flex-direction: column;
  gap: var(--padding-xs);
  width: 100%;

  .category-no-tags {
    font-size: var(--font-size-s);
  }

  .category-info {
    display: flex;
    justify-content: space-between;
    gap: var(--padding-xs);
    align-items: center;

    .category-name {
      font-weight: 600;
      font-size: var(--font-size-l);
      overflow: hidden;
      text-overflow: ellipsis;
      flex: 1 1 auto;
      white-space: nowrap;
    }

    .category-actions {
      display: flex;
      gap: var(--padding-xs);

      flex-shrink: 0;
    }
  }

  .category-tags {
    display: flex;
    flex-wrap: wrap;
    gap: calc(5 * var(--focus-shadow-offset));
    max-width: 100%;
    overflow: hidden;

    .tag-wrapper-button {
      overflow: hidden;
      text-overflow: ellipsis;
    }
  }
}

.tag-details {
  display: flex;
  flex-direction: column;
  gap: var(--padding-s);

  .tag-details-title {
    font-weight: 600;
    font-size: var(--font-size-xl);
  }

  .tag-details-inputs {
    display: flex;
    gap: var(--padding-s);
    flex-direction: column;
    margin: var(--padding-m) 0;
  }

  .tag-details-actions {
    display: flex;
    justify-content: space-between;

    :global(:only-child) {
      margin-left: auto;
    }
  }
}
</style>

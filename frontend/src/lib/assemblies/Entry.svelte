<script lang="ts">
import type { EntryProps } from '$lib/types/assemblies/entry'
import MoodInput from '$lib/components/MoodInput.svelte'
import Textarea from '$lib/components/Textarea.svelte'
import Button from '$lib/components/Button.svelte'
import {
  FolderOpen,
  NotebookText,
  Pencil,
  PencilOff,
  Plus,
  Save,
  SaveOff,
  Signature,
  Smile,
  Trash,
} from 'lucide-svelte'
import { entryMaxLength } from '$lib/types/log'
import Alert from '$lib/components/Alert.svelte'
import Chip from '$lib/components/Chip.svelte'
import Modal from '$lib/components/Modal.svelte'
import Markdown from 'svelte-exmarkdown'
import { gfmPlugin } from 'svelte-exmarkdown/gfm'
import type { MoodValue } from '$lib/types/components/moodinput'
import type { EditTag, NewTag } from '$lib/types/log'
import type { InputState } from '$lib/types/input'
import { onMount } from 'svelte'
import { currentDate, fullDate } from '$lib/utils/log'
import { diff } from 'deep-object-diff'
import { formatNumber } from '$lib/utils/numbers'
import Categories from './Categories.svelte'
import { takeAtLeast } from '$lib/utils/takeAtLeast'
import { beforeNavigate, goto } from '$app/navigation'
import { page } from '$app/state'

let {
  id = undefined,
  date = currentDate(),
  mode = $bindable('view'),
  categories = [],
  entry = '',
  mood = undefined,
  selectedTagIds = [],

  onCreate,
  onUpdate,
  onDelete,
  onAddTag,
  onEditTag,
  onRemoveTag,
  onAddCategory,
  onEditCategory,
  onDeleteCategory,
}: EntryProps = $props()

let inputState: InputState = $state('untouched')
let loading = $state(false)

let errors = $derived.by(() => {
  if (serverError !== undefined) {
    return [serverError]
  }

  const errs: string[] = []
  if (inputState === 'untouched') {
    return errs
  }

  if (editModel.entry.length > entryMaxLength) {
    errs.push(
      `Entry text exceeds maximum length of ${formatNumber(entryMaxLength)} characters`,
    )
  }

  if (editModel.mood === undefined) {
    errs.push('Mood value is required')
  }

  return errs
})

let disableButton = $derived.by(() => {
  if (serverError !== undefined) {
    return false
  } else {
    return errors.length > 0
  }
})

let serverError = $state<string | undefined>(undefined)

let entryTextModal = $state(false)

let editModel = $state<{
  mood?: MoodValue
  entry: string
  // the editmodel categories no longer work, the edit model should only be for the entry itself
  selectedTagIds: string[]
}>({
  mood: mood,
  entry: '',
  selectedTagIds: [],
})

const resetEditModel = () => {
  inputState = 'untouched'
  editModel.mood = mood || undefined
  editModel.entry = entry || ''
  editModel.selectedTagIds = selectedTagIds || []
}

const applyEditModel = async () => {
  mood = editModel.mood
  entry = editModel.entry
  selectedTagIds = editModel.selectedTagIds
}

const startEdit = () => {
  resetEditModel()
  mode = 'edit'
}

const saveChanges = async () => {
  inputState = 'touched'
  serverError = undefined

  if (errors.length > 0) {
    return
  }

  await applyEditModel()

  if (mood !== undefined) {
    loading = true
    if (mode === 'create' && onCreate) {
      const res = await takeAtLeast(
        onCreate({
          date,
          entry,
          mood,
          selected_tags: selectedTagIds,
        }),
      )
      if (res) {
        mode = 'view'
      } else {
        serverError = 'Failed to create entry'
      }
    } else if (mode === 'edit' && id && onUpdate) {
      const res = await takeAtLeast(
        onUpdate({
          id,
          date,
          entry,
          mood,
          selected_tags: selectedTagIds,
        }),
      )
      if (res) {
        mode = 'view'
      } else {
        serverError = 'Failed to update entry'
      }
    } else {
      mode = 'view'
    }
    loading = false
  }
}

const cancelChanges = () => {
  resetEditModel()
  mode = 'view'
}

const deleteEntry = () => {
  if (id && onDelete) {
    onDelete(id)
  }
}

onMount(() => {
  resetEditModel()
})

const isEdited = $derived.by(() => {
  if (mode === 'view') {
    return false
  }

  if (mode === 'edit') {
    const difference = diff(
      {
        mood: mood,
        selectedTagIds: selectedTagIds,
        entry: entry,
      },
      {
        mood: editModel.mood,
        selectedTagIds: editModel.selectedTagIds,
        entry: editModel.entry,
      },
    )

    return Object.keys(difference).length > 0
  } else if (mode === 'create') {
    return (
      editModel.entry.length > 0 ||
      editModel.mood !== undefined ||
      (editModel.selectedTagIds && editModel.selectedTagIds.length > 0)
    )
  }

  return false
})

let navigateModal = $state<{
  open: boolean
  target: string
}>({
  open: false,
  target: '',
})

const cancelNavigation = () => {
  navigateModal.open = false
  navigateModal.target = ''
}

const confirmNavigation = () => {
  if (navigateModal.target) {
    goto(navigateModal.target)
  }
}

beforeNavigate(nav => {
  if (
    !navigateModal.open &&
    isEdited &&
    page.url.pathname.startsWith('/app') &&
    nav.to &&
    nav.to.url &&
    nav.to.url.origin === page.url.origin
  ) {
    nav.cancel()
    navigateModal.open = true
    navigateModal.target = nav.to.url.pathname
  }
})

const categoryAddTag = async (tag: NewTag) => {
  if (onAddTag) {
    return onAddTag(tag)
  }
  return null
}

const categoryEditTag = async (tag: EditTag) => {
  if (onEditTag) {
    return onEditTag(tag)
  }
  return null
}

const categoryRemoveTag = async (tagId: string) => {
  if (onRemoveTag) {
    return onRemoveTag(tagId)
  }
  return null
}
</script>

<Modal bind:open={navigateModal.open}>
  <div class="navigate-modal">
    <div class="navigate-modal-title">Unsaved changes</div>
    <div class="navigate-modal-text">
      You have unsaved changes. Are you sure you want to leave this page and
      discard your changes?
    </div>
    <div class="modal-actions">
      <Button onclick={cancelNavigation}>Continue editing</Button>
      <Button type="destructive" onclick={confirmNavigation}>
        Discard changes
      </Button>
    </div>
  </div>
</Modal>

<div class="entry">
  <div class="entry-title">
    <div class="date">
      {fullDate(date)}
    </div>

    {#if isEdited}
      <Chip>
        <SaveOff />
        Unsaved changes
      </Chip>
    {/if}
  </div>

  <div class="entry-field mood-field">
    <div class="entry-field-title">
      <div class="inner">
        <Smile />
        Mood
      </div>
    </div>
    <MoodInput
      bind:value={editModel.mood}
      mode={mode === 'view' ? 'view' : 'edit'} />
  </div>

  <div class="entry-field categories-field">
    <div class="entry-field-title">
      <div class="inner">
        <FolderOpen />
        Categories
      </div>
    </div>

    <div class="categories">
      <Categories
        {categories}
        mode={mode === 'view' ? 'view' : 'select'}
        bind:selectedTagIds={editModel.selectedTagIds}
        {onAddCategory}
        {onEditCategory}
        {onDeleteCategory}
        {categoryAddTag}
        {categoryEditTag}
        {categoryRemoveTag} />
    </div>
  </div>

  <div class="entry-field entry-field-text">
    <div class="entry-field-title">
      <div class="inner">
        <NotebookText />
        Entry
      </div>
      {#if mode === 'edit' || mode === 'create'}
        <button
          onclick={() => (entryTextModal = true)}
          aria-label="Show entry text formatting help">
          <Chip>
            <Signature />
          </Chip>
        </button>

        <Modal bind:open={entryTextModal}>
          <p>
            Entry text supports markdown formatting, it uses
            <a
              href="https://ssssota.github.io/svelte-exmarkdown/"
              target="_blank">
              svelte-exmarkdown
            </a>
            for formatting
          </p>
          <p>
            See general markdown documentation <a
              href="https://www.markdownguide.org/cheat-sheet/"
              target="_blank">
              here
            </a>
          </p>
        </Modal>
      {/if}
    </div>

    {#if mode === 'edit' || mode === 'create'}
      <div class="entry-textarea">
        <Textarea
          bind:value={editModel.entry}
          maxlength={entryMaxLength}
          placeholder="Write your thoughts here..."
          fullwidth />
      </div>
    {:else}
      <div class="entry-text">
        {#if entry}
          <div class="entry-text-content">
            <Markdown md={entry} plugins={[gfmPlugin()]} />
          </div>
        {:else}
          <p class="muted">No entry</p>
        {/if}
      </div>
    {/if}
  </div>

  {#if errors.length}
    <Alert type="error" size="small">
      <b>Invalid entry</b>
      <ul class="muted">
        {#each errors as error}
          <li>{error}</li>
        {/each}
      </ul>
    </Alert>
  {/if}

  <div class="entry-actions">
    {#if mode === 'view'}
      {#if id}
        <Button
          type="destructive"
          onclick={() => deleteEntry()}
          disabled={loading}>
          <Trash /> Delete entry
        </Button>
      {/if}

      <Button onclick={() => startEdit()}>
        <Pencil /> Edit
      </Button>
    {:else if mode === 'edit'}
      <Button onclick={() => cancelChanges()} disabled={loading}>
        <PencilOff />
        Cancel
      </Button>
      <Button
        type="primary"
        {loading}
        onclick={() => saveChanges()}
        disabled={disableButton}>
        <Save />
        Save changes
      </Button>
    {:else if mode === 'create'}
      <Button
        type="primary"
        {loading}
        onclick={() => saveChanges()}
        disabled={disableButton}>
        <Plus />
        Create Entry
      </Button>
    {/if}
  </div>
</div>

<style lang="scss">
.entry {
  display: flex;
  flex-direction: column;
  gap: var(--padding-l);
  width: 100%;

  .categories {
    padding-top: var(--padding-m);
  }

  .entry-title {
    font-weight: 600;
    font-size: var(--font-size-xl);
    display: flex;
    align-items: center;
    justify-content: space-between;

    .date {
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
    }
  }

  .entry-field {
    display: flex;
    flex-direction: column;

    &.mood-field {
      gap: var(--padding-s);
      align-items: center;
    }

    &.entry-field-text {
      .entry-text {
        :global(:last-of-type) {
          margin-bottom: 0;
        }
      }

      .entry-textarea {
        margin-top: var(--padding-s);
      }
    }

    .entry-field-title {
      font-weight: 600;
      font-size: var(--font-size-l);
      display: flex;
      justify-content: space-between;
      width: 100%;

      .inner {
        display: flex;
        align-items: center;
        gap: var(--padding-s);
      }
    }
  }

  .entry-actions {
    display: flex;
    justify-content: space-between;

    :global(:only-child) {
      margin-left: auto;
    }
  }
}

.navigate-modal {
  display: flex;
  flex-direction: column;
  gap: var(--padding-m);

  .navigate-modal-title {
    font-weight: 600;
    font-size: var(--font-size-l);
  }

  .navigate-modal-text {
    font-size: var(--font-size-s);
  }

  .modal-actions {
    display: flex;
    justify-content: space-between;
    gap: var(--padding-s);
    flex-wrap: wrap;
  }
}
</style>

<script lang="ts">
import Category from './Category.svelte'
import Button from '$lib/components/Button.svelte'
import { Plus, Save, Trash } from 'lucide-svelte'
import Modal from '$lib/components/Modal.svelte'
import Input from '$lib/components/Input.svelte'
import type { Category as CategoryType } from '$lib/types/log'
import type { InputState } from '$lib/types/input'
import Message from '$lib/components/Message.svelte'
import type { CategoriesProps } from '$lib/types/assemblies/category'
import { takeAtLeast } from '$lib/utils/takeAtLeast'

let {
  categories,
  mode,
  selectedTagIds = $bindable([]),
  categoryAddTag,
  categoryEditTag,
  categoryRemoveTag,
  onAddCategory,
  onEditCategory,
  onDeleteCategory,
}: CategoriesProps = $props()

let categoryDetails: {
  mode: 'create' | 'edit'
  open: boolean
  id?: string
  name: {
    value: string
    inputstate: InputState
  }
  errors: string[]
  loading: boolean
  deleteLoading: boolean
} = $state({
  mode: 'create',
  open: false,
  id: undefined,
  name: {
    value: '',
    inputstate: 'untouched',
  },
  errors: [],
  loading: false,
  deleteLoading: false,
})

const validateCategoryDetails = () => {
  categoryDetails.errors = []
  categoryDetails.name.value = categoryDetails.name.value.trim()

  if (!categoryDetails.name.value) {
    categoryDetails.name.inputstate = 'invalid'
    categoryDetails.errors.push('Name is required')
  } else if (
    categories.find(
      c => c.name === categoryDetails.name.value && c.id !== categoryDetails.id,
    )
  ) {
    categoryDetails.name.inputstate = 'invalid'
    categoryDetails.errors.push('Name must be unique')
  }
}

const startAddCategory = () => {
  categoryDetails.open = true
  categoryDetails.name.value = ''
  categoryDetails.name.inputstate = 'untouched'
  categoryDetails.errors = []
  categoryDetails.loading = false
  categoryDetails.deleteLoading = false
  categoryDetails.id = undefined
  categoryDetails.mode = 'create'
  categoryDetails.id = Date.now().toString()
}

const startEditCategory = (category: CategoryType) => {
  categoryDetails.open = true
  categoryDetails.errors = []
  categoryDetails.loading = false
  categoryDetails.deleteLoading = false
  categoryDetails.id = undefined
  categoryDetails.mode = 'edit'
  categoryDetails.name.inputstate = 'touched'
  categoryDetails.name.value = category.name
  categoryDetails.id = category.id
}

const closeCategoryDetails = () => {
  categoryDetails.open = false
}

const submitAddCategory = async () => {
  validateCategoryDetails()
  if (categoryDetails.name.inputstate === 'invalid') {
    return
  }

  if (onAddCategory) {
    categoryDetails.loading = true
    const res = await takeAtLeast(
      onAddCategory({
        name: categoryDetails.name.value,
      }),
    )
    if (res) {
      closeCategoryDetails()
    } else {
      categoryDetails.loading = false
      categoryDetails.errors.push('Failed to add category')
    }
  } else {
    closeCategoryDetails()
  }
}

const submitEditCategory = async () => {
  validateCategoryDetails()
  if (categoryDetails.name.inputstate === 'invalid' || !categoryDetails.id) {
    return
  }

  if (onEditCategory) {
    categoryDetails.loading = true
    const res = await takeAtLeast(
      onEditCategory({
        id: categoryDetails.id,
        name: categoryDetails.name.value,
      }),
    )
    if (res) {
      closeCategoryDetails()
    } else {
      categoryDetails.loading = false
      categoryDetails.errors.push('Failed to edit category')
    }
  } else {
    closeCategoryDetails()
  }
}

const deleteCategory = async () => {
  if (!categoryDetails.id) return

  selectedTagIds = selectedTagIds.filter(tagId => {
    return categories.some(c => c.tags.some(t => t.id === tagId))
  })
  if (onDeleteCategory) {
    categoryDetails.deleteLoading = true
    const res = await takeAtLeast(onDeleteCategory(categoryDetails.id))
    if (res) {
      closeCategoryDetails()
    } else {
      categoryDetails.deleteLoading = false
      categoryDetails.errors.push('Failed to delete category')
    }
  } else {
    closeCategoryDetails()
  }
}
</script>

<div class="categories">
  {#if categories.length === 0}
    <div class="muted">No categories</div>
  {/if}

  {#each categories as category}
    {#key category.id}
      <Category
        id={category.id}
        name={category.name}
        tags={category.tags}
        {categories}
        bind:selectedTagIds
        onEditCategory={() => startEditCategory(category)}
        onAddTag={categoryAddTag}
        onEditTag={categoryEditTag}
        onRemoveTag={categoryRemoveTag}
        {mode} />
    {/key}
  {/each}
</div>

{#if mode === 'edit' || mode === 'select'}
  <div class="add-category">
    <Button type="ghost" fullwidth onclick={startAddCategory}>
      <Plus /> Add category
    </Button>

    <Modal bind:open={categoryDetails.open}>
      <div class="category-details">
        <div class="category-details-title">
          {#if categoryDetails.mode === 'create'}
            Add category
          {:else if categoryDetails.mode === 'edit'}
            Edit category
          {/if}
        </div>

        <div class="category-details-inputs">
          <Input
            required
            bind:value={categoryDetails.name.value}
            bind:inputstate={categoryDetails.name.inputstate}
            onenter={categoryDetails.mode === 'create'
              ? submitAddCategory
              : submitEditCategory}
            onchange={validateCategoryDetails}
            placeholder="Category name"
            fullwidth />

          {#if categoryDetails.errors.length}
            {#each categoryDetails.errors as error}
              <Message size="small" type="error">
                {error}
              </Message>
            {/each}
          {/if}
        </div>

        <div class="category-details-actions">
          {#if categoryDetails.mode === 'create'}
            <div class="create">
              <Button
                fullwidth
                onclick={submitAddCategory}
                loading={categoryDetails.loading}
                disabled={categoryDetails.errors.length > 0}>
                <Plus />
                Add
              </Button>
            </div>
          {:else if categoryDetails.mode === 'edit'}
            <div class="delete">
              <Button
                fullwidth
                type="destructive"
                onclick={deleteCategory}
                loading={categoryDetails.deleteLoading}
                disabled={categoryDetails.loading}>
                <Trash />
                Delete category
              </Button>
            </div>

            <div class="save">
              <Button
                fullwidth
                onclick={submitEditCategory}
                loading={categoryDetails.loading}
                disabled={categoryDetails.deleteLoading ||
                  categoryDetails.errors.length > 0}>
                <Save />
                Save changes
              </Button>
            </div>
          {/if}
        </div>
      </div>
    </Modal>
  </div>
{/if}

<style lang="scss">
.categories {
  display: flex;
  flex-direction: column;
  gap: var(--padding-m);
  width: 100%;
}

.add-category {
  margin-top: var(--padding-m);
}

.category-details {
  display: flex;
  flex-direction: column;
  gap: var(--padding-s);

  .category-details-title {
    font-weight: 600;
    font-size: var(--font-size-xl);
  }

  .category-details-inputs {
    display: flex;
    gap: var(--padding-s);
    flex-direction: column;
    margin: var(--padding-m) 0;
  }

  .category-details-actions {
    display: flex;
    justify-content: space-between;
    gap: var(--padding-xs);
    flex-wrap: wrap;

    @media (max-width: 768px) {
      flex-direction: column;
    }

    .create {
      margin-left: auto;
    }
  }
}
</style>

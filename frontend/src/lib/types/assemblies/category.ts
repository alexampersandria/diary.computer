import type {
  Category,
  CategoryWithTags,
  EditCategory,
  EditTag,
  NewCategory,
  NewTag,
  Tag,
} from '../log'

export type CategoryProps = {
  id: string
  name: string
  tags: Tag[]
  selectedTagIds?: string[]
  mode?: 'view' | 'edit' | 'select' | 'select-edit'
  categories?: CategoryWithTags[]
  onAddTag?: (tag: NewTag) => Promise<Tag | null>
  onRemoveTag?: (id: Tag['id']) => Promise<boolean | null>
  onSelectTag?: (tag: Tag, selected: boolean) => Promise<void>
  onEditTag?: (tag: Tag) => Promise<Tag | null>
  onEditCategory?: () => void
}

export type CategoriesProps = {
  categories: CategoryWithTags[]
  mode?: 'view' | 'edit' | 'select'
  selectedTagIds?: CategoryProps['selectedTagIds']
  categoryAddTag?: (tag: NewTag) => Promise<Tag | null>
  categoryEditTag?: (tag: EditTag) => Promise<Tag | null>
  categoryRemoveTag?: (id: Tag['id']) => Promise<boolean | null>
  onAddCategory?: (category: NewCategory) => Promise<Category | null>
  onEditCategory?: (category: EditCategory) => Promise<Category | null>
  onDeleteCategory?: (categoryId: string) => Promise<boolean | null>
}

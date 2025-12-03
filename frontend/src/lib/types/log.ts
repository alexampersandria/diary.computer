import type { Color } from './color'
import type { MoodValue } from './components/moodinput'

export type Category = {
  id: string
  name: string
  user_id: string
  created_at: number
}

export type NewCategory = {
  name: string
}

export type EditCategory = {
  id: string
  name: string
}

export type CategoryWithTags = Category & {
  tags: Tag[]
  selected_tags?: string[]
}

export type Tag = {
  id: string
  user_id: string
  created_at: number
  name: string
  color: Color
  category_id: string
}

export type TagWithCategory = Tag & {
  category: Category
}

export type NewTag = {
  name: string
  color: Color
  category_id: string
}

export type EditTag = {
  id: string
  name: string
  color: Color
  category_id: string
}

export type Entry = {
  id: string
  user_id: string
  date: string
  created_at: number
  mood: MoodValue
  entry?: string
  selected_tags: string[]
}

export type NewEntry = {
  date: string
  mood: MoodValue
  entry?: string
  selected_tags: string[]
}

export type EditEntry = {
  id: string
  date: string
  mood: MoodValue
  entry?: string
  selected_tags: string[]
}

export const entryMaxLength = 1000

/**
 * see /backend/src/services/log.rs
 */
export const defaultCategories: CategoryWithTags[] = [
  {
    id: crypto.randomUUID(),
    name: 'Activities',
    user_id: '',
    created_at: 0,
    tags: [
      {
        id: crypto.randomUUID(),
        user_id: '',
        created_at: 0,
        name: 'Work',
        color: 'base',
        category_id: '',
      },
      {
        id: crypto.randomUUID(),
        user_id: '',
        created_at: 0,
        name: 'Movie',
        color: 'base',
        category_id: '',
      },
      {
        id: crypto.randomUUID(),
        user_id: '',
        created_at: 0,
        name: 'Exercise',
        color: 'base',
        category_id: '',
      },
      {
        id: crypto.randomUUID(),
        user_id: '',
        created_at: 0,
        name: 'Read',
        color: 'base',
        category_id: '',
      },
      {
        id: crypto.randomUUID(),
        user_id: '',
        created_at: 0,
        name: 'Shopping',
        color: 'base',
        category_id: '',
      },
      {
        id: crypto.randomUUID(),
        user_id: '',
        created_at: 0,
        name: 'Gaming',
        color: 'base',
        category_id: '',
      },
    ],
  },
  {
    id: crypto.randomUUID(),
    name: 'Tags',
    user_id: '',
    created_at: 0,
    tags: [
      {
        id: crypto.randomUUID(),
        user_id: '',
        created_at: 0,
        name: 'Travel',
        color: 'base',
        category_id: '',
      },
      {
        id: crypto.randomUUID(),
        user_id: '',
        created_at: 0,
        name: 'Important',
        color: 'blue',
        category_id: '',
      },
      {
        id: crypto.randomUUID(),
        user_id: '',
        created_at: 0,
        name: 'Sick',
        color: 'red',
        category_id: '',
      },
    ],
  },
]

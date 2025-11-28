import { browser } from '$app/environment'
import { page } from '$app/state'
import type {
  Category,
  CategoryWithTags,
  EditCategory,
  EditEntry,
  EditTag,
  Entry,
  NewCategory,
  NewEntry,
  NewTag,
  Tag,
  TagWithCategory,
} from '$lib/types/log'
import { getEntries, type FetchEntriesOptions } from '$lib/utils/api'
import { API_URL } from '$lib/utils/env'
import {
  calendarDefaults,
  currentDateObject,
  isValidDate,
  monthDateRange,
  sortCategories,
  sortEntries,
} from '$lib/utils/log'
import { useUserStore, type UserState } from './userStore.svelte'

let userStore: UserState | null = null

export type DataState = {
  calendarPosition: {
    year: number
    month: number
  }
  categories: CategoryWithTags[]
  entries: Entry[]
  loaded: boolean
  fetchedAt: number

  fetchCategories: () => Promise<void>
  fetchEntries: (options?: FetchEntriesOptions) => Promise<void>

  fetchEntry: (date: string) => Promise<Entry | null>

  getTag: (id: string) => TagWithCategory | null
  getTags: () => TagWithCategory[] | null
  getEntry: (date: string) => Entry | null

  createEntry: (entry: NewEntry) => Promise<Entry | null>
  updateEntry: (entry: EditEntry) => Promise<Entry | null>
  deleteEntry: (id: string) => Promise<boolean | null>

  createCategory: (category: NewCategory) => Promise<CategoryWithTags | null>
  updateCategory: (category: EditCategory) => Promise<CategoryWithTags | null>
  deleteCategory: (id: string) => Promise<boolean | null>

  createTag: (tag: NewTag) => Promise<Tag | null>
  updateTag: (tag: EditTag) => Promise<Tag | null>
  deleteTag: (id: string) => Promise<boolean | null>

  deleteData: () => void
}

let calendarPosition = $state<{ year: number; month: number }>({
  year: calendarDefaults().year,
  month: calendarDefaults().month,
})
let categories: CategoryWithTags[] = $state([])
let entries: Entry[] = $state([])
let loaded: boolean = $state(false)
let fetchedAt: number = $state(0)

const fetchCategories = async () => {
  await fetch(API_URL('/v1/user/categories'), {
    headers: { Authorization: `Bearer ${userStore?.sessionId}` },
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to fetch categories')
      }
      return res.json()
    })
    .then((data: CategoryWithTags[]) => {
      categories = data
    })
    .catch(err => {
      console.error('Error fetching user categories:', err)
    })
}

const fetchEntries = async (options?: FetchEntriesOptions) => {
  const { firstDate, lastDate } = monthDateRange()
  if (userStore?.sessionId) {
    const data = await getEntries(
      userStore.sessionId,
      // if no options provided, fetch current month
      options || {
        from_date: firstDate,
        to_date: lastDate,
      },
    )
    if (data) {
      data.data.forEach(entry => {
        if (getEntry(entry.date)) {
          // update existing entry
          entries = entries.map(e => (e.date === entry.date ? entry : e))
        } else {
          // add new entry
          entries.push(entry)
        }
      })
    }
    fetchedAt = Date.now()
  }
}

const fetchEntry = async (date: string) => {
  await getEntries(userStore?.sessionId || '', {
    from_date: date,
    to_date: date,
  }).then(data => {
    if (data && data.data.length > 0) {
      const entry = data.data[0]
      if (getEntry(entry.date)) {
        entries = entries.map(e => (e.date === entry.date ? entry : e))
      } else {
        entries.push(entry)
      }
      return entry
    } else {
      entries = entries.filter(e => e.date !== date)
    }
  })
  fetchedAt = Date.now()

  return null
}

const getTag = (id: string): TagWithCategory | null => {
  if (categories) {
    for (const category of categories) {
      const tag = category.tags.find(t => t.id === id)
      if (tag) {
        return { ...tag, category }
      }
    }
  }
  return null
}

const getTags = (): TagWithCategory[] | null => {
  if (categories) {
    const sorted = sortCategories(categories)
    let tags: TagWithCategory[] = []
    sorted.forEach(category => {
      tags = tags.concat(category.tags.map(tag => ({ ...tag, category })))
    })
    return tags
  }
  return null
}

const getEntry = (date: string): Entry | null => {
  if (entries) {
    const entry = entries.find(e => e.date === date)
    if (entry) {
      return entry
    }
  }
  return null
}

const createEntry = async (entry: NewEntry): Promise<Entry | null> => {
  return fetch(API_URL('/v1/entry'), {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${userStore?.sessionId}`,
    },
    body: JSON.stringify(entry),
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to create entry')
      }
      return res.json()
    })
    .then((data: Entry) => {
      if (entries) {
        entries.push(data)
      } else {
        entries = [data]
      }

      return data
    })
    .catch(err => {
      console.error('Error creating entry:', err)
      return null
    })
}

const updateEntry = async (entry: EditEntry): Promise<Entry | null> => {
  return fetch(API_URL(`/v1/entry/${entry.id}`), {
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${userStore?.sessionId}`,
    },
    body: JSON.stringify(entry),
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to update entry')
      }
      return res.json()
    })
    .then((data: Entry) => {
      if (entries) {
        entries = entries.map(e => (e.date === data.date ? data : e))
      } else {
        entries = [data]
      }

      return data
    })
    .catch(err => {
      console.error('Error updating entry:', err)
      // if update fails, remove the entry from the store
      entries = entries.filter(e => e.id !== entry.id)
      return null
    })
}

const deleteEntry = async (id: string): Promise<boolean | null> => {
  return fetch(API_URL(`/v1/entry/${id}`), {
    method: 'DELETE',
    headers: {
      Authorization: `Bearer ${userStore?.sessionId}`,
    },
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to delete entry')
      }

      if (res.status === 204) {
        return true
      } else {
        return false
      }
    })
    .then(data => {
      if (data && entries) {
        entries = entries.filter(e => e.id !== id)
      }

      return data
    })
    .catch(err => {
      console.error('Error deleting entry:', err)
      return null
    })
}

const createCategory = async (
  category: NewCategory,
): Promise<CategoryWithTags | null> => {
  return fetch(API_URL('/v1/category'), {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${userStore?.sessionId}`,
    },
    body: JSON.stringify(category),
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to create category')
      }
      return res.json()
    })
    .then((data: Category) => {
      if (categories) {
        categories.push({ ...data, tags: [] })
      }
      return { ...data, tags: [] }
    })
    .catch(err => {
      console.error('Error creating category:', err)
      return null
    })
}

const updateCategory = async (
  category: EditCategory,
): Promise<CategoryWithTags | null> => {
  return fetch(API_URL(`/v1/category/${category.id}`), {
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${userStore?.sessionId}`,
    },
    body: JSON.stringify({ name: category.name }),
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to update category')
      }
      return res.json()
    })
    .then((data: Category) => {
      if (categories) {
        categories = categories.map(c => {
          if (c.id === data.id) {
            return { ...c, name: data.name }
          }
          return c
        })

        return categories.find(c => c.id === data.id) || null
      } else {
        return null
      }
    })
    .catch(err => {
      console.error('Error updating category:', err)
      return null
    })
}

const deleteCategory = async (id: string): Promise<boolean | null> => {
  return fetch(API_URL(`/v1/category/${id}`), {
    method: 'DELETE',
    headers: {
      Authorization: `Bearer ${userStore?.sessionId}`,
    },
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to delete category')
      }

      if (res.status === 204) {
        return true
      } else {
        return false
      }
    })
    .then(data => {
      if (data && categories) {
        categories = categories.filter(c => c.id !== id)
      }
      return data
    })
    .catch(err => {
      console.error('Error deleting category:', err)
      return null
    })
}

const createTag = async (tag: NewTag): Promise<Tag | null> => {
  return fetch(API_URL('/v1/tag'), {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${userStore?.sessionId}`,
    },
    body: JSON.stringify(tag),
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to create tag')
      }
      return res.json()
    })
    .then((data: Tag) => {
      if (categories) {
        const tagCategory = categories.find(c => c.id === data.category_id)
        if (tagCategory) {
          tagCategory.tags.push(data)
        }
      }
      return data
    })
    .catch(err => {
      console.error('Error creating tag:', err)
      return null
    })
}

const updateTag = async (tag: EditTag): Promise<Tag | null> => {
  return fetch(API_URL(`/v1/tag/${tag.id}`), {
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${userStore?.sessionId}`,
    },
    body: JSON.stringify({
      name: tag.name,
      color: tag.color,
    }),
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to update tag')
      }
      return res.json()
    })
    .then((data: Tag) => {
      if (categories) {
        const tagCategory = categories.find(c => c.id === data.category_id)
        if (tagCategory) {
          tagCategory.tags = tagCategory.tags.map(t =>
            t.id === data.id ? data : t,
          )
        }
      }

      return data
    })
    .catch(err => {
      console.error('Error updating tag:', err)
      return null
    })
}

const deleteTag = async (id: string): Promise<boolean | null> => {
  return fetch(API_URL(`/v1/tag/${id}`), {
    method: 'DELETE',
    headers: {
      Authorization: `Bearer ${userStore?.sessionId}`,
    },
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to delete tag')
      }

      if (res.status === 204) {
        return true
      } else {
        return false
      }
    })
    .then(data => {
      if (categories) {
        categories.forEach(c => {
          c.tags = c.tags.filter(t => t.id !== id)
        })
      }
      return data
    })
    .catch(err => {
      console.error('Error deleting tag:', err)
      return null
    })
}

const deleteData = () => {
  calendarPosition = {
    year: calendarDefaults().year,
    month: calendarDefaults().month,
  }
  categories = []
  entries = []
}

export const useDataStore: () => DataState = () => {
  $effect(() => {
    if (!userStore) {
      userStore = useUserStore()
    }
  })

  $effect(() => {
    if (userStore && userStore.sessionId && !loaded) {
      fetchCategories()
      // loaded true means this will only be called once per "session"
      loaded = true

      setInterval(() => {
        cleanupEntries()
        // check every minute if cleanup is needed
      }, 60_000)
    }
  })

  return {
    get categories() {
      // this sorting should be done server-side ideally
      // or at least moved to whenever categories are set/updated
      // but this works for now and doesn't seem to have massive
      // performance issues because categories are usually few
      // at leasst compared to entries
      if (categories) {
        return sortCategories(categories)
      }
      return []
    },
    get entries() {
      // remove sort because of performance issues
      // also entries are sorted on the backend
      // so this is only an issue when entries are added
      // as it will be added as a single entry unsorted
      // not sorted by the /entries api call
      // if (entries) {
      //   return sortEntries(entries)
      // }
      // return []
      return entries
    },
    get calendarPosition() {
      return calendarPosition
    },
    set calendarPosition(value) {
      calendarPosition = value
    },
    set categories(value) {
      categories = value
    },
    set entries(value) {
      entries = value
    },
    get loaded() {
      return loaded
    },
    get fetchedAt() {
      return fetchedAt
    },
    set fetchedAt(value) {
      fetchedAt = value
    },

    get fetchCategories() {
      return fetchCategories
    },
    get fetchEntries() {
      return fetchEntries
    },

    get fetchEntry() {
      return fetchEntry
    },

    get getTag() {
      return getTag
    },
    get getTags() {
      return getTags
    },
    get getEntry() {
      return getEntry
    },

    get createEntry() {
      return createEntry
    },
    get updateEntry() {
      return updateEntry
    },
    get deleteEntry() {
      return deleteEntry
    },

    get createCategory() {
      return createCategory
    },
    get updateCategory() {
      return updateCategory
    },
    get deleteCategory() {
      return deleteCategory
    },

    get createTag() {
      return createTag
    },
    get updateTag() {
      return updateTag
    },
    get deleteTag() {
      return deleteTag
    },

    get deleteData() {
      return deleteData
    },
  }
}

// periodically clean up entries that are not needed in memory
// can probably be done better but this works so 🤷‍♀️
let lastCleanup = $state(0)
let cleanupInProgress = $state(false)
const entriesMaxSize = 256_000
const entriesTargetSize = 128_000
const cleanupInterval = 60_000 * 15

const cleanupEntries = async () => {
  if (browser) {
    if (cleanupInProgress) {
      return
    }

    const timeSinceLastCleanup = Date.now() - lastCleanup
    const timeSinceFetchedAt = Date.now() - fetchedAt
    if (
      timeSinceLastCleanup < cleanupInterval ||
      timeSinceFetchedAt < cleanupInterval
    ) {
      return
    }

    let entriesSize = JSON.stringify(entries).length
    if (entriesSize < entriesMaxSize) {
      return
    }

    cleanupInProgress = true

    let monthsToPersist: { year: number; month: number }[] = []

    monthsToPersist.push({
      year: currentDateObject().year,
      month: currentDateObject().month,
    })

    monthsToPersist.push({
      year: calendarPosition.year,
      month: calendarPosition.month,
    })

    const route = page.url.pathname
    if (route.startsWith('/app/entry/')) {
      const dateStr = route.replace('/app/entry/', '').replaceAll('/', '')

      if (isValidDate(dateStr)) {
        const dateObj = new Date(dateStr)
        const year = dateObj.getFullYear()
        const month = dateObj.getMonth()
        monthsToPersist.push({ year, month })
      }
    }

    monthsToPersist.forEach(({ year, month }) => {
      // add next and previous month as well
      const prevMonth = month === 0 ? 11 : month - 1
      const prevYear = month === 0 ? year - 1 : year
      const nextMonth = month === 11 ? 0 : month + 1
      const nextYear = month === 11 ? year + 1 : year

      monthsToPersist.push({ year: prevYear, month: prevMonth })
      monthsToPersist.push({ year: nextYear, month: nextMonth })
    })
    // remove duplicates
    monthsToPersist = monthsToPersist.filter(
      (month, index, self) =>
        index ===
        self.findIndex(m => m.year === month.year && m.month === month.month),
    )

    // sort entries by date ascending so we remove oldest first
    // this creates a copy so we don't mutate the original array while iterating
    let entriesCopySorted = sortEntries(entries)
    for (const entry of entriesCopySorted) {
      setTimeout(() => {
        entriesSize = JSON.stringify(entriesCopySorted).length

        if (entriesSize < entriesTargetSize) {
          return
        }

        const entryDate = new Date(entry.date)
        const entryYear = entryDate.getFullYear()
        const entryMonth = entryDate.getMonth()

        const shouldPersist = monthsToPersist.some(
          m => m.year === entryYear && m.month === entryMonth,
        )

        if (!shouldPersist) {
          // remove entry
          entriesCopySorted = entriesCopySorted.filter(
            e => e.date !== entry.date,
          )
        }
      }, 128)

      // write the cleaned up entries back to the store
      entries = entriesCopySorted

      lastCleanup = Date.now()
      cleanupInProgress = false
    }
  }
}

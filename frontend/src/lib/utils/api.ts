import type { MoodStats, TagStats, WeekdayStats } from '$lib/types/api/stats'
import type { Entry } from '$lib/types/log'
import type { Paginated } from '$lib/types/paginated'
import type { Session } from '$lib/types/user'
import { API_URL } from './env'

export type FetchEntriesOptions = {
  from_date?: string
  to_date?: string
  tags?: string[]
  from_mood?: number
  to_mood?: number
  order?: 'date_asc' | 'date_desc'
  limit?: number
  offset?: number
}

export const getEntries = async (
  sessionId: string,
  options?: FetchEntriesOptions,
): Promise<Paginated<Entry> | void> => {
  const params = new URLSearchParams()
  if (options?.from_date) {
    params.append('from_date', options.from_date)
  }
  if (options?.to_date) {
    params.append('to_date', options.to_date)
  }
  if (options?.tags) {
    const tagString = options.tags.join(',')
    if (tagString) {
      params.append('tags', tagString)
    }
  }
  if (options?.from_mood !== undefined) {
    params.append('from_mood', `${options.from_mood}`)
  }
  if (options?.to_mood !== undefined) {
    params.append('to_mood', `${options.to_mood}`)
  }
  if (options?.order) {
    params.append('order', options.order)
  }
  if (options?.limit !== undefined) {
    params.append('limit', `${options.limit}`)
  }
  if (options?.offset !== undefined) {
    params.append('offset', `${options.offset}`)
  }
  const url = new URL(API_URL('/v1/entries'))
  url.search = params.toString()

  return fetch(url, {
    headers: { Authorization: `Bearer ${sessionId}` },
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to fetch entries')
      }
      return res.json()
    })
    .then((data: Paginated<Entry>) => {
      return data
    })
    .catch(err => {
      console.error('Error fetching user details:', err)
    })
}

export const getSessions = (sessionId: string) => {
  return fetch(API_URL('/v1/sessions'), {
    headers: { Authorization: `Bearer ${sessionId}` },
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to fetch sessions')
      }
      return res.json()
    })
    .then((data: Session[]) => {
      return data
    })
    .catch(err => {
      console.error('Error fetching sessions:', err)
    })
}

export const updatePassword = (sessionId: string, newPassword: string) => {
  return fetch(API_URL('/v1/user/password'), {
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${sessionId}`,
    },
    body: JSON.stringify({ password: newPassword }),
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to update password')
      }
      if (res.status === 204) {
        return true
      }
      return false
    })
    .catch(err => {
      console.error('Error updating password:', err)
      return false
    })
}

export const getMoodStats = async (sessionId: string) => {
  const url = new URL(API_URL('/v1/stats/mood'))

  return fetch(url, {
    headers: { Authorization: `Bearer ${sessionId}` },
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to fetch mood stats')
      }
      return res.json()
    })
    .then((data: MoodStats) => {
      return data
    })
    .catch(err => {
      console.error('Error fetching mood stats:', err)
    })
}

export const getTagStats = async (sessionId: string) => {
  const url = new URL(API_URL('/v1/stats/tags'))

  return fetch(url, {
    headers: { Authorization: `Bearer ${sessionId}` },
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to fetch tag stats')
      }
      return res.json()
    })
    .then((data: TagStats) => {
      return data
    })
    .catch(err => {
      console.error('Error fetching tag stats:', err)
    })
}

export const getWeekdayStats = async (sessionId: string) => {
  const url = new URL(API_URL('/v1/stats/weekday'))

  return fetch(url, {
    headers: { Authorization: `Bearer ${sessionId}` },
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to fetch weekday stats')
      }
      return res.json()
    })
    .then((data: WeekdayStats) => {
      return data
    })
    .catch(err => {
      console.error('Error fetching weekday stats:', err)
    })
}

import type { EditUserDetails, UserDetails } from '$lib/types/user'
import { goto } from '$app/navigation'
import { useDataStore, type DataState } from './dataStore.svelte'
import { API_URL } from '$lib/utils/env'
import { watch } from 'runed'

let dataStore: DataState | null = null

export type UserState = {
  sessionId: string | null
  userDetails: UserDetails | null
  logOut: () => void
  logIn: (sessionId: string) => void
  updateUserDetails: (
    details: Partial<UserDetails>,
  ) => Promise<UserDetails | null>
  deleteAccount: () => Promise<boolean>
  revokeSession: (sessionId: string) => Promise<boolean>
}

let sessionId: string | null = $state(null)
let userDetails: UserDetails | null = $state(null)

const logOut = () => {
  sessionId = null
  userDetails = null
  if (dataStore) {
    dataStore.deleteData()
  }
  goto('/')
}

const logIn = (newSessionId: string) => {
  sessionId = newSessionId
  if (dataStore) {
    dataStore.fetchCategories()
  }
  goto('/app')
}

const updateUserDetails = async (details: Partial<UserDetails>) => {
  if (userDetails) {
    const updatedDetails = { ...userDetails, ...details }
    const body: Partial<EditUserDetails> = {
      name: updatedDetails.name,
      email: updatedDetails.email,
    }
    const res = await fetch(API_URL('/v1/user'), {
      method: 'PATCH',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${sessionId}`,
      },
      body: JSON.stringify(body),
    })
      .then(res => {
        if (!res.ok) {
          throw new Error('Failed to update user details')
        }
        if (res.status === 204) {
          return updatedDetails
        }
        return res.json()
      })
      .then((data: UserDetails) => {
        userDetails = data
        return userDetails
      })
      .catch(err => {
        console.error('Error updating user details:', err)
        return null
      })
    return res
  }
  return null
}

const deleteAccount = async (): Promise<boolean> => {
  if (userDetails) {
    const res = await fetch(API_URL('/v1/user'), {
      method: 'DELETE',
      headers: {
        Authorization: `Bearer ${sessionId}`,
      },
    })
      .then(res => {
        if (!res.ok) {
          throw new Error('Failed to delete user account')
        }
        if (res.status === 204) {
          return true
        }
        throw new Error('Unexpected response status')
      })
      .catch(err => {
        console.error('Error deleting user account:', err)
        return false
      })
    if (res) {
      logOut()
    }
    return res
  }
  return false
}

let fetchingUserDetails = false
const getUserDetails = async () => {
  if (sessionId && !fetchingUserDetails) {
    fetchingUserDetails = true
    await fetch(API_URL('/v1/user'), {
      headers: { Authorization: `Bearer ${sessionId}` },
    })
      .then(res => {
        if (!res.ok) {
          throw new Error('Failed to fetch user details')
        }
        return res.json()
      })
      .then((data: UserDetails) => {
        userDetails = data
      })
      .catch(err => {
        console.error('Error fetching user details:', err)
      })
    fetchingUserDetails = false
  }
}

const revokeSession = async (revokeSessionId: string): Promise<boolean> => {
  if (sessionId) {
    const res = await fetch(API_URL(`/v1/sessions/${revokeSessionId}`), {
      method: 'DELETE',
      headers: {
        Authorization: `Bearer ${sessionId}`,
      },
    })
      .then(res => {
        if (!res.ok) {
          throw new Error('Failed to revoke session')
        }
        if (res.status === 204) {
          return true
        }
        throw new Error('Unexpected response status')
      })
      .catch(err => {
        console.error('Error revoking session:', err)
        return false
      })
    return res
  }
  return false
}

export const useUserStore: () => UserState = () => {
  $effect(() => {
    if (!dataStore) {
      dataStore = useDataStore()
    }
  })

  watch(
    () => sessionId,
    (to, from) => {
      if (to !== from) {
        getUserDetails()
      }
    },
  )

  return {
    get sessionId() {
      return sessionId
    },
    set sessionId(value) {
      sessionId = value
    },
    get userDetails() {
      return userDetails
    },
    set userDetails(value) {
      userDetails = value
    },

    get logOut() {
      return logOut
    },
    get logIn() {
      return logIn
    },
    get updateUserDetails() {
      return updateUserDetails
    },
    get deleteAccount() {
      return deleteAccount
    },
    get revokeSession() {
      return revokeSession
    },
  }
}

<script lang="ts">
import Input from '$lib/components/Input.svelte'
import Label from '$lib/components/Label.svelte'
import Spinner from '$lib/components/Spinner.svelte'
import { useUserStore } from '$lib/store/userStore.svelte'
import { type UserDetails, type Session } from '$lib/types/user'
import { getSessions, updatePassword } from '$lib/utils/api'
import { takeAtLeast } from '$lib/utils/takeAtLeast'
import {
  Pencil,
  PencilOff,
  Save,
  Trash,
  TriangleAlert,
  UserCog,
  X,
} from 'lucide-svelte'
import { onMount } from 'svelte'
import Button from '$lib/components/Button.svelte'
import Message from '$lib/components/Message.svelte'
import { diff } from 'deep-object-diff'
import { timestampToDate } from '$lib/utils/log'
import EmailInput from '$lib/assemblies/EmailInput.svelte'
import type { InputState } from '$lib/types/input'
import Modal from '$lib/components/Modal.svelte'
import PasswordInput from '$lib/assemblies/PasswordInput.svelte'
import Alert from '$lib/components/Alert.svelte'

// temporary feature flag
// #TODO: remove when sessions management is implemented
let sessionsEnabled = false

let userStore = useUserStore()

let sessions: Session[] | null = $state(null)

let editUser = $state(false)
let editModel = $state<UserDetails | undefined>(undefined)
let editInputState = $state<{
  name: InputState
  email: InputState
}>({
  name: 'untouched',
  email: 'untouched',
})
let editLoading = $state(false)
let editError = $state<string | null>(null)

let deleteModal = $state(false)
let deleteEmail = $state('')

const startEdit = () => {
  if (userStore.userDetails) {
    editModel = { ...userStore.userDetails }
    editUser = true
    editError = null
    editLoading = false
  }
}

const getData = async () => {
  if (userStore.sessionId) {
    if (sessionsEnabled) {
      sessions = (await takeAtLeast(getSessions(userStore.sessionId))) || null
    }
  }
}

let isActive = (session: Session): boolean => {
  return session.id === userStore.sessionId
}

let isValid = $derived.by(() => {
  return (
    editModel !== undefined &&
    editInputState.name !== 'invalid' &&
    editInputState.email !== 'invalid' &&
    editError === null
  )
})

const deleteValid = $derived.by(() => {
  return deleteEmail === userStore.userDetails?.email
})

const changed = $derived.by(() => {
  if (editModel && userStore.userDetails) {
    const objDiff = diff(userStore.userDetails, editModel)
    return Object.keys(objDiff).length > 0
  }
  return false
})

const saveChanges = async () => {
  if (editModel && userStore.userDetails) {
    if (!changed) {
      editUser = false
      return
    }

    editLoading = true
    const res = await takeAtLeast(userStore.updateUserDetails(editModel), 500)
    editLoading = false
    if (res) {
      editUser = false
    } else {
      editError = 'An error occurred'
    }
  }
}

const confirmDelete = async () => {
  if (deleteValid) {
    await takeAtLeast(userStore.deleteAccount(), 500)
  }
}

let changePassword: {
  value: string
  inputstate: InputState
  loading: boolean
  changed: boolean
  error?: string
} = $state({
  value: '',
  inputstate: 'untouched',
  loading: false,
  changed: false,
  error: undefined,
})

const submitChangePassword = async () => {
  if (userStore.sessionId && changePassword.inputstate === 'touched') {
    changePassword.loading = true
    changePassword.error = undefined
    const res = await takeAtLeast(
      updatePassword(userStore.sessionId, changePassword.value),
    )
    if (res) {
      changePassword.value = ''
      changePassword.inputstate = 'untouched'
      changePassword.changed = true
      setTimeout(() => {
        changePassword.changed = false
      }, 5000)
    } else {
      changePassword.error = 'Failed to change password'
    }
    changePassword.loading = false
  }
}

onMount(async () => {
  getData()
})
</script>

<div class="app-page user-page">
  <div class="container">
    <div class="app-page-title">
      <UserCog />
      Manage account
    </div>

    {#if userStore.userDetails}
      <div class="section details" class:editing={editUser}>
        {#if !editUser}
          <div class="text">
            <div class="text-section section-title">
              {userStore.userDetails.name}
            </div>

            <div class="text-section small muted">
              {userStore.userDetails.email}
            </div>

            <div class="text-section small muted">
              Member since {timestampToDate(userStore.userDetails.created_at)}
            </div>
          </div>

          <div class="edit">
            <Button onclick={startEdit} disabled={editLoading}>
              <Pencil />
            </Button>
          </div>
        {:else if editModel}
          <div class="detail-item display-name">
            <Label>Display name</Label>
            <Input
              required
              bind:inputstate={editInputState.name}
              bind:value={editModel.name} />
          </div>
          <div class="detail-item email">
            <Label>Email</Label>
            <EmailInput
              required
              bind:inputstate={editInputState.email}
              bind:value={editModel.email} />
          </div>

          {#if editError}
            <Message type="error" size="small">
              {editError}
            </Message>
          {/if}

          <div class="actions">
            <Button onclick={() => (editUser = false)} disabled={editLoading}>
              <PencilOff /> Cancel
            </Button>

            <Button
              type="primary"
              onclick={saveChanges}
              loading={editLoading}
              disabled={!isValid}>
              <Save /> Save Changes
            </Button>
          </div>
        {/if}
      </div>
    {/if}

    <div class="section password">
      <div class="section-title">Change password</div>
      <div class="inputs">
        <div class="password-input">
          <PasswordInput
            bind:value={changePassword.value}
            bind:inputstate={changePassword.inputstate} />
        </div>
        <div class="change-button">
          <Button
            type="primary"
            fullwidth
            onclick={submitChangePassword}
            loading={changePassword.loading}
            disabled={changePassword.inputstate !== 'touched'}>
            <Save /> Change Password
          </Button>
        </div>
      </div>

      {#if changePassword.changed}
        <div class="center">
          <Alert type="success" size="small">
            Password successfully changed
          </Alert>
        </div>
      {/if}

      {#if changePassword.error}
        <div class="center">
          <Alert type="error" size="small">
            {changePassword.error}
          </Alert>
        </div>
      {/if}
    </div>

    {#if sessionsEnabled}
      <div class="section sessions">
        <div class="section-title">Active Sessions</div>
        {#if sessions}
          <table>
            <thead>
              <tr>
                <th>Session ID</th>
                <th>Created At</th>
                <th>Accessed At</th>
                <th>IP Address</th>
                <th>User Agent</th>
              </tr>
            </thead>
            <tbody>
              {#each sessions as session}
                <tr class:active={isActive(session)}>
                  <td>{session.id}</td>
                  <td>{new Date(session.created_at).toLocaleString()}</td>
                  <td>{new Date(session.accessed_at).toLocaleString()}</td>
                  <td>{session.ip_address}</td>
                  <td>{session.user_agent}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        {:else}
          <div class="loading">
            <Spinner />
          </div>
        {/if}
      </div>
    {/if}

    <div class="section delete">
      <div class="section-title">Delete account</div>
      <div class="delete-button">
        <Button type="destructive" onclick={() => (deleteModal = true)}>
          <Trash />
          Delete account
        </Button>
      </div>

      <div class="muted small">
        <TriangleAlert />
        This action is irreversible, all of your data will be permanently deleted
        with no way to recover it
      </div>
    </div>
  </div>
</div>

<Modal bind:open={deleteModal}>
  <div class="delete-modal">
    <div class="delete-modal-title">
      <Trash />
      Delete Account
    </div>

    <div class="confirm-email">
      <div class="muted small">
        <TriangleAlert />
        This action is irreversible <br />
        To confirm the deletion of your account please enter your email
      </div>
      <EmailInput bind:value={deleteEmail} />
    </div>

    <div class="delete-actions">
      <Button type="secondary" onclick={() => (deleteModal = false)}>
        <X />
        Cancel
      </Button>
      <Button
        type="destructive"
        disabled={!deleteValid}
        onclick={confirmDelete}>
        <Trash />
        Delete account
      </Button>
    </div>
  </div>
</Modal>

<style lang="scss">
.user-page {
  .section {
    margin-bottom: var(--padding-l);

    .section-title {
      font-size: var(--font-size-m);
      font-weight: 600;
    }

    &.details {
      display: flex;
      flex-direction: column;
      position: relative;

      .section-title {
        margin-bottom: var(--padding-xs);
      }

      &.editing {
        gap: var(--padding-xs);
      }

      &:not(.editing) {
        flex-direction: row;
        align-items: flex-start;
        justify-content: space-between;
        overflow: hidden;

        .text {
          overflow: hidden;

          .text-section {
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
          }
        }
      }

      .detail-item {
        display: flex;
        flex-direction: column;
        gap: var(--padding-xxs);
      }

      .actions {
        display: flex;
        align-items: center;
        justify-content: space-between;
        margin-top: var(--padding-xs);
      }
    }

    &.password {
      display: flex;
      flex-direction: column;
      gap: var(--padding-s);

      .inputs {
        display: flex;
        gap: var(--padding-s);
        flex-wrap: wrap;

        .password-input {
          flex: 1 0 auto;
        }

        @media screen and (max-width: 768px) {
          flex-direction: column;

          .change-button,
          .password-input {
            flex: 1 0 100%;
            width: 100%;
          }
        }
      }
    }

    &.sessions {
      .section-title {
        margin-bottom: var(--padding-s);
      }

      .loading {
        display: flex;
        align-items: center;
        justify-content: center;
        padding: var(--padding-l);
      }
    }

    &.delete {
      display: flex;
      flex-direction: column;
      gap: var(--padding-xs);
    }

    table {
      width: 100%;
      border-collapse: collapse;

      th,
      td {
        text-align: left;
        padding: var(--padding-xs);
        border: 1px solid var(--border-color);
      }

      tr.active {
        background-color: var(--color-success-background);
      }
    }
  }
}

.delete-modal {
  display: flex;
  flex-direction: column;
  gap: var(--padding-m);

  .confirm-email {
    display: flex;
    flex-direction: column;
    gap: var(--padding-xs);
  }

  .delete-actions {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
}
</style>

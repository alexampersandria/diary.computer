<script lang="ts">
import EmailInput from './EmailInput.svelte'
import PasswordInput from './PasswordInput.svelte'
import Input from '$lib/components/Input.svelte'
import Button from '$lib/components/Button.svelte'
import type { AuthModel, AuthProps } from '$lib/types/assemblies/auth'
import Message from '$lib/components/Message.svelte'
import { onMount } from 'svelte'
import { env } from '$env/dynamic/public'
import Alert from '$lib/components/Alert.svelte'
import type { ServerError } from '$lib/types/error'
import Checkbox from '$lib/components/Checkbox.svelte'
import Label from '$lib/components/Label.svelte'
import { useUserStore } from '$lib/store/userStore.svelte'

let userStore = useUserStore()

let { mode = $bindable('login') }: AuthProps = $props()

let inviteRequired = $state(false)
let loading = $state(false)

let serverError: ServerError | undefined = $state()
let errorMessage: string | undefined = $state()

let model: AuthModel = $state({
  name: {
    value: '',
    inputstate: 'untouched',
  },
  email: {
    value: '',
    inputstate: 'untouched',
  },
  password: {
    value: '',
    inputstate: 'untouched',
  },
  inviteCode: {
    value: '',
    inputstate: 'untouched',
  },
  terms: {
    value: false,
    inputstate: 'untouched',
  },
})

const isValid = $derived.by(() => {
  return (
    (mode === 'login' || model.name.inputstate === 'touched') &&
    model.email.inputstate === 'touched' &&
    model.password.inputstate === 'touched' &&
    (mode === 'login' ||
      model.inviteCode.inputstate === 'touched' ||
      !inviteRequired) &&
    (mode === 'login' || model.terms.value)
  )
})

let disabled = $state(false)

$effect(() => {
  // React to model changes
  if (model && isValid) {
    disabled = false
  }
})

const switchMode = () => {
  mode = mode === 'login' ? 'register' : 'login'
}

const checkValidity = () => {
  if (!isValid) {
    disabled = true

    if (model.name.inputstate === 'untouched') {
      model.name.inputstate = 'invalid'
    }
    if (model.email.inputstate === 'untouched') {
      model.email.inputstate = 'invalid'
    }
    if (model.password.inputstate === 'untouched') {
      model.password.inputstate = 'invalid'
    }
    if (model.inviteCode.inputstate === 'untouched') {
      model.inviteCode.inputstate = 'invalid'
    }
    if (!model.terms.value) {
      model.terms.inputstate = 'invalid'
    }

    return
  }
}

const submit = async () => {
  checkValidity()
  if (disabled || loading || userStore.sessionId) return

  loading = true
  errorMessage = undefined
  serverError = undefined

  if (mode === 'login') {
    await fetch(env.PUBLIC_VITE_API_URL + '/v1/auth/', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        email: model.email.value,
        password: model.password.value,
      }),
    })
      .then(async res => {
        if (!res.ok) {
          const errorData = await res.json()
          errorMessage = errorData.message || 'Failed to log in'
          throw new Error('Failed to log in')
        }
        return await res.json()
      })
      .then(data => {
        userStore.logIn(data.id)
      })
      .catch(err => {
        console.error('Login error:', err)
        serverError = 'POST'
        loading = false
      })
  } else {
    await fetch(env.PUBLIC_VITE_API_URL + '/v1/user/', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        name: model.name.value,
        email: model.email.value,
        password: model.password.value,
        invite_code: inviteRequired ? model.inviteCode.value : undefined,
      }),
    })
      .then(async res => {
        if (!res.ok) {
          throw new Error('Failed to register')
        }
        return await res.json()
      })
      .then(data => {
        userStore.logIn(data.id)
      })
      .catch(err => {
        console.error('Registration error:', err)
        serverError = 'POST'
        loading = false
      })
  }
}

onMount(async () => {
  await fetch(env.PUBLIC_VITE_API_URL + '/v1/auth/config')
    .then(res => res.json())
    .then(data => {
      inviteRequired = data.invite_required
    })
    .catch(err => {
      console.error('Failed to fetch auth config:', err)
      serverError = 'FETCH_CONFIG'
    })
})
</script>

<div class="auth-assembly">
  <div class="title">
    {#if mode === 'login'}
      Log in
    {:else}
      Sign up
    {/if}
  </div>

  {#if serverError === 'POST'}
    <Alert type="error" size="small" solid>
      {#if errorMessage}
        {errorMessage}
      {:else}
        Failed to contact the server, please try again later or <a
          href="https://github.com/alexampersandria/ephemeride/issues/new"
          target="_blank">create an issue</a> if the problem persists.
      {/if}
    </Alert>
  {/if}

  {#if mode === 'register'}
    <div class="form-field">
      <Input
        bind:value={model.name.value}
        bind:inputstate={model.name.inputstate}
        required
        placeholder="Display Name" />
    </div>
  {/if}
  <EmailInput
    bind:value={model.email.value}
    bind:inputstate={model.email.inputstate} />
  <PasswordInput
    bind:value={model.password.value}
    bind:inputstate={model.password.inputstate} />
  {#if mode === 'register' && inviteRequired}
    <div class="form-field">
      <Input
        bind:value={model.inviteCode.value}
        bind:inputstate={model.inviteCode.inputstate}
        required
        placeholder="Invite Code" />
    </div>
  {/if}

  {#if mode === 'register'}
    <div class="terms-field">
      <div class="box">
        <Checkbox
          id="register-terms"
          bind:value={model.terms.value}
          bind:inputstate={model.terms.inputstate} />
      </div>
      <div class="details">
        <Label for="register-terms" size="small">
          Agree to the terms of use
        </Label>
        <div class="explainer extra-small muted">
          By ticking this box you agree to our
          <a href="/terms-of-use">terms of use</a>
        </div>
      </div>
    </div>
  {/if}

  <Button fullwidth type="primary" {disabled} {loading} onclick={submit}>
    {#if mode === 'login'}
      Log in
    {:else}
      Sign up
    {/if}
  </Button>

  {#if disabled}
    <Message size="small" type="error">
      Please fill in all required fields.
    </Message>
  {/if}

  <p class="small muted">
    {#if mode === 'login'}
      Don't have an account?
      <button class="link" onclick={switchMode}>Sign up</button>
    {:else}
      Already have an account?
      <button class="link" onclick={switchMode}>Log in</button>
    {/if}
  </p>
</div>

<style lang="scss">
.auth-assembly {
  display: flex;
  flex-direction: column;
  gap: var(--form-gap);
  width: 100%;
  max-width: 24rem;
  margin: 0 auto;
  justify-content: center;
  align-items: center;

  .title {
    font-size: var(--font-size-xl);
    font-weight: 600;
  }

  .terms-field {
    display: flex;
    gap: var(--padding-s);
    width: 100%;
  }
}
</style>

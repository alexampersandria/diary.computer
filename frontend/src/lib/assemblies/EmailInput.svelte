<script lang="ts">
import Message from '$lib/components/Message.svelte'
import Input from '$lib/components/Input.svelte'
import type { EmailInputProps } from '$lib/types/assemblies/input'
import { validateEmail } from '$lib/utils/validationRules'

let {
  value = $bindable(),
  disabled,
  inputstate = $bindable('untouched'),
  fullwidth,
  live,
  name,
  id,
  required,
  onenter,
}: EmailInputProps = $props()

let errors: string[] = $state([])
</script>

<div class="form-field">
  <Input
    type="email"
    validation={value => {
      return validateEmail(value, err => {
        errors = err
      })
    }}
    bind:value
    {disabled}
    bind:inputstate
    {name}
    {id}
    {required}
    {fullwidth}
    {live}
    {onenter}
    placeholder="Email" />

  {#each errors as error}
    <Message size="small" type="error">{error}</Message>
  {/each}
</div>

<script lang="ts">
import type { InputProps } from '$lib/types/components/input'
import { evaluateInputState } from '$lib/utils/input'

let {
  type = 'text',
  value = $bindable(),
  placeholder,
  disabled,
  inputstate = $bindable('untouched'),
  fullwidth,
  live,
  validation,
  name,
  id,
  required,
  onchange: emitOnChange,
  oninput: emitOnInput,
  onenter: emitOnEnter,
  'aria-label': ariaLabel,
}: InputProps = $props()

const onchange = (event: Event, fromLive = false) => {
  const target = event.target as HTMLSelectElement
  value = target.value

  inputstate = evaluateInputState({
    value,
    inputstate,
    validation,
    required,
  })

  if (emitOnChange && !fromLive) {
    emitOnChange(event)
  }
}

const oninput = (event: Event) => {
  if (live) {
    onchange(event, true)
  }

  if (emitOnInput) {
    emitOnInput(event)
  }
}

const onkeydown = (event: KeyboardEvent) => {
  if (event.key === 'Enter' && emitOnEnter) {
    emitOnEnter(event)
  }
}
</script>

<input
  class="input"
  {type}
  {value}
  {placeholder}
  {disabled}
  {name}
  {id}
  {required}
  aria-label={ariaLabel}
  class:fullwidth
  class:invalid={inputstate === 'invalid'}
  aria-invalid={inputstate === 'invalid'}
  {onkeydown}
  {onchange}
  {oninput} />

<style lang="scss">
.input {
  padding: var(--input-padding);
  border-radius: var(--input-radius);
  background-color: var(--input-background);
  border: var(--border-width) solid var(--input-border);

  &.fullwidth {
    width: 100%;
  }

  &:not(:disabled) {
    &:hover {
      background-color: var(--input-background-hover);
    }

    &:focus {
      background-color: var(--input-background);
    }
  }

  &:disabled {
    color: var(--input-disabled-color);
    background-color: var(--input-disabled-background);
  }

  &.invalid {
    box-shadow: 0 0 0 var(--state-shadow-width) var(--color-invalid-background);
    border-color: var(--color-invalid-border);
  }
}
</style>

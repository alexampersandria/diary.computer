export type InputState = 'untouched' | 'touched' | 'invalid'
export type InputType = 'text' | 'password' | 'email' | 'number' | 'tel' | 'url'

export type ButtonType =
  | 'primary'
  | 'secondary'
  | 'ghost'
  | 'destructive'
  | 'navigation'

export type InputFieldAttributes = {
  disabled?: boolean
  inputstate?: InputState
  name?: string
  required?: boolean
  id?: string
  'aria-label'?: string
  onchange?: (event: Event) => void
  oninput?: (event: Event) => void
  onenter?: (event: KeyboardEvent) => void
}

export type ValidationFunction = (
  value: string,
  inputstate?: InputState,
) => InputState

export type ValidationRule = ValidationFunction | RegExp

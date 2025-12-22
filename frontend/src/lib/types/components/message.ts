import type { Snippet } from 'svelte'
import type { MessageSize, MessageType } from '../message'

export type MessageProps = {
  children: Snippet
  type?: MessageType
  colortext?: boolean
  size?: MessageSize
  muted?: boolean
  centered?: boolean
}

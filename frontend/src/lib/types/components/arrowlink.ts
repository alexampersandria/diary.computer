import type { Snippet } from 'svelte'

export type ArrowLinkProps = {
  children: Snippet
  href?: string
  direction?: 'left' | 'right'
}

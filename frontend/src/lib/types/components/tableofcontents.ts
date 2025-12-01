import type { TableOfContentsItem } from '$lib/utils/toc'

export type TableOfContentsProps = {
  items: TableOfContentsItem[]
}

export type TableOfContentsItemProps = {
  item: TableOfContentsItem
}

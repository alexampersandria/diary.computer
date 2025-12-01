export type TableOfContentsItem = {
  id: string
  title: string
  level: number
  children: TableOfContentsItem[]
}

export type TableOfContents = {
  title?: string
  items: TableOfContentsItem[]
}

const tocElements = 'h1, h2, h3, h4, h5, h6'
const tocSelector = `${tocElements}`

export const generateTableOfContents = (
  root: HTMLElement | undefined,
): TableOfContents => {
  if (!root) {
    return { title: undefined, items: [] }
  }

  const toc: TableOfContentsItem[] = []
  const stack: TableOfContentsItem[] = []
  let title: string | undefined = undefined
  let highestLevel: number | undefined = undefined

  const elements = root.querySelectorAll(tocSelector)

  elements.forEach(el => {
    const level = parseInt(el.tagName.substring(1))
    const item: TableOfContentsItem = {
      id: el.id,
      title: el.textContent || '',
      level,
      children: [],
    }

    if (highestLevel === undefined || level < highestLevel) {
      highestLevel = level
      title = item.title
    }

    while (stack.length > 0 && stack[stack.length - 1].level >= level) {
      stack.pop()
    }

    if (stack.length === 0) {
      toc.push(item)
    } else {
      stack[stack.length - 1].children.push(item)
    }

    stack.push(item)
  })

  return { title, items: toc }
}

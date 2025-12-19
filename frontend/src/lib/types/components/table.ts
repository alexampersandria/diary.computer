export type TableProps = {
  fields?: TableField[]
  data?: TableData
  sortedBy?: string
  sortDirection?: 'asc' | 'desc'
  compareAverageTo?: number
}

export type TableField = {
  label: string
  key: string
  sortable?: boolean
}

export type TableData = {
  // needs to be any, ignore error
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  [key: string]: any
}[]

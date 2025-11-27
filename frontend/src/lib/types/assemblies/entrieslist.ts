import type { Entry } from '../log'
import type { PaginationObject } from '../paginated'

export type EntriesListProps = {
  entries?: Entry[]
  pagination?: PaginationObject
  loading?: boolean
  onloadmore?: () => Promise<void>
}

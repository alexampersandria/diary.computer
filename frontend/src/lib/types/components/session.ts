import type { Session } from '../user'

export type SessionProps = {
  session: Session
  active?: boolean
  onrevoke?: (id: string) => Promise<boolean>
}

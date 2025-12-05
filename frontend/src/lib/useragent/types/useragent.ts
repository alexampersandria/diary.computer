import type { BrowserInfo } from '../browser'
import type { OperatingSystemInfo } from '../os'

/**
 * A type representing a parsed user agent string
 */
export type UserAgent = {
  /**
   * The original user agent string
   */
  useragent: string

  device: {
    vendor?: string
    model?: string
  }
  browser: BrowserInfo
  os: OperatingSystemInfo

  // Flags for specific device types
  isBot: boolean
  isMobile: boolean
  isTablet: boolean
  isDesktop: boolean
  isTV: boolean
  isConsole: boolean

  // Flags for specific browsers
  isChrome: boolean
  isFirefox: boolean
  isSafari: boolean
  isEdge: boolean
  isOpera: boolean

  // Flags for specific operating systems
  isWindows: boolean
  isMacOS: boolean
  isLinux: boolean
  isAndroid: boolean
  isiOS: boolean

  // pretty display string for device
  display: string
}

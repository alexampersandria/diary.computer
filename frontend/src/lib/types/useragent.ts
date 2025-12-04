export type UserAgent = {
  raw: string
  browser: {
    name: string
    version: string
  }
  os: {
    name: string
    version: string
  }
  device: {
    type: string
    brand: string
    model: string
  }
  isMobile: boolean
  isTablet: boolean
  isDesktop: boolean
  isNonHuman: boolean
  isIOS: boolean
  isAndroid: boolean
  isMacOS: boolean
  isWindows: boolean
  isLinux: boolean
  /**
   * A human-readable display string for the user agent
   * @example "Windows 10 PC"
   * @example "iPhone iOS 14.4"
   */
  display: string
}

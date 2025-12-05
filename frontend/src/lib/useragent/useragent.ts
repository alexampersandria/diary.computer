import type { UserAgent } from '$lib/useragent/types/useragent'
import { browserInfo } from './browser'
import { deviceInfo } from './device'
import { operatingSystem } from './os'

const BOT_REGEX = /bot|crawl|spider|openai|http|lighthouse|scan|search|spider/i
const DEV_REGEX = /developer|devtools|lighthouse|postman|yaak|curl/i

/**
 * user agents parser
 * user agents follow format
 * `Mozilla/[version] ([system and browser information]) [platform] ([platform details]) [extensions]`ﬁ
 */
export const parseUserAgent = (userAgent: string): UserAgent => {
  if (userAgent.trim() === '') {
    return {
      useragent: userAgent,
      device: {},
      browser: {},
      os: {},
      isBot: false,
      isMobile: false,
      isTablet: false,
      isDesktop: false,
      isTV: false,
      isConsole: false,
      isChrome: false,
      isFirefox: false,
      isSafari: false,
      isEdge: false,
      isOpera: false,
      isWindows: false,
      isMacOS: false,
      isLinux: false,
      isAndroid: false,
      isiOS: false,
      display: '',
    }
  }

  // content of first parenthesis
  const systemInformation = userAgent.match(/\(([^)]+)\)/)?.[1] || ''

  let device = deviceInfo(systemInformation)
  let os = operatingSystem(systemInformation)

  const browser = browserInfo(userAgent)
  if (!device.model && !device.vendor) {
    device = deviceInfo(userAgent)
  }
  if (!os.name && !os.version) {
    os = operatingSystem(userAgent)
  }

  const isBot = BOT_REGEX.test(userAgent)
  if (isBot && !device.model) {
    device.model = 'BOT'
  }

  let isMobile = false
  let isTablet = false
  let isTV = false
  let isConsole = false

  const isChrome =
    browser.name === 'Chrome' || device.model?.includes('Chrome') || false
  const isFirefox = browser.name === 'Firefox'
  const isSafari = browser.name === 'Safari'
  const isEdge = browser.name === 'Edge'
  const isOpera = browser.name === 'Opera' || browser.name === 'OPR'

  const isWindows = os.name?.includes('Windows') || false
  const isMacOS = os.name?.includes('macOS') || false
  const isLinux = os.name?.includes('Linux') || false
  const isAndroid = os.name?.includes('Android') || false
  const isiOS = os.name?.includes('iOS') || false
  const isIpadOS = os.name?.includes('iPadOS') || false

  if (
    isIpadOS ||
    device.model?.includes('Pad') ||
    device.model?.includes('Tab') ||
    device.model?.includes('Kindle') ||
    device.model?.includes('Pixel C') ||
    userAgent.includes(' Tablet')
  ) {
    isTablet = true
  }

  if (
    os.name?.includes('TV') ||
    device.model?.includes('TV') ||
    device.model?.includes('Roku') ||
    device.model?.includes('Chromecast')
  ) {
    isTV = true
  }

  if (isiOS || (isAndroid && !isTablet) || /Mobile/.test(userAgent)) {
    isMobile = true
  }

  if (isTablet) {
    isMobile = false
  }

  if (isTV) {
    isTablet = false
    isMobile = false
  }

  if (
    device.model?.includes('Xbox') ||
    device.model?.includes('PlayStation') ||
    device.vendor?.includes('Nintendo')
  ) {
    isConsole = true
  }

  const isDesktop = !isMobile && !isTablet && !isTV && !isBot

  if (!device.vendor) {
    if (isiOS || isMacOS || /Apple^W/.test(userAgent)) {
      device.vendor = 'Apple'
    }
  }

  let display = ''
  const addDisplaySection = (section: string, separator?: string) => {
    if (section) {
      if (display) {
        if (separator) {
          display += ` ${separator} `
        } else {
          display += ' '
        }
      }
      display += section
    }
  }

  if (browser.name && !isConsole && !isTV) {
    addDisplaySection(browser.name)
  }

  if (device.model) {
    if (device.vendor && device.vendor !== 'Apple') {
      addDisplaySection(device.vendor, 'on')
      addDisplaySection(device.model)
    } else {
      addDisplaySection(device.model, 'on')
    }
  } else if (device.vendor && !isDesktop) {
    addDisplaySection(device.vendor, 'on')
    if (isMobile) {
      addDisplaySection('Phone')
    } else if (isTablet) {
      addDisplaySection('Tablet')
    } else if (isTV) {
      addDisplaySection('TV')
    } else if (isConsole) {
      addDisplaySection('Console')
    } else {
      addDisplaySection('Device')
    }
  }

  if ((device.model || device.vendor) && !isDesktop && !isConsole && !isTV) {
    if (os.name) {
      if (!display.includes('Windows Phone') && os.name !== 'Windows') {
        let osSection = os.name
        if (os.version) {
          osSection += ` ${os.version}`
        }
        addDisplaySection(`(${osSection})`)
      }
    }
  } else if (os.name && !isConsole && !isTV) {
    addDisplaySection(os.name, 'on')
    if (isMobile || isTablet) {
      addDisplaySection(isMobile ? 'Phone' : 'Tablet')
      if (os.version) {
        let osSection = os.name
        if (os.version) {
          osSection += ` ${os.version}`
        }
        addDisplaySection(`(${osSection})`)
      }
    } else if (os.version) {
      addDisplaySection(os.version)
    }
  }

  if (!device.model && !device.vendor && !os.name) {
    if (isMobile) {
      addDisplaySection('Unknown Phone', 'on')
    } else if (isTablet) {
      addDisplaySection('Unknown Tablet', 'on')
    } else if (isTV) {
      addDisplaySection('TV', 'on')
    } else if (isConsole) {
      addDisplaySection('Console', 'on')
    } else {
      addDisplaySection('Unknown Device', 'on')
    }
  }

  // remove duplicate words in a row (e.g. Microsoft Microsoft) or sequences (e.g. Windows Windows Phone)
  const words = display.split(' ')
  const filteredWords = words.filter((word, index) => word !== words[index - 1])
  display = filteredWords.join(' ')
  display = display.replace('Microsoft Phone', 'Windows Phone')
  display = display.trim()

  if (isBot) {
    display = 'Bot/Crawler'
  }

  if (
    DEV_REGEX.test(userAgent) &&
    (display === '' || display === 'Unknown Device')
  ) {
    display = 'Developer Tool'
  }

  if (display === '') {
    display = 'Unknown Device'
  }

  return {
    useragent: userAgent,
    device,
    browser,
    os,
    isBot,
    isMobile,
    isTablet,
    isDesktop,
    isTV,
    isConsole,
    isChrome,
    isFirefox,
    isSafari,
    isEdge,
    isOpera,
    isWindows,
    isMacOS,
    isLinux,
    isAndroid,
    isiOS,
    display,
  }
}

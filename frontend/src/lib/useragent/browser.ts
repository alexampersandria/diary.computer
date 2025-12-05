import { extractVersion, type Version } from './version'

export const Browsers = [
  'Silk',
  'Chrome',
  'Firefox',
  'Safari',
  'Edge',
  'Opera',
  'Internet Explorer',
  'Samsung Internet',
  'UC Browser',
  'Vivaldi',
  'Brave',
]

export type Browser = (typeof Browsers)[number]

export const BrowserAliases: Array<{
  alias: string
  browser: Browser
}> = [
  { alias: 'CriOS', browser: 'Chrome' },
  { alias: 'FxiOS', browser: 'Firefox' },
  { alias: 'MSIE', browser: 'Internet Explorer' },
  { alias: 'IE', browser: 'Internet Explorer' },
  { alias: 'Trident', browser: 'Internet Explorer' },
  { alias: 'Edg', browser: 'Edge' },
  { alias: 'OPR', browser: 'Opera' },
  { alias: 'SamsungBrowser', browser: 'Samsung Internet' },
  { alias: 'UCBrowser', browser: 'UC Browser' },
]

export type BrowserInfo = {
  name?: Browser
  version?: Version
  major?: string
}

export const matchBrowser = (
  string: string,
): {
  name?: Browser
  browserString?: string
} | null => {
  for (const browserAlias of BrowserAliases) {
    if (string.includes(browserAlias.alias)) {
      return {
        name: browserAlias.browser,
        browserString: browserAlias.alias,
      }
    }
  }

  for (const browser of Browsers) {
    if (string.includes(browser)) {
      return {
        name: browser,
        browserString: browser,
      }
    }
  }

  return null
}

export const browserInfo = (useragent: string): BrowserInfo => {
  const browserInfo: BrowserInfo = {}

  // First, try to match the browser name in the entire useragent
  const browserMatch = matchBrowser(useragent)
  if (!browserMatch) {
    return browserInfo
  }

  browserInfo.name = browserMatch.name
  const browserString = browserMatch.browserString

  if (!browserString) {
    return browserInfo
  }

  // Try to find version using the matched browser string
  let foundVersion = false
  const index = useragent.indexOf(browserString)
  if (index !== -1) {
    // Check if the next character after browser name is a forward slash
    const afterBrowser = useragent.substring(index + browserString.length)
    if (afterBrowser.startsWith('/')) {
      // Extract version after the slash
      browserInfo.version = extractVersion(afterBrowser.substring(1))
      const majorVersion = browserInfo.version?.split('.')[0]
      if (majorVersion) {
        browserInfo.major = majorVersion
      }
      foundVersion = true
    }
  }

  // If we didn't find version with the alias, try the full browser name
  if (!foundVersion && browserInfo.name) {
    const fullBrowserName = browserInfo.name
    const nameIndex = useragent.indexOf(fullBrowserName)
    if (nameIndex !== -1) {
      const afterName = useragent.substring(nameIndex + fullBrowserName.length)
      if (afterName.startsWith('/')) {
        browserInfo.version = extractVersion(afterName.substring(1))
        const majorVersion = browserInfo.version?.split('.')[0]
        if (majorVersion) {
          browserInfo.major = majorVersion
        }
        foundVersion = true
      }
    }
  }

  // Special case for Opera: look for "Version/" pattern
  if (browserInfo.name === 'Opera' && useragent.includes('Version/')) {
    const versionIndex = useragent.indexOf('Version/')
    const afterVersion = useragent.substring(versionIndex + 'Version/'.length)
    const version = extractVersion(afterVersion)
    if (version) {
      browserInfo.version = version
      const majorVersion = version.split('.')[0]
      if (majorVersion) {
        browserInfo.major = majorVersion
      }
    }
  }

  return browserInfo
}

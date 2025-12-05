import { extractVersion, formatVersion, type Version } from './version'

export const OperatingSystems = [
  'Windows',
  'Android',
  'Linux',
  'iPadOS',
  'iOS',
  'macOS',
  'Chrome OS',
  'WebOS',
  'Android TV',
  'Tizen',
  'Fire OS',
  'PlayStation',
  'Xbox',
  'Nintendo',
]

export type OperatingSystem = (typeof OperatingSystems)[number]

export const OperatingSystemsAliases: Array<{
  alias: string
  operatingSystem: OperatingSystem
}> = [
  { alias: 'Windows Phone', operatingSystem: 'Windows' },
  { alias: 'Windows NT', operatingSystem: 'Windows' },
  { alias: 'iPad', operatingSystem: 'iPadOS' },
  { alias: 'iPhone', operatingSystem: 'iOS' },
  { alias: 'Mac OS X', operatingSystem: 'macOS' },
  { alias: 'Macintosh', operatingSystem: 'macOS' },
  { alias: 'CrOS', operatingSystem: 'Chrome OS' },
  { alias: 'CrKey', operatingSystem: 'Android TV' },
  { alias: 'Kindle', operatingSystem: 'Fire OS' },
]

export type OperatingSystemInfo = {
  name?: OperatingSystem
  version?: Version
}

const matchOperatingSystem = (
  string: string,
): {
  name: OperatingSystem
  operatingSystemString: string
} | null => {
  for (const os of OperatingSystems) {
    if (!string.includes(os)) {
      continue
    }
    return {
      name: os,
      operatingSystemString: os,
    }
  }

  for (const osAlias of OperatingSystemsAliases) {
    if (!string.includes(osAlias.alias)) {
      continue
    }
    return {
      name: osAlias.operatingSystem,
      operatingSystemString: osAlias.alias,
    }
  }

  return null
}

export const operatingSystem = (useragent: string): OperatingSystemInfo => {
  const osInfo: OperatingSystemInfo = {}

  // useragent is seperated by ;
  // an OS entry should look like "Windows NT 10.0" or "Mac OS X 10_15_7" or "Android 11"
  // the name followed by the version
  const useragentParts = useragent.split(';').map(part => part.trim())

  // Special handling for iOS/iPadOS: look for "iPhone OS", "iPad OS", or "CPU OS" pattern first
  // Need to determine if it's iPad or iPhone based on device identifier in first part
  let isIPad = false
  for (const part of useragentParts) {
    if (part.includes('iPad')) {
      isIPad = true
      break
    }
  }

  for (const part of useragentParts) {
    const iosVersionMatch = part.match(
      /(?:iPhone|iPad|CPU) OS ([0-9]+(?:[._][0-9]+)*)/,
    )
    if (iosVersionMatch) {
      osInfo.name = isIPad ? 'iPadOS' : 'iOS'
      osInfo.version = formatVersion(iosVersionMatch[1])
      break
    }
  }

  // If we already found iOS version, skip further processing
  if (osInfo.name && osInfo.version) {
    // Still check for Windows version cleanup
    if (osInfo.name === 'Windows' && osInfo.version) {
      // cleanup Windows version because microsoft versioning is nonsense
      if (osInfo.version === '10.0') {
        osInfo.version = '10'
      } else if (osInfo.version === '6.3') {
        osInfo.version = '8.1'
      } else if (osInfo.version === '6.2') {
        osInfo.version = '8'
      } else if (osInfo.version === '6.1') {
        osInfo.version = '7'
      }
    }
    return osInfo
  }

  for (const part of useragentParts) {
    const osMatch = matchOperatingSystem(part)
    if (!osMatch) {
      continue
    }
    if (!osInfo.name) {
      osInfo.name = osMatch.name
      osInfo.version = extractVersion(part)
    } else {
      // compare index of OperatingSystem array to find the most relevant OS
      const existingOsIndex = OperatingSystems.indexOf(osInfo.name)
      const newOsIndex = OperatingSystems.indexOf(osMatch.name)
      if (newOsIndex < existingOsIndex) {
        osInfo.name = osMatch.name
        osInfo.version = extractVersion(part)
      }
    }
    if (osInfo.name && !osInfo.version) {
      const osAliases = [osInfo.name].flatMap(() => {
        return OperatingSystemsAliases.filter(
          alias => alias.operatingSystem === osInfo.name,
        )
      })
      if (
        osAliases &&
        osAliases.some(alias => alias.alias === osMatch.operatingSystemString)
      ) {
        osInfo.version = extractVersion(part)
      }
      // windows XP case, part will include "(Windows XP)"
      else if (osInfo.name === 'Windows' && part.includes('Windows XP')) {
        osInfo.version = 'XP'
      }
    }
  }

  if (osInfo.name === 'Windows' && osInfo.version) {
    // cleanup Windows version because microsoft versioning is nonsense
    if (osInfo.version === '10.0') {
      osInfo.version = '10'
    } else if (osInfo.version === '6.3') {
      osInfo.version = '8.1'
    } else if (osInfo.version === '6.2') {
      osInfo.version = '8'
    } else if (osInfo.version === '6.1') {
      osInfo.version = '7'
    } else if (osInfo.version === '6.0') {
      osInfo.version = 'Vista'
    }
  }

  return osInfo
}

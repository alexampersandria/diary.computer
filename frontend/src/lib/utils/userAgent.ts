import type { UserAgent } from '$lib/types/useragent'

export const parseUserAgent = (userAgent: string): UserAgent => {
  const browserMatch =
    userAgent.match(
      /(Firefox|MSIE|Trident|Edge|Chrome|Safari|Opera|OPR)\/([\d.]+)/,
    ) || []
  const osMatch = userAgent.match(/\(([^)]+)\)/)?.[1].replaceAll('_', '.') || ''
  const os = osMatch.split(';')[0]?.trim() || 'Unknown'
  const osVersion =
    osMatch
      .split(';')[1]
      ?.trim()
      .match(/\d+(\.\d+)*/)?.[0] || 'Unknown'
  const deviceType = /Mobile|Tablet/.test(userAgent) ? 'Mobile' : 'Desktop'

  const isMobile = /mobile/.test(userAgent.toLowerCase())
  const isTablet = /tablet/.test(userAgent.toLowerCase())
  const isDesktop = !isMobile && !isTablet
  const isIOS = /iphone|ipad|ipod/.test(userAgent.toLowerCase())
  const isAndroid = /android/.test(userAgent.toLowerCase())
  const isMacOS = /macintosh/.test(userAgent.toLowerCase())
  const isWindows = /windows/.test(userAgent.toLowerCase())
  const isLinux = /linux/.test(userAgent.toLowerCase()) && !isAndroid
  const isNonHuman =
    /bot|postman|yaak|crawl|spider|slurp|bingpreview|mediapartners-google/i.test(
      userAgent.toLowerCase(),
    )

  const display = () => {
    if (isIOS) {
      if (osMatch.includes('iPhone')) {
        return `iPhone iOS ${osVersion}`
      } else if (osMatch.includes('iPad')) {
        return `iPad iOS ${osVersion}`
      } else {
        return `iOS Device ${osVersion}`
      }
    } else if (isAndroid) {
      return `Android ${osVersion}`
    } else if (isMacOS) {
      return `MacOS ${osVersion}`
    } else if (isWindows) {
      return `Windows ${osVersion}`
    } else if (isLinux) {
      return `Linux Device`
    } else {
      return 'Unknown Device'
    }
  }

  return {
    raw: userAgent,
    browser: {
      name: browserMatch[1] || 'Unknown',
      version: browserMatch[2] || 'Unknown',
    },
    os: {
      name: os,
      version: osVersion,
    },
    device: {
      type: deviceType,
      brand: isIOS ? 'Apple' : isAndroid ? 'Android' : 'Unknown',
      model: isIOS ? 'iPhone/iPad' : isAndroid ? 'Android Device' : 'Unknown',
    },
    isMobile,
    isTablet,
    isDesktop,
    isNonHuman,
    isIOS,
    isAndroid,
    isMacOS,
    isWindows,
    isLinux,
    display: display(),
  }
}

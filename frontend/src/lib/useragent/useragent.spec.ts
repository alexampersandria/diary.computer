import { describe, it, expect } from 'vitest'
import { parseUserAgent } from './useragent'

describe('parseUserAgent', () => {
  describe('Android devices', () => {
    it('should parse Samsung Galaxy S25', () => {
      const ua =
        'Mozilla/5.0 (Linux; Android 15; SM-S931B Build/AP3A.240905.015.A2; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/127.0.6533.103 Mobile Safari/537.36'
      const result = parseUserAgent(ua)

      expect(result.useragent).toBe(ua)
      expect(result.os.name).toBe('Android')
      expect(result.os.version).toBe('15')
      expect(result.browser.name).toBe('Chrome')
      expect(result.browser.version).toBe('127.0.6533.103')
      expect(result.browser.major).toBe('127')
      expect(result.isAndroid).toBe(true)
      expect(result.isMobile).toBe(true)
      expect(result.isDesktop).toBe(false)
      expect(result.isBot).toBe(false)
      expect(result.display).toContain('Chrome on Samsung Galaxy (Android 15)')
    })

    it('should parse Samsung Galaxy S24 Ultra', () => {
      const ua =
        'Mozila/5.0 (Linux; Android 14; SM-S928B/DS) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.6099.230 Mobile Safari/537.36'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Android')
      expect(result.os.version).toBe('14')
      expect(result.browser.name).toBe('Chrome')
      expect(result.browser.version).toBe('120.0.6099.230')
      expect(result.browser.major).toBe('120')
      expect(result.isMobile).toBe(true)
      expect(result.isChrome).toBe(true)
      expect(result.display).toContain('Chrome on Samsung Galaxy')
    })

    it('should parse Samsung Galaxy Xcover7 with Firefox', () => {
      const ua =
        'Mozilla/5.0 (Android 15; Mobile; SM-G556B/DS; rv:130.0) Gecko/130.0 Firefox/130.0'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Android')
      expect(result.os.version).toBe('15')
      expect(result.browser.name).toBe('Firefox')
      expect(result.browser.version).toBe('130.0')
      expect(result.browser.major).toBe('130')
      expect(result.isFirefox).toBe(true)
      expect(result.isMobile).toBe(true)
      expect(result.display).toContain('Firefox on Samsung Galaxy')
    })

    it('should parse Google Pixel 9 Pro', () => {
      const ua =
        'Mozilla/5.0 (Linux; Android 14; Pixel 9 Pro Build/AD1A.240418.003; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/124.0.6367.54 Mobile Safari/537.36'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Android')
      expect(result.device.model).toBe('Pixel')
      expect(result.isMobile).toBe(true)
      expect(result.display).toContain('Chrome on Google Pixel (Android 14)')
    })

    it('should parse Google Pixel 8', () => {
      const ua =
        'Mozilla/5.0 (Linux; Android 15; Pixel 8 Build/AP4A.250105.002; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/132.0.6834.163 Mobile Safari/537.36'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Android')
      expect(result.os.version).toBe('15')
      expect(result.isMobile).toBe(true)
      expect(result.display).toContain('Chrome on Google Pixel (Android 15)')
    })

    it('should parse Motorola Moto G (2025)', () => {
      const ua =
        'Mozilla/5.0 (Linux; Android 15; moto g - 2025 Build/V1VK35.22-13-2; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/132.0.6834.163 Mobile Safari/537.36'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Android')
      expect(result.os.version).toBe('15')
      expect(result.device.vendor).toBe('Motorola')
      expect(result.isMobile).toBe(true)
      expect(result.display).toContain('Chrome on Motorola Phone (Android 15)')
    })

    it('should parse Motorola Razr 50 Ultra', () => {
      const ua =
        'Mozilla/5.0 (Linux; Android 14; motorola razr 50 ultra Build/U3UX34.56-29-2; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/126.0.6478.134 Mobile Safari/537.36'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Android')
      expect(result.os.version).toBe('14')
      expect(result.isMobile).toBe(true)
      expect(result.device.vendor).toBe('Motorola')
      expect(result.display).toContain('Chrome on Motorola Phone (Android 14)')
    })

    it('should parse Huawei Pura 70 Ultra', () => {
      const ua =
        'Mozilla/5.0 (Linux; Android 12; HBP-LX9 Build/HUAWEIHBP-L29; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/99.0.4844.88 Mobile Safari/537.36'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Android')
      expect(result.device.vendor).toBe('Huawei')
      expect(result.isMobile).toBe(true)
      expect(result.display).toContain('Chrome on Huawei Phone')
    })

    it('should parse Xiaomi 14 Ultra', () => {
      const ua =
        'Mozilla/5.0 (Linux; Android 14; 24030PN60G Build/UKQ1.231003.002; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/122.0.6261.119 Mobile Safari/537.36'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Android')
      expect(result.os.version).toBe('14')
      expect(result.isMobile).toBe(true)
      expect(result.display).toContain('Chrome on Android Phone')
    })

    it('should parse OnePlus Nord N200 5G', () => {
      const ua =
        'Mozilla/5.0 (Linux; Android 12; DE2118) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Android')
      expect(result.os.version).toBe('12')
      expect(result.device.vendor).toBe('OnePlus')
      expect(result.isMobile).toBe(true)
      expect(result.display).toContain('Chrome on OnePlus Nord')
    })

    it('should parse Android with reduced User-Agent (Client Hints)', () => {
      const ua =
        'Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Mobile Safari/537.36'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Android')
      expect(result.os.version).toBe('10')
      expect(result.browser.name).toBe('Chrome')
      expect(result.browser.version).toBe('114.0.0.0')
      expect(result.browser.major).toBe('114')
      expect(result.isMobile).toBe(true)
      expect(result.display).toContain('Chrome on Android Phone')
    })
  })

  describe('niche phones', () => {
    it('should parse blackberry bb10', () => {
      const ua =
        'Mozilla/5.0 (BB10; Touch) AppleWebKit/537.1+ (KHTML, like Gecko) Version/10.0.0.1337 Mobile Safari/537.1+'
      const result = parseUserAgent(ua)

      expect(result.isMobile).toBe(true)
      expect(result.display).toBe('Safari on Unknown Phone')
    })

    it('should parse generic android tablet', () => {
      const ua =
        'Mozilla/5.0 (Android 4.4; Tablet; rv:70.0) Gecko/70.0 Firefox/70.0'
      const result = parseUserAgent(ua)
      expect(result.os.name).toBe('Android')
      expect(result.isTablet).toBe(true)
      expect(result.display).toContain('Firefox on Android Tablet')
    })

    it('should parse iPad on iOS 8.3', () => {
      const ua =
        'Mozilla/5.0 (iPad; CPU iPhone OS 8_3 like Mac OS X) AppleWebKit/600.1.4 (KHTML, like Gecko) FxiOS/1.0 Mobile/12F69 Safari/600.1.4'
      const result = parseUserAgent(ua)
      expect(result.os.name).toBe('iPadOS')
      expect(result.os.version).toBe('8.3')
      expect(result.device.model).toBe('iPad')
      expect(result.device.vendor).toBe('Apple')
      expect(result.isTablet).toBe(true)
      expect(result.display).toContain('Firefox on iPad')
    })
  })

  describe('iPhone devices', () => {
    it('should parse iPhone with Safari', () => {
      const ua =
        'Mozilla/5.0 (iPhone; CPU iPhone OS 12_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/12.0 Mobile/15E148 Safari/604.1'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('iOS')
      expect(result.os.version).toBe('12.0')
      expect(result.browser.name).toBe('Safari')
      expect(result.browser.version).toBe('604.1')
      expect(result.browser.major).toBe('604')
      expect(result.isSafari).toBe(true)
      expect(result.isMobile).toBe(true)
      expect(result.isDesktop).toBe(false)
      expect(result.display).toContain('Safari on iPhone (iOS 12.0)')
    })

    it('should parse iPhone with Chrome', () => {
      const ua =
        'Mozilla/5.0 (iPhone; CPU iPhone OS 12_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/69.0.3497.105 Mobile/15E148 Safari/605.1'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('iOS')
      expect(result.browser.name).toBe('Chrome')
      expect(result.browser.version).toBe('69.0.3497.105')
      expect(result.browser.major).toBe('69')
      expect(result.isChrome).toBe(true)
      expect(result.isMobile).toBe(true)
      expect(result.display).toContain('Chrome on iPhone')
    })

    it('should parse iPhone with Firefox', () => {
      const ua =
        'Mozilla/5.0 (iPhone; CPU iPhone OS 12_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/13.2b11866 Mobile/16A366 Safari/605.1.15'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('iOS')
      expect(result.browser.name).toBe('Firefox')
      expect(result.browser.version).toBe('13.2')
      expect(result.browser.major).toBe('13')
      expect(result.isFirefox).toBe(true)
      expect(result.isMobile).toBe(true)
      expect(result.display).toContain('Firefox on iPhone')
    })

    it('should parse iPhone 14', () => {
      const ua =
        'Mozilla/5.0 (iPhone14,7; CPU iPhone OS 18_3_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 Mohegan Sun/4.7.3'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('iOS')
      expect(result.isMobile).toBe(true)
      expect(result.display).toBe('iPhone (iOS 18.3.2)')
    })

    it('should parse iPhone 17', () => {
      const ua =
        'Mozilla/5.0 (iPhone17,1; CPU iPhone OS 18_2_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 Mohegan Sun/4.7.4'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('iOS')
      expect(result.isMobile).toBe(true)
      expect(result.display).toBe('iPhone (iOS 18.2.1)')
    })

    it('should parse older iPhone OS 11', () => {
      const ua =
        'Mozilla/5.0 (iPhone; CPU iPhone OS 11_0 like Mac OS X) AppleWebKit/604.1.38 (KHTML, like Gecko) Version/11.0 Mobile/15A372 Safari/604.1'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('iOS')
      expect(result.os.version).toBe('11.0')
      expect(result.isMobile).toBe(true)
      expect(result.display).toContain('Safari on iPhone (iOS 11.0)')
    })

    it('should parse iPhone 9 with iOS 10', () => {
      const ua =
        'Mozilla/5.0 (iPhone9,3; U; CPU iPhone OS 10_0_1 like Mac OS X) AppleWebKit/602.1.50 (KHTML, like Gecko) Version/10.0 Mobile/14A403 Safari/602.1'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('iOS')
      expect(result.isMobile).toBe(true)
      expect(result.display).toContain('Safari on iPhone (iOS 10.0.1)')
    })
  })

  describe('Windows Phone devices', () => {
    it('should parse Windows Phone 10 Lumia 950', () => {
      const ua =
        'Mozilla/5.0 (Windows Phone 10.0; Android 4.2.1; Microsoft; Lumia 950) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/46.0.2486.0 Mobile Safari/537.36 Edge/13.1058'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Windows')
      expect(result.browser.name).toBe('Edge')
      expect(result.browser.version).toBe('13.1058')
      expect(result.browser.major).toBe('13')
      expect(result.isEdge).toBe(true)
      expect(result.isMobile).toBe(true)
      expect(result.display).toContain('Edge on Windows Phone')
    })

    it('should parse Windows Phone 10 with Edge', () => {
      const ua =
        'Mozilla/5.0 (Windows Phone 10.0; Android 6.0.1; Microsoft; RM-1152) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/52.0.2743.116 Mobile Safari/537.36 Edge/15.15254'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Windows')
      expect(result.browser.name).toBe('Edge')
      expect(result.browser.version).toBe('15.15254')
      expect(result.browser.major).toBe('15')
      expect(result.isMobile).toBe(true)
    })
  })

  describe('Tablet devices', () => {
    it('should parse iPad', () => {
      const ua =
        'Mozilla/5.0 (iPad16,3; CPU OS 18_3_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 Tropicana_NJ/5.7.1'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('iPadOS')
      expect(result.isTablet).toBe(true)
      expect(result.isMobile).toBe(false)
      expect(result.display).toBe('iPad (iPadOS 18.3.2)')
    })

    it('should parse Samsung Galaxy Tab', () => {
      const ua =
        'Dalvik/2.1.0 (Linux; U; Android 14; SM-X306B Build/UP1A.231005.007)'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Android')
      expect(result.os.version).toBe('14')
      expect(result.device.vendor).toBe('Samsung')
      expect(result.device.model).toBe('Galaxy Tab')
      expect(result.isTablet).toBe(true)
      expect(result.display).toBe('Samsung Galaxy Tab (Android 14)')
    })

    it('should parse Xiaomi tablet', () => {
      const ua =
        'Dalvik/2.1.0 (Linux; U; Android 15; 24091RPADG Build/AQ3A.240801.002)'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Android')
      expect(result.os.version).toBe('15')
      expect(result.device.vendor).toBe('Xiaomi')
      expect(result.isTablet).toBe(true)
      expect(result.display).toBe('Xiaomi Pad (Android 15)')
    })

    it('should parse Kindle Fire tablet', () => {
      const ua =
        'Dalvik/2.1.0 (Linux; U; Android 11; KFRASWI Build/RS8332.3115N)'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Android')
      expect(result.os.version).toBe('11')
      expect(result.device.model).toBe('Kindle')
      expect(result.device.vendor).toBe('Amazon')
      expect(result.isTablet).toBe(true)
      expect(result.display).toBe('Amazon Kindle (Android 11)')
    })

    it('should parse Lenovo tablet', () => {
      const ua =
        'Mozilla/5.0 (Linux; Android 11; Lenovo YT-J706X) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.45 Safari/537.36'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Android')
      expect(result.device.vendor).toBe('Lenovo')
      expect(result.isTablet).toBe(true)
      expect(result.display).toBe('Chrome on Lenovo Tablet (Android 11)')
    })

    it('should parse Google Pixel C tablet', () => {
      const ua =
        'Mozilla/5.0 (Linux; Android 7.0; Pixel C Build/NRD90M; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/52.0.2743.98 Safari/537.36'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Android')
      expect(result.device.model).toBe('Pixel C')
      expect(result.isTablet).toBe(true)
      expect(result.display).toBe('Chrome on Google Pixel C (Android 7.0)')
    })
  })

  describe('Desktop devices', () => {
    it('should parse Windows 10 with Edge', () => {
      const ua =
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36 Edg/134.0.0.0'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Windows')
      expect(result.os.version).toBe('10')
      expect(result.browser.name).toBe('Edge')
      expect(result.browser.version).toBe('134.0.0.0')
      expect(result.browser.major).toBe('134')
      expect(result.isEdge).toBe(true)
      expect(result.isDesktop).toBe(true)
      expect(result.isMobile).toBe(false)
      expect(result.display).toBe('Edge on Windows 10')
    })

    it('should parse macOS with Safari', () => {
      const ua =
        'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.3.1 Safari/605.1.15'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('macOS')
      expect(result.browser.name).toBe('Safari')
      expect(result.browser.version).toBe('605.1.15')
      expect(result.browser.major).toBe('605')
      expect(result.isSafari).toBe(true)
      expect(result.isDesktop).toBe(true)
      expect(result.isMobile).toBe(false)
      expect(result.display).toBe('Safari on macOS 10.15.7')
    })

    it('should parse ChromeOS', () => {
      const ua =
        'Mozilla/5.0 (X11; CrOS x86_64 14541.0.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Chrome OS')
      expect(result.browser.name).toBe('Chrome')
      expect(result.browser.version).toBe('134.0.0.0')
      expect(result.browser.major).toBe('134')
      expect(result.isChrome).toBe(true)
      expect(result.isDesktop).toBe(true)
    })

    it('should parse Ubuntu Linux with Firefox', () => {
      const ua =
        'Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:15.0) Gecko/20100101 Firefox/15.0.1'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Linux')
      expect(result.browser.name).toBe('Firefox')
      expect(result.browser.version).toBe('15.0.1')
      expect(result.browser.major).toBe('15')
      expect(result.isFirefox).toBe(true)
      expect(result.isDesktop).toBe(true)
    })

    it('should parse Windows 7 with Chrome', () => {
      const ua =
        'Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36>'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Windows')
      expect(result.os.version).toBe('7')
      expect(result.browser.name).toBe('Chrome')
      expect(result.browser.version).toBe('109.0.0.0')
      expect(result.browser.major).toBe('109')
      expect(result.isDesktop).toBe(true)
      expect(result.display).toContain('Chrome on Windows 7')
    })

    it('should parse Windows Vista', () => {
      const ua = 'Opera/9.80 (Windows NT 6.0) Presto/2.12.388 Version/12.14'

      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Windows')
      expect(result.os.version).toBe('Vista')
      expect(result.browser.name).toBe('Opera')
      expect(result.browser.version).toBe('12.14')
      expect(result.browser.major).toBe('12')
      expect(result.isDesktop).toBe(true)
      expect(result.display).toContain('Opera on Windows Vista')
    })

    it('should parse Windows XP', () => {
      const ua =
        'Mozilla/5.0 (Windows XP) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/73.0.3683.103 Safari/537.36'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Windows')
      expect(result.os.version).toBe('XP')
      expect(result.browser.name).toBe('Chrome')
      expect(result.browser.version).toBe('73.0.3683.103')
      expect(result.browser.major).toBe('73')
      expect(result.isDesktop).toBe(true)
      expect(result.display).toContain('Chrome on Windows XP')
    })

    it('should parse Windows 8', () => {
      const ua =
        'Mozilla/5.0 (Windows NT 6.2; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Windows')
      expect(result.os.version).toBe('8')
      expect(result.isDesktop).toBe(true)
      expect(result.display).toContain('Chrome on Windows 8')
    })

    it('should parse Windows 8.1', () => {
      const ua =
        'Mozilla/5.0 (Windows NT 6.3; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Windows')
      expect(result.os.version).toBe('8.1')
      expect(result.isDesktop).toBe(true)
      expect(result.display).toContain('Chrome on Windows 8.1')
    })
  })

  describe('Set Top Boxes', () => {
    it('should parse Fire TV Stick', () => {
      const ua =
        'Mozilla/5.0 (Linux; Android 11; AFTKRT Build/RS8101.1849N; wv)PlexTV/10.0.0.4149'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Android')
      expect(result.device.model).toBe('Fire TV')
      expect(result.isDesktop).toBe(false)
      expect(result.isMobile).toBe(false)
      expect(result.isTablet).toBe(false)
      expect(result.isTV).toBe(true)
      expect(result.display).toBe('Amazon Fire TV')
    })

    it('should parse Apple TV', () => {
      const ua = 'AppleTV14,1/16.1'
      const result = parseUserAgent(ua)

      expect(result.device.vendor).toBe('Apple')
      expect(result.device.model).toBe('Apple TV')
      expect(result.isDesktop).toBe(false)
      expect(result.isMobile).toBe(false)
      expect(result.isTablet).toBe(false)
      expect(result.isTV).toBe(true)
      expect(result.display).toBe('Apple TV')
    })

    it('should parse Roku', () => {
      const ua = 'Roku4640X/DVP-7.70 (297.70E04154A)'
      const result = parseUserAgent(ua)

      expect(result.device.vendor).toBe('Roku')
      expect(result.isDesktop).toBe(false)
      expect(result.isMobile).toBe(false)
      expect(result.isTablet).toBe(false)
      expect(result.isTV).toBe(true)
      expect(result.display).toBe('Roku')
    })

    it('should parse Chromecast', () => {
      const ua =
        'Mozilla/5.0 (CrKey armv7l 1.5.16041) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/31.0.1650.0 Safari/537.36'
      const result = parseUserAgent(ua)

      expect(result.device.model).toBe('Chromecast')
      expect(result.isDesktop).toBe(false)
      expect(result.isMobile).toBe(false)
      expect(result.isTablet).toBe(false)
      expect(result.isTV).toBe(true)
      expect(result.display).toBe('Chromecast')
    })

    it('should parse Amazon Fire TV', () => {
      const ua =
        'Mozilla/5.0 (Linux; Android 9; AFTR) AppleWebKit/537.36 (KHTML, like Gecko) Silk/98.6.10 like Chrome/98.0.4758.136 Safari/537.36'
      const result = parseUserAgent(ua)

      expect(result.device.vendor).toBe('Amazon')
      expect(result.device.model).toBe('Fire TV')
      expect(result.os.name).toBe('Android')
      expect(result.browser.name).toBe('Silk')
      expect(result.isDesktop).toBe(false)
      expect(result.isMobile).toBe(false)
      expect(result.isTablet).toBe(false)
      expect(result.isTV).toBe(true)
      expect(result.display).toBe('Amazon Fire TV')
    })
  })

  describe('Game Consoles', () => {
    it('should parse PlayStation 5', () => {
      const ua =
        'Mozilla/5.0 (PlayStation; PlayStation 5/2.26) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.0 Safari/605.1.15'
      const result = parseUserAgent(ua)

      expect(result.device.vendor).toBe('Sony')
      expect(result.device.model).toBe('PlayStation 5')
      expect(result.isConsole).toBe(true)
      expect(result.display).toBe('Sony PlayStation 5')
    })

    it('should parse PlayStation 4', () => {
      const ua =
        'Mozilla/5.0 (PlayStation 4 3.11) AppleWebKit/537.73 (KHTML, like Gecko)'
      const result = parseUserAgent(ua)

      expect(result.device.vendor).toBe('Sony')
      expect(result.device.model).toBe('PlayStation 4')
      expect(result.isConsole).toBe(true)
      expect(result.display).toBe('Sony PlayStation 4')
    })

    it('should parse Xbox Series X', () => {
      const ua =
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64; Xbox; Xbox Series X) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/48.0.2564.82 Safari/537.36 Edge/20.02'
      const result = parseUserAgent(ua)

      expect(result.device.vendor).toBe('Microsoft')
      expect(result.device.model).toBe('Xbox Series X')
      expect(result.isConsole).toBe(true)
      expect(result.display).toBe('Microsoft Xbox Series X')
    })

    it('should parse Xbox One', () => {
      const ua =
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64; XBOX_ONE_ED) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.79 Safari/537.36 Edge/14.14393'
      const result = parseUserAgent(ua)

      expect(result.device.vendor).toBe('Microsoft')
      expect(result.device.model).toBe('Xbox One')
      expect(result.isConsole).toBe(true)
      expect(result.display).toBe('Microsoft Xbox One')
    })

    it('should parse Nintendo Switch', () => {
      const ua =
        'Mozilla/5.0 (Nintendo Switch; WifiWebAuthApplet) AppleWebKit/601.6 (KHTML, like Gecko) NF/4.0.0.5.10 NintendoBrowser/5.1.0.13343'
      const result = parseUserAgent(ua)

      expect(result.device.vendor).toBe('Nintendo')
      expect(result.device.model).toBe('Switch')
      expect(result.isConsole).toBe(true)
      expect(result.display).toBe('Nintendo Switch')
    })

    it('should parse Nintendo Wii U', () => {
      const ua =
        'Mozilla/5.0 (Nintendo WiiU) AppleWebKit/536.30 (KHTML, like Gecko) NX/3.0.4.2.12 NintendoBrowser/4.3.1.11264.US'
      const result = parseUserAgent(ua)

      expect(result.device.vendor).toBe('Nintendo')
      expect(result.device.model).toBe('Wii U')
      expect(result.isConsole).toBe(true)
      expect(result.display).toBe('Nintendo Wii U')
    })

    it('should parse Nintendo 3DS', () => {
      const ua = 'Mozilla/5.0 (Nintendo 3DS; U; ; en) Version/1.7412.EU'
      const result = parseUserAgent(ua)

      expect(result.device.vendor).toBe('Nintendo')
      expect(result.device.model).toBe('3DS')
      expect(result.isConsole).toBe(true)
      expect(result.display).toBe('Nintendo 3DS')
    })

    it('should parse PlayStation Vita', () => {
      const ua =
        'Mozilla/5.0 (PlayStation Vita 3.61) AppleWebKit/537.73 (KHTML, like Gecko) Silk/3.2'
      const result = parseUserAgent(ua)

      expect(result.device.vendor).toBe('Sony')
      expect(result.device.model).toBe('PlayStation Vita')
      expect(result.isConsole).toBe(true)
      expect(result.display).toBe('Sony PlayStation Vita')
    })
  })

  describe('Bots and Crawlers', () => {
    it('should parse Googlebot', () => {
      const ua =
        'Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)'
      const result = parseUserAgent(ua)

      expect(result.isBot).toBe(true)
      expect(result.device.model).toBe('BOT')
      expect(result.isMobile).toBe(false)
      expect(result.isDesktop).toBe(false)
    })

    it('should parse Bingbot', () => {
      const ua =
        'Mozilla/5.0 (compatible; bingbot/2.0; +http://www.bing.com/bingbot.htm)'
      const result = parseUserAgent(ua)

      expect(result.isBot).toBe(true)
      expect(result.device.model).toBe('BOT')
    })

    it('should parse FacebookBot', () => {
      const ua =
        'Mozilla/5.0 (compatible; FacebookBot/1.0; +https://developers.facebook.com/docs/sharing/webmasters/facebookbot/)'
      const result = parseUserAgent(ua)

      expect(result.isBot).toBe(true)
      expect(result.device.model).toBe('BOT')
    })

    it('should parse ChatGPT bot', () => {
      const ua =
        'Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko); compatible; ChatGPT-User/1.0; +https://openai.com/bot'
      const result = parseUserAgent(ua)

      expect(result.isBot).toBe(true)
      expect(result.device.model).toBe('BOT')
    })

    it('should parse OpenAI SearchBot', () => {
      const ua =
        'Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko); compatible; OAI-SearchBot/1.0; +https://openai.com/searchbot'
      const result = parseUserAgent(ua)

      expect(result.isBot).toBe(true)
      expect(result.device.model).toBe('BOT')
      expect(result.display).toContain('Bot')
    })

    it('should parse Yahoo Slurp', () => {
      const ua =
        'Mozilla/5.0 (compatible; Yahoo! Slurp; http://help.yahoo.com/help/us/ysearch/slurp)'
      const result = parseUserAgent(ua)

      expect(result.isBot).toBe(true)
      expect(result.device.model).toBe('BOT')
    })
  })

  describe('E-Readers', () => {
    it('should parse Kindle 3', () => {
      const ua =
        'Mozilla/5.0 (X11; U; Linux armv7l like Android; en-us) AppleWebKit/531.2+ (KHTML, like Gecko) Version/5.0 Safari/533.2+ Kindle/3.0+'
      const result = parseUserAgent(ua)

      expect(result.device.vendor).toBe('Amazon')
      expect(result.device.model).toBe('Kindle')
      expect(result.display).toContain('Kindle (Android 7)')
    })

    it('should parse Kindle e-ink reader', () => {
      const ua =
        'Mozilla/5.0 (Linux; U; en-US) AppleWebKit/528.5+ (KHTML, like Gecko, Safari/528.5+) Version/4.0 Kindle/3.0 (screen 600x800; rotate)'
      const result = parseUserAgent(ua)

      expect(result.device.model).toBe('Kindle')
    })

    it('should parse NoteAir3C e-reader', () => {
      const ua =
        'Dalvik/2.1.0 (Linux; U; Android 12; NoteAir3C Build/2023-11-15_15-07_3.5_0a296ec2c)'
      const result = parseUserAgent(ua)

      expect(result.os.name).toBe('Android')
      expect(result.device.model).toBe('Note')
    })
  })

  describe('Browser detection', () => {
    it('should detect Chrome browser', () => {
      const ua =
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36'
      const result = parseUserAgent(ua)

      expect(result.browser.name).toBe('Chrome')
      expect(result.browser.version).toBe('134.0.0.0')
      expect(result.browser.major).toBe('134')
      expect(result.isChrome).toBe(true)
      expect(result.isFirefox).toBe(false)
      expect(result.isSafari).toBe(false)
      expect(result.isEdge).toBe(false)
    })

    it('should detect Edge browser', () => {
      const ua =
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36 Edg/134.0.0.0'
      const result = parseUserAgent(ua)

      expect(result.browser.name).toBe('Edge')
      expect(result.browser.version).toBe('134.0.0.0')
      expect(result.browser.major).toBe('134')
      expect(result.isEdge).toBe(true)
      expect(result.isChrome).toBe(false)
    })

    it('should detect Safari browser', () => {
      const ua =
        'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.3.1 Safari/605.1.15'
      const result = parseUserAgent(ua)

      expect(result.browser.name).toBe('Safari')
      expect(result.browser.version).toBe('605.1.15')
      expect(result.browser.major).toBe('605')
      expect(result.isSafari).toBe(true)
      expect(result.isChrome).toBe(false)
    })

    it('should detect Firefox browser', () => {
      const ua =
        'Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:15.0) Gecko/20100101 Firefox/15.0.1'
      const result = parseUserAgent(ua)

      expect(result.browser.name).toBe('Firefox')
      expect(result.browser.version).toBe('15.0.1')
      expect(result.browser.major).toBe('15')
      expect(result.isFirefox).toBe(true)
      expect(result.isChrome).toBe(false)
    })

    it('should detect Opera browser', () => {
      const ua =
        'Opera/9.80 (Windows NT 6.1; U; en) Presto/2.10.229 Version/11.62'
      const result = parseUserAgent(ua)

      expect(result.browser.name).toBe('Opera')
      expect(result.browser.version).toBe('11.62')
      expect(result.browser.major).toBe('11')
      expect(result.isOpera).toBe(true)
    })
  })

  describe('Edge cases', () => {
    it('should handle empty user agent', () => {
      const result = parseUserAgent('')

      expect(result.useragent).toBe('')
      expect(result.isBot).toBe(false)
      expect(result.isMobile).toBe(false)
      expect(result.isDesktop).toBe(false)
      expect(result.isTablet).toBe(false)
    })

    it('should handle malformed user agent', () => {
      const ua = 'Not a real user agent'
      const result = parseUserAgent(ua)

      expect(result.useragent).toBe(ua)
      // Should not throw and should return some default values
      expect(result).toBeDefined()
    })

    it('should handle very long user agent string', () => {
      const ua =
        'Mozilla/5.0 (Linux; Android 14; SM-S928B/DS) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.6099.230 Mobile Safari/537.36'.repeat(
          10,
        )
      const result = parseUserAgent(ua)

      expect(result.useragent).toBe(ua)
      expect(result).toBeDefined()
    })
  })

  describe('Multiple device types', () => {
    it('should not be both mobile and desktop', () => {
      const mobileUA =
        'Mozilla/5.0 (iPhone; CPU iPhone OS 12_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/12.0 Mobile/15E148 Safari/604.1'
      const mobileResult = parseUserAgent(mobileUA)

      expect(mobileResult.isMobile).toBe(true)
      expect(mobileResult.isDesktop).toBe(false)

      const desktopUA =
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36'
      const desktopResult = parseUserAgent(desktopUA)

      expect(desktopResult.isDesktop).toBe(true)
      expect(desktopResult.isMobile).toBe(false)
    })

    it('should not be both tablet and mobile', () => {
      const tabletUA =
        'Mozilla/5.0 (iPad16,3; CPU OS 18_3_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 Tropicana_NJ/5.7.1'
      const tabletResult = parseUserAgent(tabletUA)

      expect(tabletResult.isTablet).toBe(true)
      expect(tabletResult.isMobile).toBe(false)
    })
  })

  describe('Display value edge cases', () => {
    it('should handle bot with no device or browser', () => {
      const ua =
        'Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      expect(result.display).not.toContain('undefined')
      expect(result.display).not.toContain('null')
      expect(result.display).not.toContain('on')
      expect(result.display).toContain('Bot')
    })

    it('should handle user agent with no browser version', () => {
      const ua = 'Mozilla/5.0 (Macintosh; Intel Mac OS X) AppleWebKit Safari'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      expect(result.display).not.toContain('undefined')
      expect(result.display).toContain('Safari on macOS')
    })

    it('should handle user agent with device but no OS', () => {
      const ua = 'Roku/DVP-9.10 (519.10E04111A)'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      expect(result.display).not.toContain('undefined')
      expect(result.display).toContain('Roku')
      expect(result.display).not.toContain('on')
    })

    it('should handle user agent with OS but no version', () => {
      const ua = 'Mozilla/5.0 (Linux; Android) Mobile'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      expect(result.display).toBe('Android Phone')
    })

    it('should handle extremely minimal user agent/curl', () => {
      const ua = 'curl/7.64.1'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      expect(result.display).not.toContain('undefined')
      expect(result.display).not.toContain('on')
      expect(result.display).toBe('Developer Tool')
    })

    it('should handle yaak', () => {
      const ua = 'yaak'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      expect(result.display).not.toContain('undefined')
      expect(result.display).not.toContain('on')
      expect(result.display).toBe('Developer Tool')
    })

    it('should handle user agent with special characters + emoji in device name', () => {
      const ua =
        'Mozilla/5.0 (Linux; Android 10; 🐶 SM-G973F/DS Build/QP1A) 🎈 Chrome/91.0'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      expect(result.display).not.toMatch(/[<>]/)
    })

    it('should handle duplicate information in user agent', () => {
      const ua =
        'Mozilla/5.0 (iPhone; CPU iPhone OS 15_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0 Mobile/15E148 Safari/604.1'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      // Should not duplicate "iPhone" or version numbers
      const displayParts = result.display.split(' ')
      expect(
        displayParts.filter(p => p === 'iPhone').length,
      ).toBeLessThanOrEqual(1)
    })

    it('should handle user agent with version but no browser name', () => {
      const ua = 'Mozilla/5.0 AppleWebKit/537.36 Version/18.0'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      expect(result.display).not.toContain('undefined')
      expect(result.display).toBe('Unknown Device')
    })

    it('should handle user agent with browser but no device or OS', () => {
      const ua = 'Opera/9.80'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      expect(result.display).toContain('Opera')
      expect(result.display).toBe('Opera on Unknown Device')
    })

    it('should handle console with no OS version', () => {
      const ua =
        'Mozilla/5.0 (PlayStation 4) AppleWebKit/537.73 (KHTML, like Gecko)'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      expect(result.display).toContain('PlayStation')
    })

    it('should handle TV device with minimal info', () => {
      const ua = 'Roku4640X/DVP-7.70'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      expect(result.display).toContain('Roku')
      expect(result.display).not.toContain('undefined')
    })

    it('should handle multiple browser identifiers', () => {
      const ua =
        'Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36 Edg/91.0.864.59'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      // Should only show one browser
      expect((result.display.match(/Edge|Chrome/g) || []).length).toBe(1)
    })

    it('should handle user agent with very long version string', () => {
      const ua =
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/91.0.4472.124.10.20.30.40'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      expect(result.display).not.toContain('undefined')
    })

    it('should handle device with parentheses in OS info', () => {
      const ua =
        'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 Safari/605.1.15'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      // Should have properly balanced parentheses
      const openParens = (result.display.match(/\(/g) || []).length
      const closeParens = (result.display.match(/\)/g) || []).length
      expect(openParens).toBe(closeParens)
    })

    it('should handle Apple device without explicit model', () => {
      const ua =
        'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      expect(result.display).toContain('macOS')
      expect(result.display).not.toContain('undefined')
    })

    it('should handle Android device with no model info', () => {
      const ua = 'Mozilla/5.0 (Linux; Android 11) Mobile'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      expect(result.display).toContain('Android')
      expect(result.display).not.toContain('undefined')
    })

    it('should handle tablet with vendor but generic model', () => {
      const ua =
        'Mozilla/5.0 (Linux; Android 12; Lenovo YT-J706X) Chrome/96.0.4664.45'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      expect(result.display).toContain('Lenovo')
    })

    it('should handle e-reader with minimal browser info', () => {
      const ua = 'Mozilla/5.0 (Linux; U; en-US) AppleWebKit/528.5+ Kindle/3.0'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      expect(result.display).toContain('Kindle')
    })

    it('should handle Windows Phone with Microsoft vendor', () => {
      const ua =
        'Mozilla/5.0 (Windows Phone 10.0; Android 6.0.1; Microsoft; Lumia 950)'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      expect(result.display).not.toContain('undefined')
    })

    it('should handle smart TV with complex user agent', () => {
      const ua =
        'Mozilla/5.0 (Linux; Android 9; AFTR) AppleWebKit/537.36 Silk/98.6.10'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      expect(result.display).toContain('Fire TV')
    })

    it('should handle gaming console with Xbox identifier', () => {
      const ua =
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64; Xbox; Xbox Series X) Chrome/48.0'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      expect(result.display).toContain('Xbox')
    })

    it('should handle user agent with only version numbers', () => {
      const ua = 'Mozilla/5.0 (12.3.4; 56.78.90)'
      const result = parseUserAgent(ua)
      expect(result.display).not.toContain('undefined')
      expect(result.display).not.toContain('null')
    })

    it('should handle ChromeOS without device model', () => {
      const ua = 'Mozilla/5.0 (X11; CrOS x86_64 14541.0.0) Chrome/134.0.0.0'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      expect(result.display).toContain('Chrome')
      expect(result.display).toContain('Chrome OS')
    })

    it('should handle nested device identifiers', () => {
      const ua =
        'Mozilla/5.0 (iPhone; CPU iPhone OS 15_0 like Mac OS X) Mobile Safari'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      expect(result.display).toContain('iPhone')
      expect(result.display).toContain('iOS')
    })

    it('should handle bot with model identifier', () => {
      const ua =
        'Mozilla/5.0 (compatible; bingbot/2.0; +http://www.bing.com/bingbot.htm)'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      expect(result.display).not.toContain('undefined')
    })

    it('should handle Samsung device with long model code', () => {
      const ua =
        'Mozilla/5.0 (Linux; Android 14; SM-S928B/DS) Chrome/120.0.6099.230'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      expect(result.display).toContain('Samsung')
    })

    it('should handle device with multiple OS mentions', () => {
      const ua =
        'Mozilla/5.0 (Linux; iPad; CPU OS 15_0 like Mac OS X; Windows XP) AppleWebKit/605.1.15'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      expect(result.os.name).toBe('iPadOS')
      expect(result.display).toBe('iPad (iPadOS 15.0)')
    })

    it('should handle very short user agent', () => {
      const ua = 'Bot'
      const result = parseUserAgent(ua)
      expect(result.display).toBeDefined()
      expect(result.display).not.toContain('undefined')
    })

    it('should handle user agent with Unicode characters', () => {
      const ua = 'Mozilla/5.0 (Linux; Android 10; 中文设备) Chrome/91.0'
      const result = parseUserAgent(ua)
      expect(result.display).toBeTruthy()
      expect(result.display).not.toContain('undefined')
    })
  })
})

export const DeviceVendors = [
  'Apple',
  'Samsung',
  'Google',
  'Huawei',
  'Xiaomi',
  'OnePlus',
  'Motorola',
  'Nokia',
  'Sony',
  'LG',
  'HTC',
  'Lenovo',
  'Asus',
  'Acer',
  'Dell',
  'Microsoft',
  'Nintendo',
  'Amazon',
  'Roku',
] as const

export type DeviceVendor = (typeof DeviceVendors)[number]

export type DeviceInfo = {
  vendor?: DeviceVendor | string
  model?: string
}

const VendorAliases: Array<{
  alias: string
  vendor: DeviceVendor
}> = [
  { alias: 'iPhone', vendor: 'Apple' },
  { alias: 'iPad', vendor: 'Apple' },
  { alias: 'iPod', vendor: 'Apple' },
  { alias: 'Macintosh', vendor: 'Apple' },
  { alias: 'AppleTV', vendor: 'Apple' },
  { alias: 'SAMSUNG', vendor: 'Samsung' },
  { alias: 'SM-', vendor: 'Samsung' },
  { alias: 'Pixel', vendor: 'Google' },
  { alias: 'Nexus', vendor: 'Google' },
  { alias: 'HUAWEI', vendor: 'Huawei' },
  { alias: 'Redmi', vendor: 'Xiaomi' },
  { alias: 'moto', vendor: 'Motorola' },
  { alias: 'PlayStation', vendor: 'Sony' },
  { alias: 'ASUS', vendor: 'Asus' },
  { alias: 'Xbox', vendor: 'Microsoft' },
  { alias: 'XBOX', vendor: 'Microsoft' },
  { alias: 'XBOX_ONE_ED', vendor: 'Microsoft' },
  { alias: 'Windows Phone', vendor: 'Microsoft' },
  { alias: 'Lumia', vendor: 'Microsoft' },
  { alias: 'Kindle', vendor: 'Amazon' },
  { alias: 'KFAPWI', vendor: 'Amazon' },
  { alias: 'KFTHWI', vendor: 'Amazon' },
  { alias: 'KFRASWI', vendor: 'Amazon' },
  { alias: 'AFTT', vendor: 'Amazon' },
  { alias: 'AFTM', vendor: 'Amazon' },
  { alias: 'AFTS', vendor: 'Amazon' },
  { alias: 'AFTB', vendor: 'Amazon' },
  { alias: 'AFTR', vendor: 'Amazon' },
  { alias: 'AFTKRT', vendor: 'Amazon' },
]

const ModelStrings: Array<{
  string: string
  model: string
  vendor?: DeviceVendor
}> = [
  { string: 'iPhone', model: 'iPhone', vendor: 'Apple' },
  { string: 'iPad', model: 'iPad', vendor: 'Apple' },
  { string: 'iPod', model: 'iPod', vendor: 'Apple' },
  { string: 'AppleTV', model: 'Apple TV', vendor: 'Apple' },
  { string: 'Pixel C', model: 'Pixel C', vendor: 'Google' },
  { string: 'Pixel', model: 'Pixel', vendor: 'Google' },
  { string: 'Nexus', model: 'Nexus', vendor: 'Google' },
  { string: 'PlayStation 5', model: 'PlayStation 5', vendor: 'Sony' },
  { string: 'PlayStation 4', model: 'PlayStation 4', vendor: 'Sony' },
  { string: 'PlayStation Vita', model: 'PlayStation Vita', vendor: 'Sony' },
  { string: 'PlayStation', model: 'PlayStation', vendor: 'Sony' },
  { string: 'Xbox Series X', model: 'Xbox Series X', vendor: 'Microsoft' },
  { string: 'Xbox Series S', model: 'Xbox Series S', vendor: 'Microsoft' },
  { string: 'XBOX_ONE_ED', model: 'Xbox One', vendor: 'Microsoft' },
  { string: 'Xbox One', model: 'Xbox One', vendor: 'Microsoft' },
  { string: 'Xbox', model: 'Xbox', vendor: 'Microsoft' },
  { string: 'Nintendo Switch', model: 'Switch', vendor: 'Nintendo' },
  { string: 'WiiU', model: 'Wii U', vendor: 'Nintendo' },
  { string: 'Nintendo 3DS', model: '3DS', vendor: 'Nintendo' },
  { string: 'Kindle', model: 'Kindle', vendor: 'Amazon' },
  { string: 'KFAPWI', model: 'Kindle', vendor: 'Amazon' },
  { string: 'KFTHWI', model: 'Kindle', vendor: 'Amazon' },
  { string: 'KFRASWI', model: 'Kindle', vendor: 'Amazon' },
  { string: 'Fire TV', model: 'Fire TV', vendor: 'Amazon' },
  { string: 'AFTKRT', model: 'Fire TV', vendor: 'Amazon' },
  { string: 'AFTT', model: 'Fire TV', vendor: 'Amazon' },
  { string: 'AFTM', model: 'Fire TV', vendor: 'Amazon' },
  { string: 'AFTS', model: 'Fire TV', vendor: 'Amazon' },
  { string: 'AFTB', model: 'Fire TV', vendor: 'Amazon' },
  { string: 'AFTR', model: 'Fire TV', vendor: 'Amazon' },
  { string: 'Roku', model: 'Roku', vendor: 'Roku' },
  { string: 'CrKey', model: 'Chromecast' },
  { string: 'DE2118', model: 'Nord', vendor: 'OnePlus' },
  { string: 'RPADG', model: 'Pad', vendor: 'Xiaomi' },
  { string: 'YT-J', model: 'Tablet', vendor: 'Lenovo' },
  { string: 'Note', model: 'Note', vendor: 'Samsung' },
  { string: 'SM-X', model: 'Galaxy Tab', vendor: 'Samsung' },
  { string: 'SM-', model: 'Galaxy', vendor: 'Samsung' },
]

const matchVendor = (useragent: string): DeviceVendor | undefined => {
  for (const vendor of DeviceVendors) {
    const index = useragent.indexOf(vendor)
    if (index !== -1) {
      const nextChar = useragent[index + vendor.length]
      if (!nextChar || !/[A-Za-z0-9]/.test(nextChar)) {
        return vendor
      }
    }
  }

  for (const { alias, vendor } of VendorAliases) {
    if (useragent.includes(alias)) {
      return vendor
    }
  }
  return undefined
}

const matchModel = (
  useragent: string,
): { model: string; vendor?: DeviceVendor } | undefined => {
  // Check longer strings first to get more specific matches
  const sortedModels = [...ModelStrings].sort(
    (a, b) => b.string.length - a.string.length,
  )

  for (const { string, model, vendor } of sortedModels) {
    const index = useragent.indexOf(string)
    if (index !== -1) {
      // Special case: don't match "iPhone" if it's part of "iPhone OS"
      if (string === 'iPhone') {
        const afterString = useragent.substring(index + string.length)
        if (afterString.startsWith(' OS')) {
          continue
        }
      }
      return { model, vendor }
    }
  }
  return undefined
}

export const deviceInfo = (useragent: string): DeviceInfo => {
  const device: DeviceInfo = {}

  // Try to match a specific model first
  const modelMatch = matchModel(useragent)
  if (modelMatch) {
    device.model = modelMatch.model
    if (modelMatch.vendor) {
      device.vendor = modelMatch.vendor
    }
  }

  // If we don't have a vendor yet, try to match one
  if (!device.vendor) {
    device.vendor = matchVendor(useragent)
  }

  return device
}

export type Version = string

export const formatVersion = (
  version: string | number,
): Version | undefined => {
  const versionStr = String(version)
  const versionParts = versionStr.replace(/_/g, '.').split('.')
  let formattedVersion: string | undefined = undefined
  versionParts.forEach(part => {
    const partAsNumber = Number(part)
    if (!isNaN(partAsNumber)) {
      if (formattedVersion) {
        formattedVersion += '.'
        formattedVersion += partAsNumber.toString()
      } else {
        formattedVersion = partAsNumber.toString()
      }
    }
  })
  return formattedVersion
}

export const extractVersion = (string: string): Version | undefined => {
  const match = string.match(/([0-9]+(?:[._][0-9]+)*)/)
  if (match) {
    return formatVersion(match[1])
  }
  return undefined
}

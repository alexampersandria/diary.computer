export const formatKey = (key: string): string => {
  return key.charAt(0).toUpperCase() + key.slice(1).replaceAll('_', ' ')
}

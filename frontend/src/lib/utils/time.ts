export const timeAgo = (timestamp: number | string): string => {
  const timeValue =
    typeof timestamp === 'string' ? Date.parse(timestamp) : timestamp
  const now = Date.now()
  const secondsPast = Math.floor((now - timeValue) / 1000)

  if (secondsPast <= 0) {
    return 'Right now'
  }

  if (secondsPast < 60) {
    return `${secondsPast} seconds ago`
  }
  if (secondsPast < 3600) {
    const minutes = Math.floor(secondsPast / 60)
    return `${minutes} minute${minutes > 1 ? 's' : ''} ago`
  }
  if (secondsPast < 86400) {
    const hours = Math.floor(secondsPast / 3600)
    return `${hours} hour${hours > 1 ? 's' : ''} ago`
  }
  if (secondsPast < 2592000) {
    const days = Math.floor(secondsPast / 86400)
    return `${days} day${days > 1 ? 's' : ''} ago`
  }
  if (secondsPast < 31104000) {
    const months = Math.floor(secondsPast / 2592000)
    return `${months} month${months > 1 ? 's' : ''} ago`
  }
  const years = Math.floor(secondsPast / 31104000)
  if (years > 100) {
    return 'A very long time ago'
  }
  return `${years} year${years > 1 ? 's' : ''} ago`
}

export const formatTimestamp = (timestamp: number | string): string => {
  const date =
    typeof timestamp === 'string' ? new Date(timestamp) : new Date(timestamp)
  return date.toLocaleString(undefined, {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour12: false,
    hour: '2-digit',
    minute: '2-digit',
  })
}

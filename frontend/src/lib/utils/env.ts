import { env } from '$env/dynamic/public'
import { page } from '$app/state'

export const API_URL = (path = '') => {
  let url = ''
  if (env.PUBLIC_VITE_API_URL) {
    url = env.PUBLIC_VITE_API_URL
  } else if (page) {
    url = page.url.origin + '/api'
  } else {
    url = window.location.origin + '/api'
  }
  const trimmedPath = path.trim()
  const sanitizedPath = trimmedPath.startsWith('/')
    ? trimmedPath
    : '/' + trimmedPath
  return url + sanitizedPath
}

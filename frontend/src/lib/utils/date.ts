import type { DateObject } from '$lib/types/date'

export const formatDate = (date: Date | string | DateObject): string => {
  if (typeof date === 'object') {
    if ('day' in date) {
      const { year, month, day } = date
      return `${year}-${String(month).padStart(2, '0')}-${String(day).padStart(2, '0')}`
    } else {
      const year = date.getFullYear()
      const month = date.getMonth() + 1
      const day = date.getDate()
      return formatDate({
        year,
        month,
        day,
      })
    }
  } else {
    return formatDate(new Date(date))
  }
}

// #TODO: optimize this so if already formatted don't reformat
export const objectizeDate = (date: Date | string): DateObject => {
  const [year, month, day] = formatDate(date).split('-').map(Number)
  return { year, month, day }
}

import { colorPriority } from '$lib/types/color'
import type { Range } from '$lib/types/range'
import type { CategoryWithTags, Entry } from '$lib/types/log'

// #TODO: this file has gotten too large split it up
// alot of these functions should be moved to their own files
// or to the time utils file

/**
 * Sort categories alphabetically by name, ignoring case.
 * And sorts tags within each category, first by color, then by name.
 * @see /src/lib/types/color.ts colorPriority
 * @param categories - categories to sort
 */
export const sortCategories = (categories: CategoryWithTags[]) => {
  const copy = JSON.parse(JSON.stringify(categories)) as CategoryWithTags[]
  const sortedCategories = copy.sort((a, b) =>
    a.name.localeCompare(b.name, undefined, { sensitivity: 'base' }),
  )

  sortedCategories.forEach(category => {
    category.tags.sort((a, b) => {
      if (a.color === b.color) {
        return a.name.localeCompare(b.name, undefined, { sensitivity: 'base' })
      }
      return colorPriority[b.color] - colorPriority[a.color]
    })
  })

  return sortedCategories
}

/**
 * sorts entries by date first (newest first)
 */
export const sortEntries = (entries: Entry[]): Entry[] => {
  const copy = JSON.parse(JSON.stringify(entries)) as Entry[]
  const sorted = copy.sort((a, b) => {
    return new Date(b.date).getTime() - new Date(a.date).getTime()
  })
  return sorted
}

/**
 * Formats a date string (YYYY-MM-DD) to a more readable format, used for detailed views
 * @param dateString - YYYY-MM-DD
 * @returns formatted date string like "Monday, January 1, 2023"
 */
export const fullDate = (dateString: string) => {
  if (!isValidDate(dateString)) {
    return null
  }

  const date = new Date(dateString)

  const weekday = date.toLocaleDateString(undefined, { weekday: 'long' })
  const month = date.toLocaleDateString(undefined, { month: 'long' })
  const day = date.getDate()
  const year = date.getFullYear()

  if (year < 100) {
    return `${weekday}, ${month} ${day}, Year ${year}`
  }

  return `${weekday}, ${month} ${day}, ${year}`
}

export const isValidDate = (dateString: string) => {
  const regex = /^\d{4}\-(0[1-9]|1[012])\-(0[1-9]|[12][0-9]|3[01])$/
  return regex.test(dateString)
}

/**
 * gives the current date (YYYY-MM-DD) for the user with their local time
 * note for this definition, the current date starts at 4am, this is because quite a lot of people go to bed after midnight
 * the day start/end time will be customizable in the future
 * @returns date string YYYY-MM-DD
 */
export const currentDate = () => {
  const now = new Date()
  if (now.getHours() < 4) {
    now.setDate(now.getDate() - 1)
  }
  return now.toISOString().split('T')[0]
}

/**
 * returns currentDate but in object form
 * @returns object with year, month, day as numbers
 */
export const currentDateObject = () => {
  const date = currentDate()
  const [year, month, day] = date.split('-').map(Number)
  return { year, month, day }
}

/**
 * returns an object with the first and last date of the given month
 * defaults to current month if no parameters given
 * @param year - The year of the month
 * @param month - The month (1-12)
 * @returns object with first and last date strings YYYY-MM-DD
 */
export const monthDateRange = (year?: number, month?: number) => {
  if (!year || !month) {
    const current = currentDateObject()
    year = current.year
    month = current.month
  }

  const days: number[] = []
  calendarDaysInMonth(year, month).forEach(week => {
    week.forEach(day => {
      if (day) {
        days.push(day)
      }
    })
  })

  const firstDate = `${year}-${String(month).padStart(2, '0')}-${String(
    days[0],
  ).padStart(2, '0')}`
  const lastDate = `${year}-${String(month).padStart(2, '0')}-${String(
    days[days.length - 1],
  ).padStart(2, '0')}`

  return { firstDate, lastDate }
}

/**
 * returns an object with the first and last date of the given year
 * defaults to current year
 * @param year - the year to get the range for
 * @returns object with first and last date strings YYYY-MM-DD
 */
export const yearDateRange = (year?: number) => {
  if (!year) {
    year = currentDateObject().year
  }

  const firstDate = `${year}-01-01`
  const lastDate = `${year}-12-31`

  return { firstDate, lastDate }
}

/**
 * gets the current year and month as numbers using the currentDate function
 * @returns the current year and month as numbers using the currentDate function
 */
export const calendarDefaults = (): { year: number; month: number } => {
  const date = currentDate()
  const [year, month] = date.split('-').map(Number)
  return { year, month }
}

export type MonthFormatOptions = { short?: boolean }
export const formatMonth = (month: number, options?: MonthFormatOptions) => {
  let value = ''
  switch (month) {
    case 1:
      value = 'January'
      break
    case 2:
      value = 'February'
      break
    case 3:
      value = 'March'
      break
    case 4:
      value = 'April'
      break
    case 5:
      value = 'May'
      break
    case 6:
      value = 'June'
      break
    case 7:
      value = 'July'
      break
    case 8:
      value = 'August'
      break
    case 9:
      value = 'September'
      break
    case 10:
      value = 'October'
      break
    case 11:
      value = 'November'
      break
    case 12:
      value = 'December'
      break
    default:
      value = ''
  }

  if (options?.short) {
    value = value.slice(0, 3)
  }

  return value
}

/**
 * Array of month values for select inputs
 */
export const monthValues = Array.from({ length: 12 }, (_, i) => {
  return { label: formatMonth(i + 1), value: i + 1 }
})

/**
 * Array of month values for select inputs, short format
 */
export const monthValuesShort = Array.from({ length: 12 }, (_, i) => {
  return { label: formatMonth(i + 1, { short: true }), value: i + 1 }
})

/**
 * Array of year values for select inputs
 * Goes back 100 years from current year
 * and 1 year into the future
 */
export const yearValues = Array.from({ length: 102 }, (_, i) => {
  const currentYear = currentDateObject().year
  return { label: String(currentYear - 100 + i), value: currentYear - 100 + i }
})

/**
 * Returns an array, with the weeks of the month
 * each week is an array of either null (for days outside the month) or the day number
 * week starts on monday
 * @param year - The year of the month
 * @param month - The month (1-12)
 */
export const calendarDaysInMonth = (year: number, month: number) => {
  // #TODO: this whole thing is kinda stupid, think of a better way to do it

  const daysInMonth = new Date(year, month, 0).getDate()
  const firstDayOfMonth = new Date(year, month - 1, 1).getDay()
  const weeks: (number | null)[][] = []
  let currentWeek: (number | null)[] = []

  // Fill initial nulls for days before the first of the month
  for (let i = 1; i < firstDayOfMonth; i++) {
    currentWeek.push(null)
  }

  for (let day = 1; day <= daysInMonth; day++) {
    currentWeek.push(day)
    if (currentWeek.length === 7) {
      weeks.push(currentWeek)
      currentWeek = []
    }
  }

  // Fill remaining nulls for days after the last of the month
  while (currentWeek.length < 7 && currentWeek.length > 0) {
    currentWeek.push(null)
  }
  if (currentWeek.length > 0) {
    weeks.push(currentWeek)
  }
  // always return 6 weeks for consistent calendar display
  while (weeks.length < 6) {
    const emptyWeek: (number | null)[] = [null]
    weeks.push(emptyWeek)
  }

  return weeks
}

/**
 * Returns an array of dates in the specified range.
 * @param startDate - The start date of the range (YYYY-MM-DD)
 * @param endDate - The end date of the range (YYYY-MM-DD)
 * @returns An array of dates in the range (YYYY-MM-DD)
 */
export const datesInRange = (startDate: string, endDate: string) => {
  const start = new Date(startDate)
  const end = new Date(endDate)
  const dates: string[] = []

  for (let dt = new Date(start); dt <= end; dt.setDate(dt.getDate() + 1)) {
    dates.push(dt.toISOString().split('T')[0])
  }

  if (!dates.includes(endDate)) {
    dates.push(endDate)
  }

  return dates
}

/**
 * returns an array of weeks in a year, each week is an array of date strings (YYYY-MM-DD)
 * weeks start on monday
 * @param year - the year to get the weeks for
 */
export const weeksInYear = (year: number) => {
  const daysInYear =
    year % 4 === 0 && (year % 100 !== 0 || year % 400 === 0) ? 366 : 365
  const days = Array.from({ length: daysInYear }, (_, i) => {
    const date = new Date(Date.UTC(year, 0, i + 1))
    return date.toISOString().split('T')[0]
  })

  const weeks: string[][] = []
  let weekIndex = 0

  days.forEach(day => {
    const isMonday = new Date(day).getDay() === 1
    if (isMonday) {
      if (weeks.length !== 0) {
        weekIndex++
      }
    }

    if (weeks[weekIndex]) {
      weeks[weekIndex].push(day)
    } else {
      weeks[weekIndex] = [day]
    }
  })

  return weeks
}

/**
 * checks if a date is between two other dates (inclusive)
 * @param date - date to check (YYYY-MM-DD)
 * @param range - object with startDate and endDate (YYYY-MM-DD)
 * @returns true if date is between startDate and endDate, false otherwise
 */
export const isDateInRange = (date: string, range: Range<string>) => {
  const checkDate = new Date(date)
  const startDate = new Date(range.from)
  const endDate = new Date(range.to)
  return checkDate >= startDate && checkDate <= endDate
}

/**
 * sorts a date range so that the from date is before the to date
 */
export const sortDateRange = (range: Range<string>) => {
  const fromDate = new Date(range.from)
  const toDate = new Date(range.to)
  if (fromDate <= toDate) {
    return range
  } else {
    return { from: range.to, to: range.from }
  }
}

/**
 * determines whether a date is closest to the start or end of a date range
 */
export const dateClosestToRangeEdge = (date: string, range: Range<string>) => {
  const checkDate = new Date(date)
  const fromDate = new Date(range.from)
  const toDate = new Date(range.to)

  if (checkDate <= fromDate) {
    return 'from'
  } else if (checkDate >= toDate) {
    return 'to'
  }

  const distanceToFrom = Math.abs(checkDate.getTime() - fromDate.getTime())
  const distanceToTo = Math.abs(checkDate.getTime() - toDate.getTime())

  return distanceToFrom <= distanceToTo ? 'from' : 'to'
}

/**
 * returns YYYY-MM-DD date string from a timestamp
 */
export const timestampToDate = (timestamp: number | string) => {
  const date = new Date(timestamp)
  return date.toISOString().split('T')[0]
}

/**
 * returns the following date string (YYYY-MM-DD) for a given date
 */
export const nextDate = (dateString: string) => {
  const date = new Date(dateString)
  date.setDate(date.getDate() + 1)
  return date.toISOString().split('T')[0]
}

/**
 * returns the previous date string (YYYY-MM-DD) for a given date
 */
export const previousDate = (dateString: string) => {
  const date = new Date(dateString)
  date.setDate(date.getDate() - 1)
  return date.toISOString().split('T')[0]
}

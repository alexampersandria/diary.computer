import type { Range } from '../range'

/**
 * mode of determining the functionality of the calendar
 */
export type CalendarMode = 'navigation' | 'datepicker' | 'rangepicker'

/**
 * datepicker value
 * either a single date or a date range
 */
export type CalendarValue = DatepickerValue | RangepickerValue

export type DatepickerValue = string | undefined
export type RangepickerValue = Range<DatepickerValue>

/**
 * props for the Calendar component
 */
export type CalendarProps = {
  mode?: CalendarMode
  value?: CalendarValue
  from?: DatepickerValue
  to?: DatepickerValue
  month?: number
  year?: number
  usedata?: boolean
  onchange?: () => void
}

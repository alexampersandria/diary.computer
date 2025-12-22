export type NumberFormatOptions = {
  decimals?: boolean
}

export const defaultNumberFormatOptions: NumberFormatOptions = {
  decimals: undefined,
}

export const formatNumber = (number: number, options?: NumberFormatOptions) => {
  const optionsWithDefaults = {
    ...defaultNumberFormatOptions,
    ...options,
  }

  const whole = Math.floor(number)
  // format whole with thousands separator ","
  let value = whole.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ',')
  const fraction = (number - whole).toFixed(2).substring(2)

  if (
    optionsWithDefaults.decimals === true ||
    (optionsWithDefaults.decimals === undefined && fraction !== '00')
  ) {
    // format fraction with two decimal places
    value = `${value}.${fraction}`
  } else if (optionsWithDefaults.decimals === false) {
    // round number to nearest whole number
    value = Math.round(number)
      .toString()
      .replace(/\B(?=(\d{3})+(?!\d))/g, ',')
  }

  return value
}

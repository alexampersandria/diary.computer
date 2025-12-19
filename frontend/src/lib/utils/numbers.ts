export const formatNumber = (number: number, decimals?: boolean) => {
  const whole = Math.floor(number)
  // format whole with thousands separator ","
  let value = whole.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ',')
  const fraction = (number - whole).toFixed(2).substring(2)

  if (decimals === true || (decimals === undefined && fraction !== '00')) {
    // format fraction with two decimal places
    value = `${value}.${fraction}`
  }

  return value
}

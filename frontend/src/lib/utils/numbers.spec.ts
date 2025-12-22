import { expect, it, describe } from 'vitest'
import { formatNumber } from './numbers'

describe('formatNumber', () => {
  it('should format number with two decimal places by default', () => {
    const result = formatNumber(1234.5)
    expect(result).toBe('1,234.50')
  })

  it('should format number with no decimal places', () => {
    const result = formatNumber(1234.5, {
      decimals: false,
    })
    expect(result).toBe('1,235')
  })

  it('should format number with two decimal places when decimals is true', () => {
    const decimalvalue = formatNumber(1234.5, {
      decimals: true,
    })
    const nondecimalvalue = formatNumber(1234, {
      decimals: true,
    })
    expect(decimalvalue).toBe('1,234.50')
    expect(nondecimalvalue).toBe('1,234.00')
  })

  it('should format number with two decimal places when decimals is undefined and there are decimal digits', () => {
    const result = formatNumber(1234.56, {
      decimals: undefined,
    })
    expect(result).toBe('1,234.56')
  })

  it('should format number with no decimal places when decimals is undefined and there are no decimal digits', () => {
    const nodecimals = formatNumber(1234, {
      decimals: undefined,
    })
    const decimals = formatNumber(1234.1, {
      decimals: undefined,
    })
    expect(nodecimals).toBe('1,234')
    expect(decimals).toBe('1,234.10')
  })

  it('should format 0.1 correctly', () => {
    const result = formatNumber(0.1)
    expect(result).toBe('0.10')
  })
})

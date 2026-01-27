import { describe, it, expect } from 'vitest'

// Simple test to verify the testing setup works
describe('Basic Tests', () => {
  it('should pass a simple test', () => {
    expect(1 + 1).toBe(2)
  })

  it('should verify string contains expected text', () => {
    const message = 'Welcome to MiniDebet'
    expect(message).toContain('MiniDebet')
  })
})
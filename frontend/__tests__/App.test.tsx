import { render, screen } from '@testing-library/react'
import App from '../src/App'

describe('App', () => {
  test('renders welcome message', () => {
    render(<App />)
    const welcomeElement = screen.getByText(/Welcome to MiniDebet/i)
    expect(welcomeElement).toBeInTheDocument()
  })

  test('renders description', () => {
    render(<App />)
    const descriptionElement = screen.getByText(/German invoicing solution is loading/i)
    expect(descriptionElement).toBeInTheDocument()
  })
})
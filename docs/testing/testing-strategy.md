# Testing Strategy

## Overview

MiniDebet employs a comprehensive testing strategy covering unit tests, integration tests, and end-to-end tests to ensure code quality and reliability. Tests are organized by component and follow industry best practices.

## Test Organization

```
minidebet/
├── backend/
│   ├── tests/
│   │   ├── unit/
│   │   │   ├── models/
│   │   │   ├── handlers/
│   │   │   └── auth/
│   │   ├── integration/
│   │   │   ├── api_tests.rs
│   │   │   └── database_tests.rs
│   │   └── integration_test.rs
├── frontend/
│   ├── __tests__/
│   │   ├── components/
│   │   ├── pages/
│   │   └── services/
│   └── __mocks__/
└── shared/
    └── __tests__/
```

## Backend Testing (Rust)

### Unit Tests

Located in the same files as the code they test, using Rust's built-in testing framework.

**Example Model Test:**
```rust
// src/models/user.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(
            "test@example.com".to_string(),
            "hashed_password".to_string(),
            Some("John".to_string()),
            Some("Doe".to_string()),
            Some("Test Company".to_string()),
            Some("DE123456789".to_string()),
        );

        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.first_name, Some("John".to_string()));
        assert!(!user.id.is_empty());
    }

    #[test]
    fn test_new_user_builder() {
        let new_user = NewUser::new(
            "test@example.com".to_string(),
            "password123".to_string(),
            Some("Jane".to_string()),
            Some("Smith".to_string()),
            Some("Another Company".to_string()),
            Some("DE987654321".to_string()),
        );

        assert_eq!(new_user.email, "test@example.com");
        assert_eq!(new_user.password, "password123");
    }
}
```

**Running Unit Tests:**
```bash
cd backend
cargo test
# Or for specific modules:
cargo test models::user
```

### Integration Tests

Separate test files that test the interaction between components.

**API Integration Test:**
```rust
// tests/integration/api_tests.rs
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn test_user_registration() {
    let app = create_test_app().await;
    
    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/api/users")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "email": "test@example.com",
                        "password": "password123",
                        "first_name": "Test",
                        "last_name": "User"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let response_body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(response_body["email"], "test@example.com");
}
```

**Database Integration Test:**
```rust
// tests/integration/database_tests.rs
use sqlx::SqlitePool;

#[sqlx::test]
async fn test_user_crud_operations(pool: SqlitePool) {
    // Test user creation
    let user = create_test_user(&pool).await;
    assert!(!user.id.is_empty());

    // Test user retrieval
    let retrieved_user = get_user_by_email(&pool, &user.email).await.unwrap();
    assert_eq!(retrieved_user.email, user.email);

    // Test user update
    update_user_name(&pool, &user.id, "Updated Name").await;
    let updated_user = get_user_by_id(&pool, &user.id).await.unwrap();
    assert_eq!(updated_user.first_name, Some("Updated Name".to_string()));

    // Test user deletion
    delete_user(&pool, &user.id).await;
    let deleted_user = get_user_by_id(&pool, &user.id).await;
    assert!(deleted_user.is_none());
}
```

**Running Integration Tests:**
```bash
cd backend
cargo test --test integration_test
```

## Frontend Testing (TypeScript/Jest)

### Unit Tests

Component and utility function testing using Jest and React Testing Library.

**Component Test:**
```typescript
// frontend/__tests__/components/LoginForm.test.tsx
import { render, screen, fireEvent } from '@testing-library/react';
import LoginForm from '../../src/components/LoginForm';

describe('LoginForm', () => {
  const mockOnRegisterClick = jest.fn();

  beforeEach(() => {
    mockOnRegisterClick.mockClear();
  });

  test('renders login form correctly', () => {
    render(<LoginForm onRegisterClick={mockOnRegisterClick} />);
    
    expect(screen.getByLabelText(/email/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/password/i)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /sign in/i })).toBeInTheDocument();
  });

  test('calls onRegisterClick when register link is clicked', () => {
    render(<LoginForm onRegisterClick={mockOnRegisterClick} />);
    
    const registerLink = screen.getByText(/don't have an account/i);
    fireEvent.click(registerLink);
    
    expect(mockOnRegisterClick).toHaveBeenCalledTimes(1);
  });

  test('submits form with valid data', async () => {
    const mockSubmit = jest.fn();
    render(<LoginForm onRegisterClick={mockOnRegisterClick} />);
    
    fireEvent.change(screen.getByLabelText(/email/i), {
      target: { value: 'test@example.com' }
    });
    fireEvent.change(screen.getByLabelText(/password/i), {
      target: { value: 'password123' }
    });
    
    fireEvent.click(screen.getByRole('button', { name: /sign in/i }));
    
    // Wait for async operations
    await screen.findByText(/signing in/i);
  });
});
```

**Hook Test:**
```typescript
// frontend/__tests__/hooks/useAuth.test.ts
import { renderHook, act } from '@testing-library/react';
import { useAuth } from '../../src/contexts/AuthContext';

describe('useAuth', () => {
  test('initial state is correct', () => {
    const { result } = renderHook(() => useAuth());
    
    expect(result.current.isAuthenticated).toBe(false);
    expect(result.current.isLoading).toBe(true);
    expect(result.current.user).toBeNull();
  });

  test('login function works correctly', async () => {
    const { result } = renderHook(() => useAuth());
    
    await act(async () => {
      await result.current.login('test@example.com', 'password123');
    });
    
    expect(result.current.isAuthenticated).toBe(true);
    expect(result.current.user).toEqual({
      id: expect.any(String),
      email: 'test@example.com'
    });
  });
});
```

### Integration Tests

Testing component interactions and API integrations.

**API Service Test:**
```typescript
// frontend/__tests__/services/api.test.ts
import { api } from '../../src/services/api';
import { login, register } from '../../src/services/auth';

jest.mock('../../src/services/api');

describe('Auth Services', () => {
  beforeEach(() => {
    (api.post as jest.Mock).mockClear();
  });

  test('login service calls API correctly', async () => {
    const mockResponse = { 
      data: { 
        token: 'jwt-token', 
        user: { id: '1', email: 'test@example.com' } 
      } 
    };
    (api.post as jest.Mock).mockResolvedValue(mockResponse);

    const result = await login('test@example.com', 'password123');

    expect(api.post).toHaveBeenCalledWith('/auth/login', {
      email: 'test@example.com',
      password: 'password123'
    });
    expect(result).toEqual(mockResponse.data);
  });

  test('register service handles errors', async () => {
    const mockError = new Error('Email already exists');
    (api.post as jest.Mock).mockRejectedValue(mockError);

    await expect(register({
      email: 'test@example.com',
      password: 'password123'
    })).rejects.toThrow('Email already exists');
  });
});
```

**Running Frontend Tests:**
```bash
cd frontend
npm run test
# Or for specific tests:
npm run test -- LoginForm.test.tsx
```

## Shared Package Testing

Testing shared TypeScript interfaces and utilities.

```typescript
// shared/__tests__/index.test.ts
import { User, Client, Invoice } from '../src/index';

describe('Shared Types', () => {
  test('User interface validation', () => {
    const user: User = {
      id: 'user-123',
      email: 'test@example.com',
      firstName: 'John',
      lastName: 'Doe',
      companyName: 'Test Company'
    };

    expect(user.email).toBe('test@example.com');
    expect(user.id).toBe('user-123');
  });

  test('Client interface validation', () => {
    const client: Client = {
      id: 'client-123',
      userId: 'user-123',
      name: 'Acme Corp',
      email: 'contact@acme.com',
      company: 'Acme Corporation'
    };

    expect(client.name).toBe('Acme Corp');
  });
});
```

## Test Data Management

### Test Fixtures

```typescript
// frontend/__tests__/fixtures/users.ts
export const testUser = {
  id: 'test-user-id',
  email: 'test@example.com',
  firstName: 'Test',
  lastName: 'User',
  companyName: 'Test Company GmbH'
};

export const testClient = {
  id: 'test-client-id',
  userId: 'test-user-id',
  name: 'Acme Corporation',
  email: 'billing@acme.com',
  company: 'Acme Corporation'
};

export const testInvoice = {
  id: 'test-invoice-id',
  clientId: 'test-client-id',
  invoiceNumber: 'INV-2024-001',
  issueDate: '2024-01-15',
  dueDate: '2024-02-15',
  totalAmount: 1725.50,
  status: 'draft'
};
```

### Mock Data Generation

```typescript
// frontend/__tests__/utils/generateMockData.ts
export const generateMockUsers = (count: number) => {
  return Array.from({ length: count }, (_, i) => ({
    id: `user-${i + 1}`,
    email: `user${i + 1}@example.com`,
    firstName: `First${i + 1}`,
    lastName: `Last${i + 1}`,
    companyName: `Company ${i + 1} GmbH`
  }));
};

export const generateMockClients = (userId: string, count: number) => {
  return Array.from({ length: count }, (_, i) => ({
    id: `client-${i + 1}`,
    userId,
    name: `Client ${i + 1}`,
    email: `client${i + 1}@example.com`,
    company: `Client Company ${i + 1}`
  }));
};
```

## Continuous Integration

### GitHub Actions Workflow

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    # Backend tests
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Run backend tests
      run: |
        cd backend
        cargo test --verbose
        
    # Frontend tests
    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
        
    - name: Install frontend dependencies
      run: |
        cd frontend
        npm ci
        
    - name: Run frontend tests
      run: |
        cd frontend
        npm run test -- --coverage
        
    # Shared package tests
    - name: Test shared package
      run: |
        cd shared
        npm ci
        npm run test
```

## Test Coverage

### Coverage Goals
- **Unit Tests**: 80%+ coverage
- **Integration Tests**: 70%+ coverage
- **Critical Paths**: 100% coverage

### Coverage Reports

**Backend Coverage:**
```bash
cd backend
cargo tarpaulin --out Html
# Generates coverage report in tarpaulin-report.html
```

**Frontend Coverage:**
```bash
cd frontend
npm run test -- --coverage --watchAll=false
# Reports available in coverage/ directory
```

## Test Best Practices

### Backend
1. Use descriptive test names
2. Test edge cases and error conditions
3. Mock external dependencies
4. Use test databases for integration tests
5. Follow AAA pattern (Arrange, Act, Assert)

### Frontend
1. Test user interactions, not implementation details
2. Use React Testing Library queries
3. Mock API calls and context providers
4. Test accessibility
5. Snapshot testing for UI components

### General
1. Keep tests independent and isolated
2. Use factories for test data creation
3. Run tests frequently during development
4. Maintain test documentation
5. Refactor tests along with production code

## Debugging Tests

### Backend Debugging
```bash
# Run specific test with output
cargo test test_name -- --nocapture

# Run tests with logging
RUST_LOG=debug cargo test

# Debug with debugger
cargo test -- --test-threads=1
```

### Frontend Debugging
```bash
# Run tests in watch mode
npm run test -- --watch

# Debug specific test file
npm run test -- FileName.test.tsx

# Run with verbose output
npm run test -- --verbose
```

## Performance Testing

### Load Testing Setup
```typescript
// tests/performance/load-test.js
import { check, sleep } from 'k6';
import http from 'k6/http';

export const options = {
  stages: [
    { duration: '30s', target: 100 },  // Ramp up to 100 users
    { duration: '1m', target: 100 },   // Stay at 100 users
    { duration: '30s', target: 0 },    // Ramp down
  ],
};

export default function () {
  const res = http.get('http://localhost:3000/health');
  check(res, {
    'status is 200': (r) => r.status === 200,
    'response time < 200ms': (r) => r.timings.duration < 200,
  });
  sleep(1);
}
```

Run with: `k6 run load-test.js`

## Security Testing

### Automated Security Scans
- Dependency vulnerability scanning
- Static code analysis
- Penetration testing scripts
- Authentication/authorization testing

This comprehensive testing strategy ensures MiniDebet maintains high code quality and reliability throughout development.
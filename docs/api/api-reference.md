# API Documentation

## Base URL

**Development**: `http://localhost:3000/api` (proxied to Cloudflare Workers)
**Production**: `https://minidebet.jamshaidnasar.workers.dev/api`

## Authentication

All protected endpoints require a valid JWT token in the Authorization header:

```sh
Authorization: Bearer <your-jwt-token>
```

### Error Responses

401 Unauthorized

```json
{
  "error": "Unauthorized",
  "message": "Invalid or missing authentication token"
}
```

403 Forbidden

```json
{
  "error": "Forbidden",
  "message": "Insufficient permissions"
}
```

## User Management

### Register User

**POST** `/api/users`

Creates a new user account.

**Request Body:**

```json
{
  "email": "user@example.com",
  "password": "securepassword123",
  "first_name": "Max",
  "last_name": "Mustermann",
  "company_name": "My Company GmbH",
  "tax_id": "DE123456789"
}
```

**Success Response (201 Created):**

```json
{
  "id": "user-uuid",
  "email": "user@example.com",
  "first_name": "Max",
  "last_name": "Mustermann",
  "company_name": "My Company GmbH"
}
```

**Error Responses:**

- 400 Bad Request: Invalid input data
- 409 Conflict: Email already exists

### User Login

**POST** `/api/auth/login`

Authenticate user and receive JWT token.

**Request Body:**

```json
{
  "email": "user@example.com",
  "password": "securepassword123"
}
```

**Success Response (200 OK):**

```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "user-uuid",
    "email": "user@example.com",
    "first_name": "Max",
    "last_name": "Mustermann",
    "company_name": "My Company GmbH"
  }
}
```

**Error Responses:**

- 400 Bad Request: Missing credentials
- 401 Unauthorized: Invalid credentials

### Get Current User

**GET** `/api/users/me`

Retrieve authenticated user's profile information.

**Headers:**

```sh
Authorization: Bearer <jwt-token>
```

**Success Response (200 OK):**

```json
{
  "id": "user-uuid",
  "email": "user@example.com",
  "first_name": "Max",
  "last_name": "Mustermann",
  "company_name": "My Company GmbH",
  "tax_id": "DE123456789",
  "created_at": "2024-01-15T10:30:00Z"
}
```

### Update User Profile

**PUT** `/api/users/me`

Update authenticated user's profile information.

**Headers:**

```sh
Authorization: Bearer <jwt-token>
```

**Request Body:**

```json
{
  "first_name": "Maximilian",
  "last_name": "Mustermann",
  "company_name": "Updated Company GmbH",
  "tax_id": "DE987654321"
}
```

**Success Response (200 OK):**

```json
{
  "id": "user-uuid",
  "email": "user@example.com",
  "first_name": "Maximilian",
  "last_name": "Mustermann",
  "company_name": "Updated Company GmbH",
  "tax_id": "DE987654321"
}
```

## Client Management

### Create Client

**POST** `/api/clients`

Create a new client for the authenticated user.

**Headers:**

```sh
Authorization: Bearer <jwt-token>
```

**Request Body:**

```json
{
  "name": "Acme Corporation",
  "email": "billing@acme.com",
  "company": "Acme Corporation",
  "street": "Musterstraße 123",
  "city": "Berlin",
  "postal_code": "10115",
  "country": "DE",
  "vat_number": "DE123456789"
}
```

**Success Response (201 Created):**

```json
{
  "id": "client-uuid",
  "user_id": "user-uuid",
  "name": "Acme Corporation",
  "email": "billing@acme.com",
  "company": "Acme Corporation",
  "street": "Musterstraße 123",
  "city": "Berlin",
  "postal_code": "10115",
  "country": "DE",
  "vat_number": "DE123456789",
  "created_at": "2024-01-15T10:30:00Z"
}
```

### List Clients

**GET** `/api/clients`

Retrieve all clients for the authenticated user.

**Headers:**

```sh
Authorization: Bearer <jwt-token>
```

**Query Parameters:**

- `page` (optional): Page number (default: 1)
- `limit` (optional): Items per page (default: 20, max: 100)

**Success Response (200 OK):**

```json
{
  "clients": [
    {
      "id": "client-uuid",
      "name": "Acme Corporation",
      "email": "billing@acme.com",
      "company": "Acme Corporation",
      "created_at": "2024-01-15T10:30:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 1,
    "total_pages": 1
  }
}
```

### Get Client Details

**GET** `/api/clients/{id}`

Retrieve specific client information.

**Headers:**

```sh
Authorization: Bearer <jwt-token>
```

**Success Response (200 OK):**

```json
{
  "id": "client-uuid",
  "user_id": "user-uuid",
  "name": "Acme Corporation",
  "email": "billing@acme.com",
  "company": "Acme Corporation",
  "street": "Musterstraße 123",
  "city": "Berlin",
  "postal_code": "10115",
  "country": "DE",
  "vat_number": "DE123456789",
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-15T10:30:00Z"
}
```

### Update Client

**PUT** `/api/clients/{id}`

Update client information.

**Headers:**

```sh
Authorization: Bearer <jwt-token>
```

**Request Body:**

```json
{
  "name": "Acme Corporation GmbH",
  "email": "accounts@acme.com",
  "company": "Acme Corporation GmbH",
  "street": "Neue Straße 456",
  "city": "Hamburg",
  "postal_code": "20095",
  "country": "DE",
  "vat_number": "DE987654321"
}
```

### Delete Client

**DELETE** `/api/clients/{id}`

Delete a client and all associated invoices.

**Headers:**

```sh
Authorization: Bearer <jwt-token>
```

Success Response (204 No Content)

## Invoice Management

### Create Invoice

**POST** `/api/invoices`

Create a new invoice for a client.

**Headers:**

```sh
Authorization: Bearer <jwt-token>
```

**Request Body:**

```json
{
  "client_id": "client-uuid",
  "issue_date": "2024-01-15",
  "due_date": "2024-02-15",
  "currency": "EUR",
  "notes": "Vielen Dank für Ihren Auftrag",
  "items": [
    {
      "description": "Webentwicklung",
      "quantity": 10,
      "unit_price": 85.00
    },
    {
      "description": "Beratung",
      "quantity": 5,
      "unit_price": 120.00
    }
  ]
}
```

**Success Response (201 Created):**

```json
{
  "id": "invoice-uuid",
  "invoice_number": "INV-2024-001",
  "client_id": "client-uuid",
  "issue_date": "2024-01-15",
  "due_date": "2024-02-15",
  "currency": "EUR",
  "subtotal": 1450.00,
  "tax_rate": 19.0,
  "tax_amount": 275.50,
  "total_amount": 1725.50,
  "status": "draft",
  "notes": "Vielen Dank für Ihren Auftrag",
  "items": [
    {
      "id": "item-uuid-1",
      "description": "Webentwicklung",
      "quantity": 10,
      "unit_price": 85.00,
      "total_price": 850.00
    },
    {
      "id": "item-uuid-2",
      "description": "Beratung",
      "quantity": 5,
      "unit_price": 120.00,
      "total_price": 600.00
    }
  ],
  "created_at": "2024-01-15T10:30:00Z"
}
```

### List Invoices

**GET** `/api/invoices`

Retrieve all invoices for the authenticated user.

**Headers:**

```sh
Authorization: Bearer <jwt-token>
```

**Query Parameters:**

- `status` (optional): Filter by status (draft, sent, paid, overdue)
- `client_id` (optional): Filter by client
- `page` (optional): Page number (default: 1)
- `limit` (optional): Items per page (default: 20)

**Success Response (200 OK):**

```json
{
  "invoices": [
    {
      "id": "invoice-uuid",
      "invoice_number": "INV-2024-001",
      "client_name": "Acme Corporation",
      "issue_date": "2024-01-15",
      "due_date": "2024-02-15",
      "total_amount": 1725.50,
      "currency": "EUR",
      "status": "draft",
      "created_at": "2024-01-15T10:30:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 1,
    "total_pages": 1
  }
}
```

### Get Invoice Details

**GET** `/api/invoices/{id}`

Retrieve detailed invoice information including line items.

**Headers:**

```sh
Authorization: Bearer <jwt-token>
```

**Success Response (200 OK):**

```json
{
  "id": "invoice-uuid",
  "invoice_number": "INV-2024-001",
  "client": {
    "id": "client-uuid",
    "name": "Acme Corporation",
    "email": "billing@acme.com",
    "company": "Acme Corporation"
  },
  "issue_date": "2024-01-15",
  "due_date": "2024-02-15",
  "currency": "EUR",
  "subtotal": 1450.00,
  "tax_rate": 19.0,
  "tax_amount": 275.50,
  "total_amount": 1725.50,
  "status": "draft",
  "notes": "Vielen Dank für Ihren Auftrag",
  "items": [
    {
      "id": "item-uuid-1",
      "description": "Webentwicklung",
      "quantity": 10,
      "unit_price": 85.00,
      "total_price": 850.00
    }
  ],
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-15T10:30:00Z"
}
```

### Update Invoice

**PUT** `/api/invoices/{id}`

Update invoice details and line items.

**Headers:**

```sh
Authorization: Bearer <jwt-token>
```

**Request Body:**

```json
{
  "due_date": "2024-02-28",
  "notes": "Aktualisierte Zahlungsbedingungen",
  "items": [
    {
      "id": "item-uuid-1",
      "description": "Webentwicklung",
      "quantity": 12,
      "unit_price": 85.00
    }
  ]
}
```

### Delete Invoice

**DELETE** `/api/invoices/{id}`

Delete an invoice (only draft invoices can be deleted).

**Headers:**

```sh
Authorization: Bearer <jwt-token>
```

Success Response (204 No Content)

### Send Invoice

**POST** `/api/invoices/{id}/send`

Mark invoice as sent and send to client.

**Headers:**

```sh
Authorization: Bearer <jwt-token>
```

**Success Response (200 OK):**

```json
{
  "id": "invoice-uuid",
  "status": "sent",
  "sent_at": "2024-01-15T11:00:00Z"
}
```

### Mark Invoice as Paid

**POST** `/api/invoices/{id}/pay`

Mark invoice as paid.

**Headers:**

```sh
Authorization: Bearer <jwt-token>
```

**Request Body:**

```json
{
  "payment_date": "2024-01-20",
  "payment_method": "bank_transfer"
}
```

**Success Response (200 OK):**

```json
{
  "id": "invoice-uuid",
  "status": "paid",
  "paid_at": "2024-01-20T00:00:00Z",
  "payment_method": "bank_transfer"
}
```

## Settings Management

### Get User Settings

**GET** `/api/settings`

Retrieve user-specific settings.

**Headers:**

```sh
Authorization: Bearer <jwt-token>
```

**Success Response (200 OK):**

```json
{
  "id": "settings-uuid",
  "user_id": "user-uuid",
  "company_logo": null,
  "invoice_prefix": "INV",
  "next_invoice_number": 2,
  "tax_rate": 19.0,
  "currency": "EUR",
  "payment_terms_days": 30,
  "footer_note": "Vielen Dank für Ihren Auftrag",
  "created_at": "2024-01-15T10:30:00Z"
}
```

### Update Settings

**PUT** `/api/settings`

Update user settings.

**Headers:**

```sh
Authorization: Bearer <jwt-token>
```

**Request Body:**

```json
{
  "invoice_prefix": "FACT",
  "tax_rate": 7.0,
  "currency": "EUR",
  "payment_terms_days": 14,
  "footer_note": "Mit freundlichen Grüßen"
}
```

## Utility Endpoints

### Health Check

**GET** `/health`

Check if the API is running.

**Success Response (200 OK):**

```json
{
  "status": "healthy",
  "timestamp": "2024-01-15T10:30:00Z"
}
```

### Root Endpoint

**GET** `/`

Welcome message.

**Success Response (200 OK):**

```json
"Welcome to MiniDebet API!"
```

## Error Handling

All error responses follow this format:

```json
{
  "error": "Error Type",
  "message": "Human-readable error message",
  "details": "Additional error details (optional)"
}
```

### Common HTTP Status Codes

- **200 OK**: Successful GET, PUT requests
- **201 Created**: Successful POST requests
- **204 No Content**: Successful DELETE requests
- **400 Bad Request**: Invalid request data
- **401 Unauthorized**: Missing or invalid authentication
- **403 Forbidden**: Insufficient permissions
- **404 Not Found**: Resource not found
- **409 Conflict**: Resource already exists
- **422 Unprocessable Entity**: Validation errors
- **500 Internal Server Error**: Server-side errors

## Rate Limiting

Current rate limits:

- **Anonymous requests**: 100 requests per hour
- **Authenticated requests**: 1000 requests per hour

Exceeding limits returns:

```json
{
  "error": "Rate Limit Exceeded",
  "message": "Too many requests. Please try again later.",
  "retry_after": 3600
}
```

## Versioning

API version is included in the URL path:
`/api/v1/endpoint`

Current version: v1

## Changelog

### v1.0.0 (Initial Release)

- User registration and authentication
- Client management (CRUD operations)
- Invoice creation and management
- Basic settings management
- JWT-based authentication

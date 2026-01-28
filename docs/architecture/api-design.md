# API Design

## Base URL

**Development**: `http://localhost:3000/api` (proxied to Cloudflare Workers)
**Production**: `https://minidebet.jamshaidnasar.workers.dev/api`

```sh
# Direct access
https://minidebet.jamshaidnasar.workers.dev/api
```

## Authentication

All API endpoints require JWT authentication except for user registration and login.

## Endpoints

### Users

- `POST /api/users` - Register new user
- `POST /api/auth/login` - Login user
- `GET /api/users/me` - Get current user
- `PUT /api/users/me` - Update user profile

### Clients

- `POST /api/clients` - Create new client
- `GET /api/clients` - List all clients
- `GET /api/clients/:id` - Get client details
- `PUT /api/clients/:id` - Update client
- `DELETE /api/clients/:id` - Delete client

### Invoices

- `POST /api/invoices` - Create new invoice
- `GET /api/invoices` - List all invoices
- `GET /api/invoices/:id` - Get invoice details
- `PUT /api/invoices/:id` - Update invoice
- `DELETE /api/invoices/:id` - Delete invoice
- `POST /api/invoices/:id/send` - Send invoice
- `POST /api/invoices/:id/pay` - Mark invoice as paid

### Settings

- `GET /api/settings` - Get user settings
- `PUT /api/settings` - Update user settings

## Request/Response Examples

### Create User

```json
POST /api/users
{
  "email": "user@example.com",
  "password": "securepassword",
  "first_name": "Max",
  "last_name": "Mustermann",
  "company_name": "My Company GmbH",
  "tax_id": "DE123456789"
}
```

### Create Client

```json
POST /api/clients
{
  "name": "Acme Corp",
  "email": "contact@acme.com",
  "company": "Acme Corporation",
  "street": "Musterstraße 123",
  "city": "Berlin",
  "postal_code": "10115",
  "country": "DE",
  "vat_number": "DE123456789"
}
```

### Create Invoice

```json
POST /api/invoices
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
    }
  ]
}
```


<div align="center">

# 🇩🇪 MiniDebet
### The boring, high-performance invoicing SaaS for German freelancers.

[Frontend: React] · [Backend: Rust] · [DB: SQLite]

</div>

---

## 1. The Mission

MiniDebet is a **utility-first financial tool**. We are not trying to reinvent accounting. We are building a specialized, high-performance engine that solves exactly three problems for German freelancers:

1.  **Legally compliant invoices** (GoBD & §19 UStG ready).
2.  **Anxiety-free tax estimation** (VAT vs. Flat tax).
3.  **Clean data exports** for the Steuerberater.

**Why this exists:** To generate predictable, recurring revenue that funds the bigger vision (LedokozOS).

> *"Boring technology pays for the future."*

---

## 2. The Tech Stack (Performance First)

We chose modern, type-safe technologies to ensure correctness and speed.

### 🎨 Frontend (Client Side)
*   **Framework:** React (v18+)
*   **Styling:** Tailwind CSS (For a clean, minimal, system-first aesthetic)
*   **State:** Zustand or React Query (Server state management)
*   **Build:** Vite

### ⚙️ Backend (Server Side)
*   **Language:** Rust
*   **Framework:** Axum
*   **Async Runtime:** Tokio
*   **Why Rust:** Memory safety, extremely low latency, and strict type enforcement prevent business logic errors.

### 💾 Data & Infrastructure
*   **Database:** PostgreSQL (ACID compliance is non-negotiable for financial data).
*   **ORM:** Diesel (or SeaORM) for Rust type-safe queries.
*   **Hosting:** Hetzner Cloud (CX series).
*   **Payments:** Stripe (Webhooks handled by Rust).
*   **Files:** Object Storage (S3 compatible) for PDF archiving.

---

## 3. Target Audience

### ✅ Who this is for
*   **Selbstständige** (Sole proprietors) in Germany.
*   Freelancers in Tech, Design, and Consulting.
*   Users who currently track money in Excel or paper.
*   **Needs:** Invoicing + basic expense tracking.

### 🚫 Who this is NOT for
*   GmbHs / AGs (Corp structures).
*   Merchants (Handelsregistereintragung).
*   Companies needing payroll (Lohnabrechnung) or complex depreciation.

---

## 4. Legal Compliance (Germany)

The app is built around strict German tax laws.

**Invoice Mandatory Fields (GoBD):**
*   Full Name & Address (Seller + Buyer).
*   Tax Number (**Steuernummer**) or VAT ID (**USt-IdNr.**).
*   Invoice Number (Sequential, unique, no gaps).
*   Invoice Date & Delivery Date.
*   Net Amount, VAT Rate (%), VAT Amount.
*   Gross Amount.

**The Kleinunternehmerregelung (§19 UStG):**
*   If enabled, **no VAT** is calculated.
*   Mandatory Note on Invoice:
    > *"Gemäß §19 UStG wird keine Umsatzsteuer berechnet."*

---

## 5. MVP Feature Scope

**Phase 1 is strictly about invoicing.**

#### 5.1 Authentication
*   Email/Password (JWT tokens handled by Rust).
*   Secure session management (HttpOnly cookies).

#### 5.2 Dashboard
*   Overview of Revenue (Current Month vs. Last Month).
*   Open invoices counter.
*   Quick action: "Create New Invoice".

#### 5.3 Invoice Builder
*   Select Customer (or create new on fly).
*   Add Line Items (Description, Qty, Rate).
*   Auto-calculate totals (Net, VAT 7%, VAT 19%, Gross).
*   **Generate PDF** (Server-side rendering using `lopdf` or similar Rust crate).
*   Download / Email PDF.

#### 5.4 Settings
*   Toggle **Kleinunternehmer** status (recalculates all logic).
*   Set company letterhead data.
*   Input Tax Number.

#### 5.5 Exports
*   Export to CSV (Date, Sender, Receiver, Net, Tax, Type).
*   Format ready for DATEV import.

---

## 6. System Architecture

```
[Client: React SPA]
       ⬇
[API Gateway: Nginx Reverse Proxy]
       ⬇
[Application Server: Rust (Actix/Axum)]
       ⬇
[Database: PostgreSQL]
```

*   **PDF Generation:** Handled asynchronously by the Rust backend to prevent UI blocking.
*   **API Design:** RESTful. JSON over HTTPS.

---

## 8. API Design Principles

The Rust backend will expose a clean REST API.

*   **Base URL:** `/api/v1`
*   **Content-Type:** `application/json`
*   **Error Handling:** Standardized JSON error responses.

**Example Endpoints:**

| Method | Endpoint | Description |
| :--- | :--- | :--- |
| `POST` | `/auth/register` | Create new user |
| `POST` | `/auth/login` | Exchange creds for JWT |
| `GET` | `/profile` | Get user settings |
| `PUT` | `/profile` | Update user settings |
| `GET` | `/invoices` | List all invoices (paginated) |
| `POST` | `/invoices` | Create new invoice |
| `GET` | `/invoices/:id/pdf` | Stream PDF bytes |

---

## 9. Pricing Strategy

Simple, flat pricing. No complex tiers.

*   **Free Tier:** 3 Invoices per month.
*   **Pro (€15/mo):** Unlimited invoices + PDF branding + Exports.
*   **Accepted:** Credit Card (Stripe) & SEPA Direct Debit (via Stripe).

---

## 10. AI Coding Instructions

If you are an AI assistant generating code for MiniDebet, follow these rules:

1.  **Language:** Use **Rust** for backend, **React** for frontend.
2.  **Style:** Keep the UI minimal (Apple-esque or Stripe-like). Avoid clutter.
3.  **Strictness:** Do not add "nice-to-have" features. If it's not in Section 5, don't code it.
4.  **Error Handling:** Rust code must handle `Result<T, E>` properly; never use `.unwrap()` in production logic.
5.  **German Context:** All invoice templates must use proper German terminology.

---

## 11. Founder Note

This app is the fuel for LedokozOS. 
We prioritize **stability over speed of development**. 
A correct invoice that never crashes is better than a buggy app with 100 features.

**Let's build MiniDebet.**

# MiniDebet

**The boring, high-performance invoicing SaaS for German freelancers.**

## Quick Start

### Prerequisites
- Node.js 18+
- Rust 1.70+
- SQLite3

### Development Setup

1. **Install frontend dependencies:**
```bash
cd frontend
npm install
```

2. **Install shared package dependencies:**
```bash
cd ../shared
npm install
npm run build
```

3. **Start the backend:**
```bash
cd ../backend
cargo run
```

4. **Start the frontend:**
```bash
cd ../frontend
npm run dev
```

## Project Structure

```
minidebet/
├── frontend/          # React + TypeScript frontend
├── backend/           # Axum + Rust backend
├── shared/            # Shared TypeScript types
├── docs/              # Documentation
└── README.md
```

## Features

- ✅ Legally compliant German invoices
- ✅ VAT/tax calculations
- ✅ PDF generation
- ✅ Data exports for tax advisors
- ✅ Free/Pro tier pricing
- ✅ Stripe payment integration

## License

This project is licensed under the Source Available License - see the [LICENSE](LICENSE) file for details.

The source code is available for viewing and learning purposes, but commercial distribution is restricted.
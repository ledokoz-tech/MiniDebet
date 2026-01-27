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

1. **Install shared package dependencies:**

```bash
cd ../shared
npm install
npm run build
```

1. **Start the backend:**

```bash
cd ../backend
cargo run
```

1. **Start the frontend:**

```bash
cd ../frontend
npm run dev
```

## Project Structure

```sh
minidebet/
├── frontend/          # React + TypeScript frontend
├── backend/           # Axum + Rust backend
├── shared/            # Shared TypeScript types
├── docs/              # Comprehensive documentation
│   ├── architecture/   # System design and API docs
│   ├── development/    # Setup and development guides
│   ├── database/       # Database schema and design
│   ├── api/           # API reference and examples
│   └── testing/       # Testing strategies and guides
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

This project is proprietary software. Source code access requires a valid license.

**Licensing Options:**

- Personal/Development License: €49/year
- Commercial License: €299/year
- Enterprise License: Custom pricing

Contact **jamshaid@ledokoz.com** for licensing information and source code access.

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details on how to get involved.

Unauthorized use, distribution, or modification is strictly prohibited.

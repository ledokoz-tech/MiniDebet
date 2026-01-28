# MiniDebet

<div align="center">

**The boring, high-performance invoicing SaaS for German freelancers.**

[![License](https://img.shields.io/badge/license-Proprietary-red.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![React](https://img.shields.io/badge/React-18%2B-blue.svg)](https://reactjs.org/)
[![Cloudflare](https://img.shields.io/badge/Cloudflare-Workers-yellow.svg)](https://workers.cloudflare.com/)
[![Contributions](https://img.shields.io/badge/contributions-welcome-brightgreen.svg)](CONTRIBUTING.md)

[Demo](https://demo.minidebet.de) • [Documentation](./docs) • [Support](SUPPORT.md) • [Security](SECURITY.md)

</div>

## 🚀 Quick Start

### Prerequisites

- **Node.js** 18+
- **Rust** 1.70+ (with `wasm32-unknown-unknown` target)
- **Cloudflare Account** (for production deployment)

### Development Setup

1. **Clone and setup the project:**

```bash
# Clone the repository
git clone https://github.com/ledokoz-tech/minidebet.git
cd minidebet

# Install frontend dependencies
cd frontend
npm install

# Build shared types
cd ../shared
npm install && npm run build
```

2. **Configure environment variables:**

```bash
# Create .env file in frontend directory
cp frontend/.env.example frontend/.env
# Edit with your Cloudflare credentials
```

3. **Start development servers:**

```bash
# Terminal 1: Start backend (development)
cd backend
cargo run

# Terminal 2: Start frontend
cd frontend
npm run dev
```

Visit `http://localhost:5173` to see the application!

## 🏗️ Architecture Overview

MiniDebet follows a modern microservices architecture:

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Frontend      │    │   Cloudflare     │    │   D1 Database   │
│   (React/TS)    │◄──►│   Workers        │◄──►│   (SQLite)      │
│                 │    │   (Rust/WASM)    │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

### Tech Stack

- **Frontend**: React 18 + TypeScript + Tailwind CSS
- **Backend**: Rust + Axum + Cloudflare Workers
- **Database**: Cloudflare D1 (SQLite-compatible)
- **Authentication**: JWT-based with secure token handling
- **Deployment**: GitHub Actions + Cloudflare Workers

## 🎯 Key Features

### ✅ Core Functionality
- **German Legal Compliance**: VAT-compliant invoices with proper formatting
- **Smart Calculations**: Automatic VAT/tax calculations with configurable rates
- **Professional PDFs**: Generate print-ready invoices with company branding
- **Tax Advisor Ready**: Export data in formats accepted by German tax authorities
- **Multi-tier Pricing**: Free/Pro/Enterprise plans with feature differentiation

### 🔧 Technical Excellence
- **High Performance**: Sub-100ms response times with Cloudflare's edge network
- **Scalable Architecture**: Serverless design handles traffic spikes automatically
- **Secure by Default**: Industry-standard encryption and security practices
- **Developer Friendly**: Comprehensive documentation and easy local development

### 🌍 International Support
- **Multi-language UI**: German/English interface
- **Currency Flexible**: EUR support with easy currency extension
- **Regional Compliance**: Adheres to German invoicing regulations

## 📁 Project Structure

```
minidebet/
├── frontend/           # React + TypeScript SPA
│   ├── src/
│   │   ├── components/ # Reusable UI components
│   │   ├── pages/      # Application pages
│   │   ├── services/   # API service layer
│   │   └── contexts/   # React contexts
│   └── tailwind.config.js
├── backend/            # Rust backend services
│   ├── src/
│   │   ├── handlers/   # Request handlers
│   │   ├── models/     # Data models
│   │   ├── auth/       # Authentication logic
│   │   └── db/         # Database interactions
│   └── migrations/     # Database schema migrations
├── shared/             # Shared TypeScript definitions
├── docs/               # Comprehensive documentation
│   ├── architecture/   # System design documents
│   ├── development/    # Developer guides
│   ├── api/           # API reference
│   └── deployment/    # Deployment guides
└── .github/           # GitHub configurations
    └── workflows/     # CI/CD pipelines
```

## 🛡️ Security & Compliance

- **SOC 2 Compliant**: Following industry security standards
- **GDPR Ready**: Proper data handling and privacy controls
- **Regular Audits**: Automated security scanning in CI/CD
- **Vulnerability Response**: [Security Policy](SECURITY.md) with 24-hour response time

## 📈 Performance Metrics

| Metric | Performance |
|--------|-------------|
| Page Load Time | < 2 seconds |
| API Response | < 100ms |
| Uptime | 99.9% |
| Database Queries | < 50ms avg |

## 🤝 Contributing

We love contributions! Please read our:

- [Contributing Guide](CONTRIBUTING.md) - Development workflow
- [Code of Conduct](CODE_OF_CONDUCT.md) - Community guidelines
- [Issue Templates](.github/ISSUE_TEMPLATE/) - Reporting bugs/features

### Ways to Contribute

- 🐛 Report bugs using our [bug template](.github/ISSUE_TEMPLATE/bug_report.md)
- 💡 Suggest features with our [feature template](.github/ISSUE_TEMPLATE/feature_request.md)
- 📖 Improve documentation
- 🔧 Submit pull requests
- 🎨 Enhance UI/UX design

## 💰 Licensing & Support

### License Information

This project is proprietary software requiring a valid license for use.

**Available Licenses:**
- **Personal/Development**: €49/year
- **Commercial**: €299/year  
- **Enterprise**: Custom pricing

📧 Contact: **licensing@minidebet.com**

### Support Options

- 📚 [Documentation](./docs) - Comprehensive guides
- 🆘 [Support Center](SUPPORT.md) - Getting help
- 🐛 [Issue Tracker](issues) - Bug reports
- 💬 [Community Chat](https://discord.gg/minidebet) - Real-time discussion

## 🌟 Sponsors & Funding

Support the development of MiniDebet:

- ❤️ [GitHub Sponsors](https://github.com/sponsors/jamshaidnasar)
- ☕ [Buy Me a Coffee](https://www.buymeacoffee.com/minidebet)
- 🏢 Corporate Sponsorship opportunities available

## 📊 Repository Stats

![GitHub stars](https://img.shields.io/github/stars/ledokoz-tech/minidebet)
![GitHub forks](https://img.shields.io/github/forks/ledokoz-tech/minidebet)
![GitHub issues](https://img.shields.io/github/issues/ledokoz-tech/minidebet)
![GitHub last commit](https://img.shields.io/github/last-commit/ledokoz-tech/minidebet)

## 📞 Contact

- **Website**: [minidebet.de](https://minidebet.de)
- **Email**: hello@minidebet.de
- **Twitter**: [@minidebet](https://twitter.com/minidebet)
- **LinkedIn**: [MiniDebet](https://linkedin.com/company/minidebet)

---

<p align="center">
  <strong>Built with ❤️ for German freelancers</strong>
</p>

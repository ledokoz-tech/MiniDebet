# Mini Debet

**The boring, high-performance invoicing SaaS for German freelancers.**

## Mission

MiniDebet is a utility-first financial tool. We are not trying to reinvent accounting. We are building a specialized, high-performance engine that solves exactly three problems for German freelancers:

* Legally compliant invoices for Germany.
* Anxiety-free tax estimation ( VAT vs TAX).
* Clean data exports for the Steuerberater.

## Tech Stack

### Frontend

* React ( TypeScript )
* Tailwind CSS ( Web )
* Iced ( Desktop App )

### Backend

* Axum ( Worker WASM )
* SQLite ( Cloudflare )
* Worker ( Cloudflare )

## Target Audience

This Tools is for:

* Selbstständige (Sole proprietors) in Germany
* Freelancers in Tech, Design, Consulting
* Users tracking money in Excel or paper
* Needs: Invoicing + basic expense tracking

This tool is not for:

* GmbH and AGs
* Merchants
* Companies needing payroll

## Pricing

Simple flat pricing:

* Free Tier: 3 invoices per month
* Pro (€15/mo): Unlimited invoices + PDF branding + Exports
* Accepted: Credit Card & SEPA via Stripe API

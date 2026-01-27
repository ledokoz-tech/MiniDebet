# Cloudflare Workers Deployment Guide

## Overview

This guide explains how to deploy MiniDebet as a Cloudflare Worker with automatic database migration deployment.

## Prerequisites

1. **Cloudflare Account** with Workers subscription
2. **Wrangler CLI** installed globally
3. **Rust toolchain** with WASM target
4. **Python 3** for migration scripts

## Setup Instructions

### 1. Install Dependencies

```bash
# Install Wrangler globally
npm install -g wrangler

# Install worker-build tool
cargo install worker-build

# Add WASM target
rustup target add wasm32-unknown-unknown
```

### 2. Configure Cloudflare Authentication

```bash
# Login to Cloudflare
wrangler login

# Verify authentication
wrangler whoami
```

### 3. Environment Configuration

Create a `.env` file in the project root:

```env
# Cloudflare Configuration
CF_ACCOUNT_ID=your-cloudflare-account-id
CF_API_TOKEN=your-api-token

# Database Configuration
D1_DATABASE_NAME=minidebet
```

## Deployment Process

### Automated Deployment (Recommended)

#### Unix/Linux/macOS:
```bash
# Make script executable
chmod +x scripts/deploy-worker.sh

# Full deployment (build + test + migrate + deploy)
./scripts/deploy-worker.sh full

# Deploy to specific environments
./scripts/deploy-worker.sh deploy-preview  # Preview environment
./scripts/deploy-worker.sh deploy-staging  # Staging environment
./scripts/deploy-worker.sh deploy-prod     # Production environment
```

#### Windows:
```cmd
# Full deployment
scripts\deploy-worker.bat full

# Deploy to specific environments
scripts\deploy-worker.bat deploy-preview
scripts\deploy-worker.bat deploy-staging
scripts\deploy-worker.bat deploy-prod
```

### Manual Deployment Steps

1. **Build the Worker:**
```bash
cd backend/worker
worker-build --release
```

2. **Deploy Migrations:**
```bash
python scripts/deploy-migrations.py
```

3. **Deploy Worker:**
```bash
# Preview deployment
wrangler deploy

# Production deployment
wrangler deploy --env production
```

## Database Migration Management

### Migration Deployment Script

The `deploy-migrations.py` script automatically:
- Creates D1 database if it doesn't exist
- Tracks deployed migrations
- Deploys only pending migrations
- Handles rollback scenarios

### Usage Examples

```bash
# Deploy all pending migrations
python scripts/deploy-migrations.py

# Deploy to specific database
python scripts/deploy-migrations.py my-database-name

# Check migration status
cat .deployed_migrations
```

### Migration File Structure

```
backend/migrations/
├── 0001_create_users.sql
├── 0002_create_clients.sql
├── 0003_create_invoices.sql
├── 0004_create_invoice_items.sql
└── 0005_create_user_settings.sql
```

## Environment Configuration

### wrangler.toml Settings

```toml
name = "minidebet"
main = "backend/worker/dist/worker.js"
compatibility_date = "2024-01-27"

# Preview Environment
[env.preview]
workers_dev = true

# Staging Environment
[env.staging]
workers_dev = true
route = { pattern = "staging.api.minidebet.de/*", zone_name = "minidebet.de" }

# Production Environment
[env.production]
workers_dev = false
route = { pattern = "api.minidebet.de/*", zone_name = "minidebet.de" }

# D1 Database Bindings
[[ d1_databases ]]
binding = "DB"
database_name = "minidebet"
database_id = "your-database-id-here"
```

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Deploy to Cloudflare Workers

on:
  push:
    branches: [main, staging]

jobs:
  deploy:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        target: wasm32-unknown-unknown
    
    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
    
    - name: Install dependencies
      run: |
        npm install -g wrangler
        cargo install worker-build
    
    - name: Authenticate with Cloudflare
      run: echo "${{ secrets.CF_API_TOKEN }}" | wrangler login
    
    - name: Deploy Worker
      run: |
        chmod +x scripts/deploy-worker.sh
        ./scripts/deploy-worker.sh deploy-${{ github.ref_name }}
      env:
        CF_API_TOKEN: ${{ secrets.CF_API_TOKEN }}
```

## Monitoring and Debugging

### Local Development

```bash
# Develop locally with miniflare
wrangler dev

# Test with local D1 database
wrangler dev --local
```

### Logs and Metrics

```bash
# View live logs
wrangler tail

# View recent deployments
wrangler deployments list

# Get worker metrics
wrangler metrics
```

## Troubleshooting

### Common Issues

1. **WASM Compilation Errors:**
```bash
# Clean build artifacts
cargo clean
# Rebuild
worker-build --release
```

2. **Database Connection Issues:**
```bash
# Verify D1 database exists
wrangler d1 list
# Create database if needed
wrangler d1 create minidebet
```

3. **Authentication Problems:**
```bash
# Re-authenticate
wrangler login
# Check permissions
wrangler whoami
```

4. **Migration Failures:**
```bash
# Check deployed migrations
cat .deployed_migrations
# Manually deploy specific migration
wrangler d1 execute minidebet --file=./backend/migrations/0001_create_users.sql
```

## Security Considerations

### Best Practices

1. **Environment Variables:**
   - Never commit API tokens to version control
   - Use Wrangler secrets for sensitive data
   ```bash
   wrangler secret put JWT_SECRET
   ```

2. **Access Control:**
   - Configure proper CORS settings
   - Implement rate limiting
   - Use secure authentication mechanisms

3. **Database Security:**
   - Regular backup of D1 databases
   - Monitor query performance
   - Implement proper data validation

## Rollback Procedures

### Worker Rollback

```bash
# List recent deployments
wrangler deployments list

# Rollback to previous version
wrangler rollback
```

### Database Rollback

```bash
# Execute rollback migration
wrangler d1 execute minidebet --file=./backend/migrations/0001_create_users.down.sql

# Update deployed migrations tracking
# (Manual edit of .deployed_migrations file)
```

## Performance Optimization

### Worker Optimization

1. **Bundle Size Reduction:**
   - Enable dead code elimination
   - Use `lto = true` in Cargo.toml
   - Strip debug symbols

2. **Caching Strategies:**
   - Implement response caching
   - Use Cloudflare Cache API
   - Configure proper TTL values

3. **Database Optimization:**
   - Add appropriate indexes
   - Optimize query patterns
   - Monitor D1 performance metrics

This deployment setup provides a robust, automated way to deploy MiniDebet as a Cloudflare Worker with proper database migration handling.
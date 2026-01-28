# Development Setup Guide

## Prerequisites

Before you begin, ensure you have the following installed:

- **Node.js** 18+ ([Download](https://nodejs.org/))
- **Rust** 1.70+ ([Installation Guide](https://www.rust-lang.org/tools/install))
- **SQLite3** ([Download](https://www.sqlite.org/download.html))
- **Git** ([Download](https://git-scm.com/))

## Initial Setup

### 1. Clone the Repository

```bash
git clone <repository-url>
cd MiniDebet
```

### 2. Environment Configuration

Create a `.env` file in the backend directory:

```bash
cd backend
touch .env
```

Add the following configuration:

```env
# Database
DATABASE_URL=sqlite:minidebet.db

# JWT Secret (change in production)
JWT_SECRET=your-secret-key-change-in-production

# Server Configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=3000

# Frontend URL (for CORS)
FRONTEND_URL=http://localhost:5173

# Cloudflare Workers Backend URL
CLOUDFLARE_WORKERS_URL=https://minidebet.jamshaidnasar.workers.dev
```

### 3. Install Dependencies

Using the Makefile (recommended):

```bash
make setup-all
```

Or manually:

**Frontend:**

```bash
cd frontend
npm install
```

**Shared Package:**

```bash
cd ../shared
npm install
npm run build
```

**Backend:**

```bash
cd ../backend
cargo build
```

## Running the Application

### Development Mode

**Using Makefile:**

```bash
make dev-all
```

**Manually:**

Terminal 1 (Backend):

```bash
cd backend
cargo run
```

Terminal 2 (Frontend):

```bash
cd frontend
npm run dev
```

The application will be available at:

- Frontend: <http://localhost:5173>
- Backend API: <http://localhost:3000>

### Production Build

**Frontend:**

```bash
cd frontend
npm run build
```

**Backend:**

```bash
cd backend
cargo build --release
```

## Database Management

### Running Migrations

```bash
cd backend
# Run all pending migrations
sqlx migrate run

# Or using Makefile
make migrate
```

### Creating New Migrations

```bash
cd backend
sqlx migrate add migration_name
```

This creates:

- `migrations/{timestamp}_migration_name.sql` (up migration)
- `migrations/{timestamp}_migration_name.down.sql` (down migration)

### Resetting Database

```bash
make reset-db
```

⚠️ **Warning:** This will delete all data!

## Development Tools

### Code Quality

**Format code:**

```bash
# Rust (backend)
cd backend
cargo fmt

# TypeScript (frontend/shared)
cd frontend
npm run format
```

**Linting:**

```bash
# Rust
cd backend
cargo clippy

# TypeScript
cd frontend
npm run lint
```

### Testing

```bash
# Run all tests
make test-all

# Backend tests only
make test-backend

# Frontend tests only
make test-frontend
```

### Database Inspection

```bash
# Open SQLite CLI
cd backend
sqlite3 minidebet.db

# Useful commands:
.tables                    # List tables
.schema users             # Show table schema
SELECT * FROM users;      # Query data
.quit                     # Exit
```

## Troubleshooting

### Common Issues

1. Rust compilation errors

```bash
# Update Rust toolchain
rustup update

# Clear cargo cache
cargo clean
```

1. Node.js version issues

```bash
# Check version
node --version

# Use nvm to manage versions
nvm install 18
nvm use 18
```

1. Database connection issues

- Ensure SQLite is installed and in PATH
- Check `DATABASE_URL` in `.env` file
- Verify database file permissions

1. Port conflicts

- Change ports in `.env` file
- Kill processes using the ports:

```bash
# Linux/Mac
lsof -i :3000
kill -9 <PID>

# Windows
netstat -ano | findstr :3000
taskkill /PID <PID> /F
```

### Debugging

**Enable verbose logging:**

Backend `.env`:

```env
RUST_LOG=debug
```

Frontend console:

```javascript
localStorage.debug = '*'
```

## IDE Setup Recommendations

### VS Code Extensions

- **Rust**: rust-analyzer
- **TypeScript**: TypeScript Importer, ESLint
- **Database**: SQLite Viewer
- **General**: GitLens, Prettier

### IntelliJ IDEA

- **Rust plugin**: IntelliJ Rust
- **Node.js plugin**: NodeJS

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for contribution guidelines.

## Next Steps

After successful setup:

1. Review the [API Documentation](../architecture/api-design.md)
2. Explore the [Project Structure](../../PROJECT_STATUS.md)
3. Check out the [Deployment Guide](./deployment.md)

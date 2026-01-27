.PHONY: help
help: ## Show this help
	@echo "MiniDebet Development Commands:"
	@echo ""
	@echo "Setup:"
	@echo "  setup-frontend    Install frontend dependencies"
	@echo "  setup-backend     Install backend dependencies"
	@echo "  setup-all         Setup both frontend and backend"
	@echo ""
	@echo "Development:"
	@echo "  dev-frontend      Start frontend development server"
	@echo "  dev-backend       Start backend development server"
	@echo "  dev-all           Start both servers"
	@echo ""
	@echo "Build:"
	@echo "  build-frontend    Build frontend for production"
	@echo "  build-backend     Build backend for production"
	@echo "  build-all         Build both for production"
	@echo ""
	@echo "Testing:"
	@echo "  test-frontend     Run frontend tests"
	@echo "  test-backend      Run backend tests"
	@echo "  test-all          Run all tests"
	@echo ""
	@echo "Database:"
	@echo "  migrate           Run database migrations"
	@echo "  reset-db          Reset database"

# Setup commands
setup-frontend:
	cd frontend && npm install

setup-backend:
	cd backend && cargo build

setup-all: setup-frontend setup-backend

# Development commands
dev-frontend:
	cd frontend && npm run dev

dev-backend:
	cd backend && cargo run

dev-all:
	make dev-backend &
	make dev-frontend

# Build commands
build-frontend:
	cd frontend && npm run build

build-backend:
	cd backend && cargo build --release

build-all: build-frontend build-backend

# Test commands
test-frontend:
	cd frontend && npm run test

test-backend:
	cd backend && cargo test

test-all: test-frontend test-backend

# Database commands
migrate:
	cd backend && DATABASE_URL="sqlite:minidebet.db" sqlx migrate run

reset-db:
	rm -f backend/minidebet.db
	make migrate

.DEFAULT_GOAL := help
#!/usr/bin/env bash
# MiniDebet Cloudflare Worker Deployment Script

set -e  # Exit on any error

# Configuration
WORKER_NAME="minidebet"
BACKEND_DIR="./backend"
WORKER_DIR="$BACKEND_DIR/worker"
BUILD_DIR="$BACKEND_DIR/dist"

echo "🚀 Starting MiniDebet Cloudflare Worker Deployment"

# Function to check dependencies
check_dependencies() {
    echo "🔍 Checking dependencies..."
    
    # Check if Rust is installed
    if ! command -v rustc &> /dev/null; then
        echo "❌ Rust is not installed. Please install Rust from https://rustup.rs/"
        exit 1
    fi
    
    # Check if wasm32-unknown-unknown target is installed
    if ! rustc --print target-list | grep -q wasm32-unknown-unknown; then
        echo "🔧 Installing wasm32-unknown-unknown target..."
        rustup target add wasm32-unknown-unknown
    fi
    
    # Check if wrangler is installed
    if ! command -v wrangler &> /dev/null; then
        echo "🔧 Installing Wrangler..."
        npm install -g wrangler
    fi
    
    # Check if worker-build is installed
    if ! command -v worker-build &> /dev/null; then
        echo "🔧 Installing worker-build..."
        cargo install worker-build
    fi
    
    echo "✅ All dependencies are ready"
}

# Function to build the worker
build_worker() {
    echo "🏗️  Building Cloudflare Worker..."
    
    cd "$WORKER_DIR"
    
    # Build the worker
    echo "📦 Compiling Rust to WASM..."
    worker-build --release
    
    # Check if build was successful
    if [ ! -f "dist/worker.js" ]; then
        echo "❌ Build failed - worker.js not found"
        exit 1
    fi
    
    echo "✅ Worker built successfully"
    cd "../.."
}

# Function to deploy to Cloudflare
deploy_worker() {
    local environment=${1:-preview}
    
    echo "☁️  Deploying to Cloudflare Workers ($environment)..."
    
    case $environment in
        "production")
            wrangler deploy --env production
            ;;
        "staging")
            wrangler deploy --env staging
            ;;
        "preview"|*)
            wrangler deploy
            ;;
    esac
    
    if [ $? -eq 0 ]; then
        echo "✅ Deployment successful!"
    else
        echo "❌ Deployment failed!"
        exit 1
    fi
}

# Function to run tests
run_tests() {
    echo "🧪 Running tests..."
    
    cd "$BACKEND_DIR"
    
    # Run Rust tests
    cargo test
    
    cd ".."
    echo "✅ Tests passed"
}

# Function to deploy migrations
deploy_migrations() {
    echo "🗄️  Deploying database migrations..."
    
    python3 ./scripts/deploy-migrations.py
    
    if [ $? -eq 0 ]; then
        echo "✅ Migrations deployed successfully"
    else
        echo "❌ Migration deployment failed"
        exit 1
    fi
}

# Function to show usage
show_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  build           Build the worker only"
    echo "  test            Run tests only"
    echo "  deploy-preview  Deploy to preview environment (default)"
    echo "  deploy-staging  Deploy to staging environment"
    echo "  deploy-prod     Deploy to production environment"
    echo "  migrate         Deploy database migrations only"
    echo "  full            Build, test, migrate, and deploy to preview"
    echo "  help            Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0                    # Full deployment to preview"
    echo "  $0 build              # Build only"
    echo "  $0 deploy-prod        # Deploy to production"
    echo "  $0 migrate            # Deploy migrations only"
}

# Main execution
main() {
    local action=${1:-full}
    
    case $action in
        "build")
            check_dependencies
            build_worker
            ;;
        "test")
            run_tests
            ;;
        "deploy-preview")
            check_dependencies
            deploy_worker "preview"
            ;;
        "deploy-staging")
            check_dependencies
            deploy_worker "staging"
            ;;
        "deploy-prod")
            check_dependencies
            deploy_worker "production"
            ;;
        "migrate")
            deploy_migrations
            ;;
        "full")
            check_dependencies
            run_tests
            build_worker
            deploy_migrations
            deploy_worker "preview"
            ;;
        "help"|"-h"|"--help")
            show_usage
            ;;
        *)
            echo "❌ Unknown action: $action"
            show_usage
            exit 1
            ;;
    esac
}

# Run main function
main "$@"
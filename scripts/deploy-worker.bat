@echo off
REM MiniDebet Cloudflare Worker Deployment Script (Windows)

setlocal enabledelayedexpansion

REM Configuration
set WORKER_NAME=minidebet
set BACKEND_DIR=backend
set WORKER_DIR=%BACKEND_DIR%\worker

echo 🚀 Starting MiniDebet Cloudflare Worker Deployment

REM Function to check dependencies
:check_dependencies
echo 🔍 Checking dependencies...

REM Check if Rust is installed
rustc --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Rust is not installed. Please install Rust from https://rustup.rs/
    exit /b 1
)

REM Check if wasm32-unknown-unknown target is installed
rustc --print target-list | findstr wasm32-unknown-unknown >nul
if %errorlevel% neq 0 (
    echo 🔧 Installing wasm32-unknown-unknown target...
    rustup target add wasm32-unknown-unknown
)

REM Check if wrangler is installed
wrangler --version >nul 2>&1
if %errorlevel% neq 0 (
    echo 🔧 Installing Wrangler...
    npm install -g wrangler
)

REM Check if worker-build is installed
worker-build --version >nul 2>&1
if %errorlevel% neq 0 (
    echo 🔧 Installing worker-build...
    cargo install worker-build
)

echo ✅ All dependencies are ready
goto :eof

REM Function to build the worker
:build_worker
echo 🏗️  Building Cloudflare Worker...

cd %WORKER_DIR%

REM Build the worker
echo 📦 Compiling Rust to WASM...
worker-build --release

REM Check if build was successful
if not exist "dist\worker.js" (
    echo ❌ Build failed - worker.js not found
    exit /b 1
)

echo ✅ Worker built successfully
cd ..\..
goto :eof

REM Function to deploy to Cloudflare
:deploy_worker
set environment=%~1
if "%environment%"=="" set environment=preview

echo ☁️  Deploying to Cloudflare Workers (%environment%)...

if "%environment%"=="production" (
    wrangler deploy --env production
) else if "%environment%"=="staging" (
    wrangler deploy --env staging
) else (
    wrangler deploy
)

if %errorlevel% equ 0 (
    echo ✅ Deployment successful!
) else (
    echo ❌ Deployment failed!
    exit /b 1
)
goto :eof

REM Function to run tests
:run_tests
echo 🧪 Running tests...

cd %BACKEND_DIR%

REM Run Rust tests
cargo test

cd ..
echo ✅ Tests passed
goto :eof

REM Function to deploy migrations
:deploy_migrations
echo 🗄️  Deploying database migrations...

python ./scripts/deploy-migrations.py

if %errorlevel% equ 0 (
    echo ✅ Migrations deployed successfully
) else (
    echo ❌ Migration deployment failed
    exit /b 1
)
goto :eof

REM Function to show usage
:show_usage
echo Usage: %~nx0 [OPTIONS]
echo.
echo Options:
echo   build           Build the worker only
echo   test            Run tests only
echo   deploy-preview  Deploy to preview environment (default)
echo   deploy-staging  Deploy to staging environment
echo   deploy-prod     Deploy to production environment
echo   migrate         Deploy database migrations only
echo   full            Build, test, migrate, and deploy to preview
echo   help            Show this help message
echo.
echo Examples:
echo   %~nx0                    # Full deployment to preview
echo   %~nx0 build              # Build only
echo   %~nx0 deploy-prod        # Deploy to production
echo   %~nx0 migrate            # Deploy migrations only
goto :eof

REM Main execution
set action=%~1
if "%action%"=="" set action=full

if "%action%"=="build" (
    call :check_dependencies
    call :build_worker
) else if "%action%"=="test" (
    call :run_tests
) else if "%action%"=="deploy-preview" (
    call :check_dependencies
    call :deploy_worker preview
) else if "%action%"=="deploy-staging" (
    call :check_dependencies
    call :deploy_worker staging
) else if "%action%"=="deploy-prod" (
    call :check_dependencies
    call :deploy_worker production
) else if "%action%"=="migrate" (
    call :deploy_migrations
) else if "%action%"=="full" (
    call :check_dependencies
    call :run_tests
    call :build_worker
    call :deploy_migrations
    call :deploy_worker preview
) else if "%action%"=="help" (
    call :show_usage
) else (
    echo ❌ Unknown action: %action%
    call :show_usage
    exit /b 1
)

:end
endlocal
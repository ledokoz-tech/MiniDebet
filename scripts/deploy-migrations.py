#!/usr/bin/env python3
"""
Cloudflare D1 Migration Deployment Script

This script automatically deploys database migrations to Cloudflare D1.
"""

import os
import sys
import subprocess
import json
from pathlib import Path
from typing import List, Dict

class MigrationDeployer:
    def __init__(self, database_name: str = "minidebet"):
        self.database_name = database_name
        self.migrations_dir = Path("backend/migrations")
        self.deployed_migrations_file = Path(".deployed_migrations")
        
    def get_available_migrations(self) -> List[Dict]:
        """Get list of all available migration files"""
        migrations = []
        
        if not self.migrations_dir.exists():
            print("❌ Migrations directory not found")
            return migrations
            
        for migration_file in sorted(self.migrations_dir.glob("*.sql")):
            if not migration_file.name.endswith(".down.sql"):  # Skip rollback files
                migration_id = migration_file.stem
                migrations.append({
                    "id": migration_id,
                    "file": migration_file,
                    "name": migration_file.name
                })
                
        return migrations
    
    def get_deployed_migrations(self) -> List[str]:
        """Get list of already deployed migrations"""
        if not self.deployed_migrations_file.exists():
            return []
            
        try:
            with open(self.deployed_migrations_file, 'r') as f:
                return json.load(f)
        except Exception as e:
            print(f"⚠️  Could not read deployed migrations file: {e}")
            return []
    
    def save_deployed_migrations(self, migrations: List[str]):
        """Save list of deployed migrations"""
        try:
            with open(self.deployed_migrations_file, 'w') as f:
                json.dump(migrations, f, indent=2)
        except Exception as e:
            print(f"❌ Failed to save deployed migrations: {e}")
    
    def deploy_migration(self, migration: Dict) -> bool:
        """Deploy a single migration to D1"""
        print(f"🚀 Deploying migration: {migration['name']}")
        
        try:
            # Read migration content
            with open(migration['file'], 'r') as f:
                sql_content = f.read()
            
            # Deploy using wrangler - write to temp file to avoid argument issues
            import tempfile
            with tempfile.NamedTemporaryFile(mode='w', suffix='.sql', delete=False) as f:
                f.write(sql_content)
                temp_file = f.name
            
            cmd = [
                "npx", "wrangler", "d1", "execute", self.database_name,
                "--file", temp_file
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True)
            
            # Clean up temp file
            import os
            os.unlink(temp_file)
            
            if result.returncode == 0:
                print(f"✅ Successfully deployed: {migration['name']}")
                return True
            else:
                print(f"❌ Failed to deploy {migration['name']}:")
                print(f"Error: {result.stderr}")
                return False
                
        except Exception as e:
            print(f"❌ Exception deploying {migration['name']}: {e}")
            return False
    
    def create_database_if_not_exists(self):
        """Create D1 database if it doesn't exist"""
        print("🔍 Checking if database exists...")
        
        try:
            # List existing databases
            result = subprocess.run([
                "npx", "wrangler", "d1", "list", "--json"
            ], capture_output=True, text=True)
            
            if result.returncode == 0:
                databases = json.loads(result.stdout)
                db_exists = any(db['name'] == self.database_name for db in databases)
                
                if not db_exists:
                    print(f"🏗️  Creating database: {self.database_name}")
                    create_result = subprocess.run([
                        "npx", "wrangler", "d1", "create", self.database_name
                    ], capture_output=True, text=True)
                    
                    if create_result.returncode != 0:
                        print(f"❌ Failed to create database: {create_result.stderr}")
                        return False
                    else:
                        print(f"✅ Database created successfully")
                else:
                    print(f"✅ Database {self.database_name} already exists")
                    
            return True
        except Exception as e:
            print(f"❌ Error checking/creating database: {e}")
            return False
    
    def deploy_all_migrations(self):
        """Deploy all pending migrations"""
        print("📋 Starting migration deployment process...")
        
        # Check if wrangler is available
        if not self.check_wrangler():
            return False
            
        # Create database if needed
        if not self.create_database_if_not_exists():
            return False
        
        # Get migrations
        available_migrations = self.get_available_migrations()
        deployed_migrations = self.get_deployed_migrations()
        
        if not available_migrations:
            print("ℹ️  No migrations found")
            return True
            
        print(f"📊 Found {len(available_migrations)} migrations")
        print(f"💾 {len(deployed_migrations)} migrations already deployed")
        
        # Deploy pending migrations
        pending_migrations = [
            m for m in available_migrations 
            if m['id'] not in deployed_migrations
        ]
        
        if not pending_migrations:
            print("✅ All migrations already deployed")
            return True
            
        print(f"🚀 Deploying {len(pending_migrations)} pending migrations...")
        
        success_count = 0
        failed_migrations = []
        
        for migration in pending_migrations:
            if self.deploy_migration(migration):
                deployed_migrations.append(migration['id'])
                success_count += 1
            else:
                failed_migrations.append(migration['name'])
        
        # Save deployed migrations
        self.save_deployed_migrations(deployed_migrations)
        
        # Report results
        print("\n" + "="*50)
        print("📊 MIGRATION DEPLOYMENT SUMMARY")
        print("="*50)
        print(f"✅ Successfully deployed: {success_count}")
        print(f"❌ Failed deployments: {len(failed_migrations)}")
        
        if failed_migrations:
            print("\nFailed migrations:")
            for migration in failed_migrations:
                print(f"  - {migration}")
            return False
        else:
            print("\n🎉 All migrations deployed successfully!")
            return True
    
    def check_wrangler(self) -> bool:
        """Check if wrangler is installed and configured"""
        try:
            result = subprocess.run(["npx", "wrangler", "--version"], 
                                  capture_output=True, text=True)
            if result.returncode == 0:
                print(f"✅ Wrangler version: {result.stdout.strip()}")
                return True
            else:
                print("❌ Wrangler not found. Please install with: npm install -g wrangler")
                return False
        except Exception as e:
            print(f"❌ Error checking wrangler: {e}")
            return False

def main():
    """Main entry point"""
    if len(sys.argv) > 1:
        database_name = sys.argv[1]
    else:
        database_name = "minidebet"
    
    deployer = MigrationDeployer(database_name)
    
    if deployer.deploy_all_migrations():
        print("\n✅ Migration deployment completed successfully!")
        sys.exit(0)
    else:
        print("\n❌ Migration deployment failed!")
        sys.exit(1)

if __name__ == "__main__":
    main()
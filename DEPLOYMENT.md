# Deployment Guide

## For New Developers or Production Deployment

When you clone this repository or deploy to a new environment, follow these steps:

### 1. Clone the Repository
```bash
git clone <repository-url>
cd owc-fantasy-rs
```

### 2. Set Up the Database
```bash
# Create the database (first time only)
mysql -u root -p < db.sql

# Run migrations (every time you pull new code with database changes)
mysql -u root -p owc_fantasy < migration_average_scores.sql
```

### 3. Configure Environment Variables
Create a `.env` file in the `backend` directory with your configuration:
```env
DATABASE_URL=mysql://root:password@localhost/owc_fantasy
OAUTH_CLIENT_ID=your_osu_client_id
OAUTH_CLIENT_SECRET=your_osu_client_secret
# ... other env vars
```

### 4. Build and Run
```bash
cd backend
cargo build --release
cargo run --release
```

## Checking If Migrations Are Needed

After pulling new code from Git, check if there are new migration files:

```bash
# List all migrations
ls -la migrations/

# Check what the last migration was
git log --oneline -- migrations/
```

If you see new `.sql` files in the `migrations/` folder or `migration_*.sql` at the root, run them.

## Production Deployment Checklist

- [ ] Backup production database before running migrations
- [ ] Test migration on a copy of production data first
- [ ] Run migration: `mysql -u prod_user -p prod_db < migration_average_scores.sql`
- [ ] Build release binary: `cargo build --release`
- [ ] Deploy new binary
- [ ] Re-import multiplayer links if needed (for this migration)
- [ ] Monitor logs for errors

## Migration Safety

All migrations now use `IF NOT EXISTS` or similar checks, making them **idempotent** (safe to run multiple times). If you accidentally run a migration twice, it won't break anything.

## Rollback

If you need to rollback a migration, you'll need to manually reverse the changes. For the average scores migration:

```sql
-- Rollback 001_average_scores.sql
ALTER TABLE PlayerScores 
    DROP COLUMN IF EXISTS total_score,
    DROP COLUMN IF EXISTS match_count;
```

**Note:** Only rollback if absolutely necessary and you have backups!

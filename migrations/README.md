# Database Migrations

This folder contains SQL migration scripts that update the database schema.

## How to Run Migrations

**Initial Setup or After Pulling New Code:**
```bash
# Run all migrations in order
mysql -u root -p owc_fantasy < migrations/001_average_scores.sql
```

**For Production Deployment:**
```bash
# On your production server, run the same command
mysql -u <prod_user> -p <prod_database> < migrations/001_average_scores.sql
```

## Migration Files

Each migration is numbered and named descriptively:

- `001_average_scores.sql` - Adds total_score and match_count columns to PlayerScores table for averaging scores across multiple matches

## Important Notes

1. **Run migrations in order** - The numbers (001, 002, etc.) indicate the order
2. **Run each migration only once** - They use `IF NOT EXISTS` checks to be safe
3. **Always backup your database before running migrations in production**
4. **Test migrations on a development database first**

## Creating New Migrations

When you need to change the database schema:

1. Create a new file: `migrations/00X_description.sql` (increment the number)
2. Add `IF NOT EXISTS` or similar checks to make it idempotent (safe to run multiple times)
3. Document what the migration does in this README
4. Test it on your development database
5. Commit it to Git so others can run it too

## Checking What's Been Run

You can check if a migration has been applied by looking at the database structure:

```bash
# Check if a table exists
echo "SHOW TABLES;" | mysql -u root -p owc_fantasy

# Check table structure
echo "DESCRIBE PlayerScores;" | mysql -u root -p owc_fantasy
```

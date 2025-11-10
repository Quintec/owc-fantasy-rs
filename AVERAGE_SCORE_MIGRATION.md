# Average Score Migration

## Overview
Updated the scoring system to calculate **average points across matches** instead of just storing the last match's score.

## Changes Made

### Database Schema
Added two new columns to `PlayerScores` table:
- `total_score INT` - Sum of all points across all matches in the round
- `match_count INT` - Number of matches played in the round
- `score INT` - Now stores the **average** (total_score / match_count), rounded to nearest integer

### Code Changes
1. **`admin.rs`**: Modified multiplayer parser to accumulate scores across all matches before saving
2. **`players.rs`**: Updated `update_player_round_score()` to accept total_score and match_count parameters
3. Score calculation: `avg_score = round(total_score / match_count)`

## Migration Steps

### 1. Run the SQL migration
```bash
mysql -u root -p owc_fantasy < migration_average_scores.sql
```

This will:
- Add `total_score` and `match_count` columns
- Migrate existing data (assumes current scores are already averages with match_count=1)

### 2. Rebuild and restart backend
```bash
cd backend
cargo build --release
# Then restart your backend server
```

### 3. Re-import all multiplayer links
Since the scoring logic changed, you should re-import all MPs for each round to recalculate averages properly.

## How It Works Now

When you import MPs for a round:
1. **Accumulates** scores across all matches in memory
2. **Calculates** average: `total_score / match_count`
3. **Stores** all three values: `score` (average), `total_score`, `match_count`
4. **Re-importing** the same round will **replace** all scores (not add to them)

## Example
If a player plays in 3 matches in RO16:
- Match 1: 10 points
- Match 2: 15 points  
- Match 3: 20 points

Stored values:
- `total_score` = 45
- `match_count` = 3
- `score` = 15 (average, rounded)

This average score (15) is what's displayed and used for team calculations.

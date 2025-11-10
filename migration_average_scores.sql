-- Migration: Add match tracking for average score calculation
-- Date: 2025-11-10
-- Description: This allows storing total_score and match_count to calculate averages

USE owc_fantasy;

-- Add new columns to PlayerScores table (if they don't exist)
ALTER TABLE PlayerScores 
    ADD COLUMN IF NOT EXISTS total_score INT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS match_count INT NOT NULL DEFAULT 0;

-- Migrate existing data: assume score column is already the total/average
-- Set match_count to 1 for existing non-zero scores (only update if not already set)
UPDATE PlayerScores 
SET total_score = score, match_count = 1 
WHERE score > 0 AND match_count = 0;

-- Note: The 'score' column will now be used as a computed average
-- We'll keep it for backward compatibility and update it via application logic

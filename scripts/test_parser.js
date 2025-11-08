#!/usr/bin/env node

/**
 * test_parser.js - Quick test to verify ID extraction from participants file
 * Usage: node test_parser.js <participants_file>
 */

const fs = require('fs');

if (process.argv.length < 3) {
  console.error('Usage: node test_parser.js <participants_file>');
  process.exit(1);
}

const file = process.argv[2];

/**
 * Extract all numeric IDs from the input text.
 * This handles various formats:
 * - osu! profile links: https://osu.ppy.sh/users/12345
 * - Plain numeric IDs: one per line, comma-separated, JSON arrays, etc.
 * - Markdown links: [Username](https://osu.ppy.sh/users/12345)
 */
function extractPlayerIds(text) {
  const ids = new Set();
  
  // First, try to extract IDs from osu! profile URLs
  // Matches: https://osu.ppy.sh/users/12345 or osu.ppy.sh/users/12345
  const urlMatches = text.matchAll(/osu\.ppy\.sh\/users\/(\d+)/g);
  for (const match of urlMatches) {
    ids.add(parseInt(match[1], 10));
  }
  
  // If we found IDs from URLs, use those (most reliable for osu! data)
  if (ids.size > 0) {
    return Array.from(ids);
  }
  
  // Fallback: extract any numeric IDs from the text
  const matches = text.match(/\b\d+\b/g);
  if (!matches) return [];
  
  // Convert to numbers and deduplicate
  return [...new Set(matches.map(m => parseInt(m, 10)))];
}

const content = fs.readFileSync(file, 'utf-8');
const ids = extractPlayerIds(content);

console.log(`Found ${ids.length} player IDs:`);
console.log(ids.slice(0, 10).join(', ') + (ids.length > 10 ? '...' : ''));
console.log('\nAll IDs:');
console.log(JSON.stringify(ids, null, 2));

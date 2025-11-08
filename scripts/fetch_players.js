#!/usr/bin/env node

/**
 * fetch_players.js
 * 
 * Extracts player IDs from a tournament participants list, fetches their
 * profile information using OAuth2 client credentials flow, and outputs
 * a JSON array of Player objects ready for database insertion.
 * 
 * Usage:
 *   node fetch_players.js <participants_file>
 * 
 * Environment variables required:
 *   OAUTH_TOKEN_URL       - OAuth2 token endpoint URL
 *   OAUTH_CLIENT_ID       - OAuth2 client ID
 *   OAUTH_CLIENT_SECRET   - OAuth2 client secret
 *   OAUTH_USERINFO_URL    - User info endpoint template with {id} placeholder
 *                          Example: https://api.example.com/users/{id}
 * 
 * The participants file should contain player IDs, one per line or in any
 * parseable format (the script will extract numeric IDs).
 * 
 * Output: JSON array written to stdout, matching the Player struct:
 * {
 *   "id": number,
 *   "username": string,
 *   "avatar_url": string,
 *   "country": string,
 *   "rank": number
 * }
 */

const fs = require('fs');
const https = require('https');
const http = require('http');
const { URL } = require('url');

// Read environment variables
const TOKEN_URL = process.env.OAUTH_TOKEN_URL;
const CLIENT_ID = process.env.OAUTH_CLIENT_ID;
const CLIENT_SECRET = process.env.OAUTH_CLIENT_SECRET;
const USERINFO_TEMPLATE = process.env.OAUTH_USERINFO_URL;

if (!TOKEN_URL || !CLIENT_ID || !CLIENT_SECRET || !USERINFO_TEMPLATE) {
  console.error('Error: Missing required environment variables.');
  console.error('Required: OAUTH_TOKEN_URL, OAUTH_CLIENT_ID, OAUTH_CLIENT_SECRET, OAUTH_USERINFO_URL');
  process.exit(1);
}

// Parse command line arguments
const args = process.argv.slice(2);
if (args.length === 0) {
  console.error('Usage: node fetch_players.js <participants_file>');
  console.error('');
  console.error('The participants file should contain player IDs (one per line or any format).');
  process.exit(1);
}

const participantsFile = args[0];

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

/**
 * Make an HTTP/HTTPS request and return the response as a parsed JSON object.
 */
function httpRequest(url, options = {}) {
  return new Promise((resolve, reject) => {
    const urlObj = new URL(url);
    const client = urlObj.protocol === 'https:' ? https : http;
    
    const reqOptions = {
      hostname: urlObj.hostname,
      port: urlObj.port,
      path: urlObj.pathname + urlObj.search,
      method: options.method || 'GET',
      headers: options.headers || {},
    };

    const req = client.request(reqOptions, (res) => {
      let data = '';
      res.on('data', (chunk) => { data += chunk; });
      res.on('end', () => {
        try {
          const parsed = JSON.parse(data);
          resolve({ status: res.statusCode, data: parsed });
        } catch (e) {
          reject(new Error(`Failed to parse JSON response: ${e.message}`));
        }
      });
    });

    req.on('error', (e) => reject(e));
    
    if (options.body) {
      req.write(options.body);
    }
    
    req.end();
  });
}

/**
 * Obtain an OAuth2 access token using client credentials flow.
 */
async function getAccessToken() {
  const body = new URLSearchParams({
    grant_type: 'client_credentials',
    client_id: CLIENT_ID,
    client_secret: CLIENT_SECRET,
  }).toString();

  const response = await httpRequest(TOKEN_URL, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded',
      'Content-Length': Buffer.byteLength(body),
    },
    body,
  });

  if (response.status !== 200) {
    throw new Error(`Token request failed with status ${response.status}: ${JSON.stringify(response.data)}`);
  }

  if (!response.data.access_token) {
    throw new Error('No access_token in token response');
  }

  return response.data.access_token;
}

/**
 * Fetch user profile information for a given player ID.
 */
async function fetchPlayerInfo(playerId, accessToken) {
  const url = USERINFO_TEMPLATE.replace('{id}', playerId);
  
  const response = await httpRequest(url, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${accessToken}`,
    },
  });

  if (response.status !== 200) {
    throw new Error(`User info request failed for player ${playerId} with status ${response.status}`);
  }

  return response.data;
}

/**
 * Map API response to Player struct format.
 * Handles osu! API v2 response format with nested statistics object.
 * Adjust field mappings based on your OAuth provider's response format.
 */
function mapToPlayer(playerId, userInfo) {
  // For osu! API v2, statistics are nested
  const stats = userInfo.statistics || {};
  
  return {
    id: userInfo.id || playerId,
    username: userInfo.username || userInfo.name || `player_${playerId}`,
    avatar_url: userInfo.avatar_url || userInfo.avatar || userInfo.picture || '',
    country: userInfo.country_code || userInfo.country || 'XX',
    rank: stats.global_rank || stats.pp_rank || userInfo.rank || userInfo.global_rank || 0,
  };
}

/**
 * Main execution
 */
async function main() {
  try {
    // Read and parse participants file
    console.error('Reading participants file...');
    const fileContent = fs.readFileSync(participantsFile, 'utf-8');
    const playerIds = extractPlayerIds(fileContent);
    
    if (playerIds.length === 0) {
      console.error('Error: No player IDs found in the participants file.');
      process.exit(1);
    }
    
    console.error(`Found ${playerIds.length} player IDs: ${playerIds.slice(0, 5).join(', ')}${playerIds.length > 5 ? '...' : ''}`);
    
    // Obtain OAuth2 access token
    console.error('Obtaining OAuth2 access token...');
    const accessToken = await getAccessToken();
    console.error('Access token obtained successfully.');
    
    // Fetch player information
    console.error('Fetching player information...');
    const players = [];
    const errors = [];
    
    for (const playerId of playerIds) {
      try {
        console.error(`Fetching info for player ${playerId}...`);
        const userInfo = await fetchPlayerInfo(playerId, accessToken);
        const player = mapToPlayer(playerId, userInfo);
        players.push(player);
      } catch (error) {
        errors.push({ playerId, error: error.message });
        console.error(`Error fetching player ${playerId}: ${error.message}`);
      }
    }
    
    console.error(`\nSuccessfully fetched ${players.length} players.`);
    if (errors.length > 0) {
      console.error(`Failed to fetch ${errors.length} players.`);
    }
    
    // Output JSON array to stdout
    console.log(JSON.stringify(players, null, 2));
    
  } catch (error) {
    console.error(`Fatal error: ${error.message}`);
    process.exit(1);
  }
}

main();

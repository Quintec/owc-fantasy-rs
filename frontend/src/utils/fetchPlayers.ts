/**
 * Client-side player data fetcher
 * Extracts player IDs from markdown text and fetches player info from osu! API
 */

interface Player {
  id: number;
  username: string;
  avatar_url: string;
  country: string;
  rank: number;
}

/**
 * Extract player IDs from markdown text containing osu! profile links
 */
function extractPlayerIds(text: string): number[] {
  const regex = /osu\.ppy\.sh\/users\/(\d+)/g;
  const ids = new Set<number>();
  
  let match;
  while ((match = regex.exec(text)) !== null) {
    ids.add(parseInt(match[1], 10));
  }
  
  return Array.from(ids);
}

/**
 * Get OAuth2 access token using client credentials
 */
async function getAccessToken(
  tokenUrl: string,
  clientId: string,
  clientSecret: string
): Promise<string> {
  const body = new URLSearchParams({
    grant_type: 'client_credentials',
    client_id: clientId,
    client_secret: clientSecret,
  });

  const response = await fetch(tokenUrl, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded',
    },
    body: body.toString(),
  });

  if (!response.ok) {
    throw new Error(`Token request failed: ${response.status}`);
  }

  const data = await response.json();
  
  if (!data.access_token) {
    throw new Error('No access_token in response');
  }

  return data.access_token;
}

/**
 * Fetch player info from osu! API
 */
async function fetchPlayerInfo(
  playerId: number,
  accessToken: string,
  userinfoUrl: string
): Promise<Player> {
  const url = userinfoUrl.replace('{id}', playerId.toString());
  
  const response = await fetch(url, {
    headers: {
      'Authorization': `Bearer ${accessToken}`,
    },
  });

  if (!response.ok) {
    throw new Error(`Failed to fetch player ${playerId}: HTTP ${response.status}`);
  }

  const userInfo = await response.json();
  const stats = userInfo.statistics || {};

  return {
    id: userInfo.id || playerId,
    username: userInfo.username || `player_${playerId}`,
    avatar_url: userInfo.avatar_url || '',
    country: userInfo.country_code || 'XX',
    rank: stats.global_rank || 0,
  };
}

export interface FetchPlayersResult {
  players: Player[];
  errors: string[];
}

/**
 * Main function: Extract IDs from markdown, fetch player data via OAuth2
 */
export async function fetchPlayersFromParticipants(
  participantsText: string,
  oauthConfig: {
    tokenUrl: string;
    clientId: string;
    clientSecret: string;
    userinfoUrl: string;
  }
): Promise<FetchPlayersResult> {
  // Extract player IDs
  const playerIds = extractPlayerIds(participantsText);
  
  if (playerIds.length === 0) {
    throw new Error('No player IDs found in the provided text');
  }

  // Get OAuth2 token
  const accessToken = await getAccessToken(
    oauthConfig.tokenUrl,
    oauthConfig.clientId,
    oauthConfig.clientSecret
  );

  // Fetch all players
  const players: Player[] = [];
  const errors: string[] = [];

  for (const playerId of playerIds) {
    try {
      const player = await fetchPlayerInfo(playerId, accessToken, oauthConfig.userinfoUrl);
      players.push(player);
    } catch (err: any) {
      errors.push(`Player ${playerId}: ${err.message}`);
    }
  }

  return { players, errors };
}

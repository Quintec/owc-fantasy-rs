# Player Data Fetcher

A Node.js script to extract player IDs from tournament participants and fetch their profile information via OAuth2.

## Prerequisites

- Node.js (v14 or higher)
- OAuth2 credentials with access to user profile API

## Environment Variables

Set these environment variables before running the script:

```bash
export OAUTH_TOKEN_URL="https://oauth.example.com/token"
export OAUTH_CLIENT_ID="your_client_id"
export OAUTH_CLIENT_SECRET="your_client_secret"
export OAUTH_USERINFO_URL="https://api.example.com/users/{id}"
```

The `OAUTH_USERINFO_URL` should contain `{id}` as a placeholder that will be replaced with each player ID.

### For osu! API v2

If you're using the osu! API:

```bash
export OAUTH_TOKEN_URL="https://osu.ppy.sh/oauth/token"
export OAUTH_CLIENT_ID="your_osu_client_id"
export OAUTH_CLIENT_SECRET="your_osu_client_secret"
export OAUTH_USERINFO_URL="https://osu.ppy.sh/api/v2/users/{id}"
```

## Usage

```bash
node fetch_players.js <participants_file>
```

### Example

```bash
node fetch_players.js participants.txt > players.json
```

## Participants File Format

The script is flexible and can extract player IDs from various formats:

### Format 1: Markdown table with osu! profile links (recommended for OWC)
```markdown
## Participants

|  | Country | Members |
| :-: | :-: | :-- |
| ::{ flag=AR }:: | **Argentina** | **[Bomilk](https://osu.ppy.sh/users/7081596)**, [Amuro](https://osu.ppy.sh/users/7119659) |
| ::{ flag=AU }:: | **Australia** | **[mrekk](https://osu.ppy.sh/users/7562902)**, [cloppit](https://osu.ppy.sh/users/19851850) |
```

You can copy-paste this directly from the osu! wiki or tournament pages!

### Format 2: One ID per line
```
123456
789012
345678
```

### Format 3: Comma-separated
```
123456, 789012, 345678, 901234
```

### Format 4: JSON array
```json
[123456, 789012, 345678]
```

### Format 5: Mixed format (e.g., from a tournament spreadsheet)
```
Player 1: 123456
Player 2: 789012
Player 3: 345678
```

The script automatically extracts all player IDs from osu! profile URLs (most reliable) or falls back to extracting numeric IDs from the file.

## Output Format

The script outputs a JSON array to stdout, with each player object matching the Player struct from `models.rs`:

```json
[
  {
    "id": 123456,
    "username": "player_name",
    "avatar_url": "https://example.com/avatar.jpg",
    "country": "US",
    "rank": 1500
  },
  {
    "id": 789012,
    "username": "another_player",
    "avatar_url": "https://example.com/avatar2.jpg",
    "country": "JP",
    "rank": 2300
  }
]
```

## Field Mapping

The script attempts to map the OAuth provider's response fields to the Player struct. You may need to adjust the `mapToPlayer` function in `fetch_players.js` based on your provider's API response format.

Default mappings:
- `id` ← `userInfo.id` or the original player ID
- `username` ← `userInfo.username` or `userInfo.name`
- `avatar_url` ← `userInfo.avatar_url` or `userInfo.avatar` or `userInfo.picture`
- `country` ← `userInfo.country` or `userInfo.country_code`
- `rank` ← `userInfo.rank` or `userInfo.pp_rank` or `userInfo.global_rank`

### For osu! API v2

The osu! API returns statistics in a nested structure. You may want to modify the `mapToPlayer` function:

```javascript
function mapToPlayer(playerId, userInfo) {
  const stats = userInfo.statistics || {};
  return {
    id: userInfo.id || playerId,
    username: userInfo.username || `player_${playerId}`,
    avatar_url: userInfo.avatar_url || '',
    country: userInfo.country_code || 'XX',
    rank: stats.global_rank || stats.pp_rank || 0,
  };
}
```

## Error Handling

- Errors are logged to stderr
- The script continues processing remaining players if one fails
- Successful results are still output even if some players fail
- Exit code 1 indicates a fatal error (missing env vars, file not found, etc.)

## Example Workflow

1. Create a participants file:
```bash
echo "123456
789012
345678" > participants.txt
```

2. Set environment variables:
```bash
export OAUTH_TOKEN_URL="https://osu.ppy.sh/oauth/token"
export OAUTH_CLIENT_ID="1234"
export OAUTH_CLIENT_SECRET="secret_here"
export OAUTH_USERINFO_URL="https://osu.ppy.sh/api/v2/users/{id}"
```

3. Run the script:
```bash
node fetch_players.js participants.txt > players.json
```

4. Import to database (using an API endpoint or direct SQL):
```bash
# Example: POST to bulk_create endpoint
curl -X POST http://localhost:8080/api/players/bulk_create \
  -H "Content-Type: application/json" \
  -d @players.json
```

## Troubleshooting

### "No player IDs found in the participants file"
- Ensure the file contains numeric IDs
- Check the file is not empty
- Verify the file encoding is UTF-8

### "Token request failed"
- Verify your OAuth credentials are correct
- Check the token URL is accessible
- Ensure client credentials grant type is enabled for your OAuth app

### "User info request failed"
- Verify the userinfo URL template is correct
- Check the player ID exists in the system
- Ensure your OAuth app has permission to access user profiles

## License

MIT

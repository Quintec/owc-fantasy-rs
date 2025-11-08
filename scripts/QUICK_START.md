# Quick Start: Fetch Tournament Players

This guide shows you how to quickly fetch all player data for a tournament using the `fetch_players.js` script.

## TL;DR

```bash
# 1. Copy your .env file or set environment variables
cp .env.example .env
# Edit .env and add your OAUTH_CLIENT_ID and OAUTH_CLIENT_SECRET

# 2. Copy-paste the participants table from osu! wiki into a file
# Visit: https://osu.ppy.sh/wiki/en/Tournaments/OWC/2024
# Copy the entire "Participants" section
cat > participants.md
# Paste the markdown table here, then press Ctrl+D

# 3. Run the script
node fetch_players.js participants.md > players.json

# 4. Import to database (using existing bulk_create endpoint)
# You'll need admin authentication for this
```

## Step-by-Step Guide

### 1. Set up OAuth credentials

Edit your `.env` file and ensure these variables are set:

```bash
OAUTH_CLIENT_ID=your_client_id
OAUTH_CLIENT_SECRET=your_client_secret
OAUTH_TOKEN_URL=https://osu.ppy.sh/oauth/token
OAUTH_USERINFO_URL=https://osu.ppy.sh/api/v2/users/{id}
```

To get OAuth credentials:
1. Go to https://osu.ppy.sh/home/account/edit
2. Scroll to "OAuth" section
3. Click "New OAuth Application"
4. Fill in the details (callback URL doesn't matter for this script)
5. Copy the Client ID and Client Secret

### 2. Get tournament participants data

**Option A: From osu! wiki (recommended)**

1. Visit the tournament page, e.g., https://osu.ppy.sh/wiki/en/Tournaments/OWC/2024
2. Find the "Participants" section
3. Copy the entire markdown table (including headers)
4. Paste into a file named `participants.md`

**Option B: Manual list**

Create a text file with player IDs (one per line or any format):
```
7081596
7119659
7562902
```

### 3. Run the script

```bash
# Load environment variables if using .env file
source .env  # or: export $(cat .env | xargs)

# Run the script and save output
node fetch_players.js participants.md > players.json

# Or run with explicit environment variables
OAUTH_CLIENT_ID=xxx OAUTH_CLIENT_SECRET=yyy \
  node fetch_players.js participants.md > players.json
```

The script will:
- Extract all player IDs from the file
- Obtain an OAuth2 access token
- Fetch each player's profile information
- Output a JSON array to `players.json`

### 4. Verify the output

```bash
# Check how many players were fetched
cat players.json | jq '. | length'

# View a sample player
cat players.json | jq '.[0]'
```

Expected output:
```json
{
  "id": 7081596,
  "username": "Bomilk",
  "avatar_url": "https://a.ppy.sh/7081596",
  "country": "AR",
  "rank": 1234
}
```

### 5. Import to database

**Option A: Using the bulk_create endpoint (requires admin auth)**

```bash
curl -X POST http://localhost:8080/api/players/bulk_create \
  -H "Content-Type: application/json" \
  -H "Cookie: session=your_admin_session_cookie" \
  -d @players.json
```

**Option B: Direct SQL import (if you have DB access)**

```bash
# Convert JSON to SQL INSERT statements
cat players.json | jq -r '.[] | "INSERT INTO Players (id, username, avatar_url, country, `rank`) VALUES (\(.id), \"\(.username)\", \"\(.avatar_url)\", \"\(.country)\", \(.rank));"' > import.sql

# Run the SQL
mysql -u rustuser -p owc_fantasy < import.sql
```

## Testing the parser

Before running the full script, test that ID extraction works:

```bash
node test_parser.js participants.md
```

This will show you all extracted IDs without making any API calls.

## Troubleshooting

### "Missing required environment variables"
- Make sure all OAuth variables are set
- Check for typos in variable names
- Use `echo $OAUTH_CLIENT_ID` to verify they're loaded

### "Token request failed"
- Verify your client ID and secret are correct
- Check that your OAuth app is active
- Ensure you're using client credentials grant type

### "No player IDs found"
- Run `node test_parser.js participants.md` to debug
- Ensure the file contains osu! profile links or numeric IDs
- Check file encoding (should be UTF-8)

### Players have rank 0
- This means the osu! API didn't return rank data
- Could be restricted accounts or API changes
- Check the raw API response by adding debug logs to `fetch_players.js`

## Example: OWC 2024

```bash
# 1. Get the participants page
curl -s "https://osu.ppy.sh/wiki/en/Tournaments/OWC/2024" | \
  grep -A 1000 "## Participants" | \
  grep -B 1000 "## Mappools" > participants.md

# 2. Run the script
node fetch_players.js participants.md > owc2024_players.json

# 3. Check results
echo "Fetched $(cat owc2024_players.json | jq '. | length') players"

# 4. Import to DB
curl -X POST http://localhost:8080/api/players/bulk_create \
  -H "Content-Type: application/json" \
  --cookie "session=$(cat ~/.owc_session)" \
  -d @owc2024_players.json
```

## Advanced: Rate Limiting

If you're fetching many players and hit rate limits, modify the script to add delays:

```javascript
// In the main() function, after the for loop line:
for (const playerId of playerIds) {
  // Add a delay between requests (e.g., 100ms)
  await new Promise(resolve => setTimeout(resolve, 100));
  
  try {
    // ... existing code
```

## Next Steps

- Update player ranks periodically by re-running the script
- Add eliminated flags manually or via the admin UI
- Set player prices using the `/api/players/{id}/price/{round}` endpoint

# Player Import Scripts

This folder contains scripts for importing tournament player data into the OWC Fantasy database.

## Overview

The player import feature allows admins to:
1. Copy-paste participant tables from osu! wiki pages (markdown format)
2. Automatically extract player IDs from profile links
3. Fetch player data via OAuth2 from the osu! API
4. Import all players into the database in bulk

## Setup

### Option 1: Frontend UI (Recommended)

The easiest way is to use the Admin page in the frontend:

1. **Configure OAuth credentials in frontend `.env`:**
   ```bash
   cd frontend
   cp .env.example .env
   # Edit .env and add your OAuth credentials
   ```

2. **Set these variables:**
   ```
   VITE_OAUTH_CLIENT_ID=your_client_id
   VITE_OAUTH_CLIENT_SECRET=your_client_secret
   VITE_OAUTH_TOKEN_URL=https://osu.ppy.sh/oauth/token
   VITE_OAUTH_USERINFO_URL=https://osu.ppy.sh/api/v2/users/{id}
   ```

3. **Use the Admin page:**
   - Navigate to `/admin` in the frontend
   - Paste the markdown participants table
   - Click "Import Players"
   - Players are automatically fetched and imported

### Option 2: Command-Line Script

If you prefer to run the script manually:

1. **Set environment variables:**
   ```bash
   export OAUTH_CLIENT_ID="your_client_id"
   export OAUTH_CLIENT_SECRET="your_client_secret"
   export OAUTH_TOKEN_URL="https://osu.ppy.sh/oauth/token"
   export OAUTH_USERINFO_URL="https://osu.ppy.sh/api/v2/users/{id}"
   ```

2. **Run the script:**
   ```bash
   node fetch_players.js participants.md > players.json
   ```

3. **Import to database via API:**
   ```bash
   curl -X POST http://localhost:8080/api/players/bulk_create \
     -H "Content-Type: application/json" \
     -H "Cookie: session=your_admin_session" \
     -d @players.json
   ```

## Getting OAuth Credentials

1. Go to https://osu.ppy.sh/home/account/edit
2. Scroll to "OAuth" section
3. Click "New OAuth Application"
4. Fill in the form:
   - Name: "OWC Fantasy Import"
   - Callback URL: (any URL, not used for client credentials)
5. Copy the Client ID and Client Secret

## Participants File Format

The import feature accepts markdown tables from osu! wiki pages. Example:

```markdown
## Participants

|  | Country | Members |
| :-: | :-: | :-- |
| ::{ flag=US }:: | **United States** | **[player1](https://osu.ppy.sh/users/123456)**, [player2](https://osu.ppy.sh/users/789012) |
| ::{ flag=JP }:: | **Japan** | **[player3](https://osu.ppy.sh/users/345678)**, [player4](https://osu.ppy.sh/users/901234) |
```

The script automatically extracts all player IDs from `osu.ppy.sh/users/{id}` links.

## Files

- `fetch_players.js` - Node.js script to extract IDs and fetch player data
- `test_parser.js` - Test utility to verify ID extraction
- `participants_test.md` - Example participants file
- `participants.example.txt` - Example input formats
- `FETCH_PLAYERS_README.md` - Detailed documentation
- `QUICK_START.md` - Quick reference guide

## Troubleshooting

### "OAuth credentials not configured"
- Make sure `VITE_OAUTH_CLIENT_ID` and `VITE_OAUTH_CLIENT_SECRET` are set in `frontend/.env`
- Restart the frontend dev server after changing `.env`

### "Token request failed"
- Verify your OAuth credentials are correct
- Check that your OAuth app is active on osu.ppy.sh
- Ensure you're using the correct token URL

### "No player IDs found"
- Verify the markdown contains osu! profile links
- Test extraction with: `node test_parser.js participants.md`

### Players have rank 0
- This is normal for restricted accounts or API issues
- The import will still succeed with rank 0

## Security Notes

- OAuth credentials should be kept secure
- Only admins can access the import feature (protected by admin middleware)
- Frontend credentials are exposed in the browser, so use a dedicated OAuth app for imports only
- Consider setting up backend-only imports for production

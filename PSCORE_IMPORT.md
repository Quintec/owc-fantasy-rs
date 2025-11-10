# pScore Import Feature

## Overview
This feature allows administrators to import player performance scores (pScores) and automatically calculate and update player prices for the current round.

## How to Use

### 1. Navigate to Admin Page
- Log in as an admin user
- Go to the Admin page

### 2. Prepare Your Data
Format your pScore data with one player per line, in the format:
```
username<TAB or SPACES>pscore
```

Example:
```
scylla	1.692
Raikouhou	1.659
[MG]Arnold24x24	1.649
Kamensh1k	1.614
badeu	1.599
misha awa	1.562
Intercambing	1.550
NathanRam1918	1.548
lolol235	1.484
Kyujin	1.468
lolol233	1.458
NINERIK	1.451
```

### 3. Import pScores
1. Find the "Import Player Prices from pScores" section
2. Paste your pScore data into the textarea
3. Click "Import pScores & Calculate Prices"

### 4. Review Results
The system will:
- Match usernames (case-insensitive) to players in the database
- Calculate prices using the formula: `price = pscore * 1000` (rounded to nearest integer)
- Update prices for the **current round**
- Show you:
  - Number of players successfully updated
  - Players that were skipped (not found in database)
  - Any errors that occurred

## Backend Details

### Endpoint
```
POST /api/players/import_pscores
```

### Request Body
```json
{
  "pscore_text": "scylla\t1.692\nRaikouhou\t1.659\n...",
  "round": "ro16"
}
```

### Response
```json
{
  "updated_count": 12,
  "skipped": ["unknown_player (not found in database)"],
  "errors": []
}
```

### Price Calculation Formula
Currently using: `price = pscore * 1000` (rounded)

You can adjust this formula in `backend/src/scripts/players.rs` in the `pscore_to_price()` function.

## Implementation Files

### Backend
- `backend/src/scripts/players.rs` - Contains `parse_pscores()`, `pscore_to_price()`, and `players_import_pscores()` endpoint
- `backend/src/api/players.rs` - Registers the endpoint in the players controller

### Frontend
- `frontend/src/api/pscores.ts` - API client function
- `frontend/src/pages/Admin.tsx` - UI for importing pScores

## Notes
- Username matching is **case-insensitive**
- Players not found in the database will be listed in the "skipped" array
- Prices are always updated for the **current round** (automatically detected)
- The import uses tab or whitespace as delimiter between username and pScore


import axios from "axios";
import type { PlayerProps } from "../types";

const API_BASE = import.meta.env.VITE_API_URL || "http://localhost:8080";

const defaultAxiosConfig = {
    withCredentials: true,
};

export async function getAllPlayers(config = {}) {
    try {
        const res = await axios.get(`${API_BASE}/api/players`, {
            ...defaultAxiosConfig,
            ...config,
        });
        return res.data as PlayerProps[];
    } catch (err) {
        console.error("getAllPlayers error:", err);
        throw err;
    }
}

export async function getRemainingPlayers(config = {}) {
    try {
        const res = await axios.get(`${API_BASE}/api/players/remaining`, {
            ...defaultAxiosConfig,
            ...config,
        });
        return res.data as PlayerProps[];
    } catch (err) {
        console.error("getRemainingPlayers error:", err);
        throw err;
    }
}

export async function getRemainingPlayersWithPrices(round: string, config = {}) {
    try {
        const res = await axios.get(`${API_BASE}/api/players/remaining/${round}`, {
            ...defaultAxiosConfig,
            ...config,
        });
        return res.data as PlayerProps[];
    } catch (err) {
        console.error("getRemainingPlayers error:", err);
        throw err;
    }
}

export async function getTeamPlayers(teamId: number, config = {}) {
    try {
        const res = await axios.get(`${API_BASE}/api/teams/${teamId}`, {
            ...defaultAxiosConfig,
            ...config,
        });
        return res.data as PlayerProps[];
    } catch (err) {
        console.error(`getTeamPlayers(${teamId}) error:`, err);
        throw err;
    }
}

    // Admin helper: parse multiplayer links (backend must implement endpoint to accept these links)
    export async function parseMultiplayerLinks(links: string[], round?: string, config = {}) {
        try {
            const res = await axios.post(`${API_BASE}/api/admin/parse-multiplayer`, { links, round }, {
                ...defaultAxiosConfig,
                ...config,
            });
            return res.data;
        } catch (err) {
            console.error('parseMultiplayerLinks error:', err);
            throw err;
        }
    }

    // Admin helper: eliminate players in the tournament (not tied to any user/team).
    // By default this will target players returned by `/api/players/remaining` (players still in the tournament).
    // If `country` is provided, it can be a single country code or comma-separated codes (e.g., "AR, BY, BE").
    export async function eliminatePlayers(country?: string, config = {}) {
        try {
            // fetch players still remaining in tournament
            const players = await getRemainingPlayers(config);

            let toEliminate: PlayerProps[];
            
            if (country) {
                // Parse comma-separated country codes
                const countryCodes = country
                    .split(',')
                    .map(c => c.trim().toLowerCase())
                    .filter(c => c.length > 0);
                
                toEliminate = players.filter((p: PlayerProps) => 
                    countryCodes.includes(p.country?.toLowerCase() || '')
                );
            } else {
                toEliminate = players;
            }

            for (const p of toEliminate) {
                await axios.post(`${API_BASE}/api/players/${p.id}/eliminate`, {}, {
                    ...defaultAxiosConfig,
                    ...config,
                });
            }

            return { eliminated: toEliminate.map((p: PlayerProps) => p.id) };
        } catch (err) {
            console.error(`eliminatePlayers(country=${country}) error:`, err);
            throw err;
        }
    }

    // Admin helper: un-eliminate players in the tournament (admin-only endpoint).
    // This will call POST /api/players/{id}/uneliminate for each matched player.
    // If `country` is provided, it can be a single country code or comma-separated codes (e.g., "AR, BY, BE").
    export async function unEliminatePlayers(country?: string, config = {}) {
        try {
            // Note: getRemainingPlayers returns players where eliminated = false. To find
            // players that are eliminated (so we can un-eliminate them), we should fetch all players
            // and then filter by eliminated === true. However, the API doesn't currently expose
            // a dedicated "eliminated" endpoint. We'll fetch all players and filter client-side.
            const all = await getAllPlayers(config);
            const eliminatedPlayers = all.filter((p: PlayerProps) => (p as any).eliminated);

            let toUnEliminate: PlayerProps[];
            
            if (country) {
                // Parse comma-separated country codes
                const countryCodes = country
                    .split(',')
                    .map(c => c.trim().toLowerCase())
                    .filter(c => c.length > 0);
                
                toUnEliminate = eliminatedPlayers.filter((p: PlayerProps) => 
                    countryCodes.includes(p.country?.toLowerCase() || '')
                );
            } else {
                toUnEliminate = eliminatedPlayers;
            }

            for (const p of toUnEliminate) {
                await axios.post(`${API_BASE}/api/players/${p.id}/uneliminate`, {}, {
                    ...defaultAxiosConfig,
                    ...config,
                });
            }

            return { uneliminated: toUnEliminate.map((p: PlayerProps) => p.id) };
        } catch (err) {
            console.error(`unEliminatePlayers(country=${country}) error:`, err);
            throw err;
        }
    }

export async function getAllUserTeams(userId: number, config = {}) {
    try {
        const url = `${API_BASE}/api/users/${userId}/teams`;
        const res = await axios.get(url, {
            ...defaultAxiosConfig,
            ...config,
        });
        return res.data as PlayerProps[][];
    } catch (err) {
        console.error(`getUserTeams(${userId}) error:`, err);
        throw err;
    }
}

// Admin helper: bulk create players
export async function bulkCreatePlayers(players: PlayerProps[], config = {}) {
    try {
        const res = await axios.post(
            `${API_BASE}/api/players/bulk_create`,
            players,
            {
                ...defaultAxiosConfig,
                ...config,
            }
        );
        return res.data;
    } catch (err) {
        console.error('bulkCreatePlayers error:', err);
        throw err;
    }
}

// Admin helper: import players from participants markdown
// Sends the markdown to backend, which handles OAuth and DB insertion
export async function importPlayersFromParticipants(participantsText: string, config = {}) {
    try {
        const res = await axios.post(
            `${API_BASE}/api/players/import_from_participants`,
            { participants_text: participantsText },
            {
                ...defaultAxiosConfig,
                ...config,
            }
        );
        return res.data as {
            players: PlayerProps[];
            count: number;
            errors: string[];
        };
    } catch (err) {
        console.error('importPlayersFromParticipants error:', err);
        throw err;
    }
}


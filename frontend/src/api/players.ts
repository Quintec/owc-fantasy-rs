
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
    // If `country` is provided, only players with that country code will be eliminated.
    export async function eliminatePlayers(country?: string, config = {}) {
        try {
            // fetch players still remaining in tournament
            const players = await getRemainingPlayers(config);

            const toEliminate = country
                ? players.filter((p: PlayerProps) => p.country?.toLowerCase() === country.toLowerCase())
                : players;

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
    export async function unEliminatePlayers(country?: string, config = {}) {
        try {
            // Note: getRemainingPlayers returns players where eliminated = false. To find
            // players that are eliminated (so we can un-eliminate them), we should fetch all players
            // and then filter by eliminated === true. However, the API doesn't currently expose
            // a dedicated "eliminated" endpoint. We'll fetch all players and filter client-side.
            const all = await getAllPlayers(config);
            const eliminatedPlayers = all.filter((p: PlayerProps) => (p as any).eliminated);

            const toUnEliminate = country
                ? eliminatedPlayers.filter((p: PlayerProps) => p.country?.toLowerCase() === country.toLowerCase())
                : eliminatedPlayers;

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


import axios from "axios";
import type { PlayerProps, User, Team } from "../types";

const API_BASE = import.meta.env.VITE_API_URL || "http://localhost:8080";

const defaultAxiosConfig = {
    withCredentials: true,
};

export async function getUsers(config = {}) {
    try {
        const res = await axios.get(`${API_BASE}/api/users`, {
            ...defaultAxiosConfig,
            ...config,
        });
        return res.data as User[];
    } catch (err) {
        console.error("getUsers error:", err);
        throw err;
    }
}

export async function getLeaderboard(config = {}) {
    try {
        const res = await axios.get(`${API_BASE}/api/users/leaderboard`, {
            ...defaultAxiosConfig,
            ...config,
        });
        return res.data as User[];
    } catch (err) {
        console.error("getLeaderboard error:", err);
        throw err;
    }
}

export async function getLeaderboardByRound(round: string, config = {}) {
    try {
        // "all" uses the aggregate endpoint for compatibility with the old behavior
        const endpoint = round === "all" 
            ? `${API_BASE}/api/users/leaderboard`
            : `${API_BASE}/api/users/leaderboard/${round}`;

        const res = await axios.get(endpoint, {
            ...defaultAxiosConfig,
            ...config,
        });
        return res.data as User[];
    } catch (err) {
        console.error("getLeaderboardByRound error:", err);
        throw err;
    }
}

export async function getUserTeamByRound(userId: number, round: string, config = {}): Promise<Team> {
    try {
        const url = `${API_BASE}/api/users/${userId}/teams/${round}`;
        const res = await axios.get(url, {
            ...defaultAxiosConfig,
            ...config,
        });
        return res.data as Team;
    } catch (err) {
        console.error(`getUserTeams(${userId}) error:`, err);
        throw err;
    }
}

// Fetch team players with captain information properly marked
export async function getTeamPlayersWithCaptain(teamId: number, captainId?: number | null, config = {}): Promise<PlayerProps[]> {
    try {
        const res = await axios.get(`${API_BASE}/api/teams/${teamId}`, {
            ...defaultAxiosConfig,
            ...config,
        });
        
        const players = res.data as PlayerProps[];
        
        // Mark the captain and sort players so captain is first
        const playersWithCaptain = players.map(p => ({
            ...p,
            captain: captainId ? p.id === captainId : false
        }));
        
        // Sort: captain first, then by ID for consistency
        playersWithCaptain.sort((a, b) => {
            if (a.captain && !b.captain) return -1;
            if (!a.captain && b.captain) return 1;
            return a.id - b.id;
        });
        
        return playersWithCaptain;
    } catch (err) {
        console.error(`getTeamPlayersWithCaptain error:`, err);
        throw err;
    }
}

// Create or update a team for a given userId with the provided player IDs and captain.
// Uses the /teams/{user_id}/{round} endpoint which creates/updates the team atomically.
export async function postPlayers(
    userId: number, 
    playerIds: number[], 
    round: string, 
    captainId?: number,
    config = {}
): Promise<PlayerProps[]> {
    try {
        // Send all player IDs and captain in one request
        await axios.post(
            `${API_BASE}/api/teams/${userId}/${round}`,
            {
                player_ids: playerIds,
                captain_id: captainId || (playerIds.length > 0 ? playerIds[0] : null),
            },
            {
                ...defaultAxiosConfig,
                ...config,
            }
        );

        // Fetch the team to get the team ID and captain_id
        const team = await getUserTeamByRound(userId, round, config);
        
        // Fetch the players for this team
        const playersRes = await axios.get(`${API_BASE}/api/teams/${team.id}`, {
            ...defaultAxiosConfig,
            ...config,
        });
        
        const players = playersRes.data as PlayerProps[];
        
        // Mark the captain and sort players so captain is first
        const playersWithCaptain = players.map(p => ({
            ...p,
            captain: p.id === team.captain_id
        }));
        
        // Sort: captain first, then by ID for consistency
        playersWithCaptain.sort((a, b) => {
            if (a.captain && !b.captain) return -1;
            if (!a.captain && b.captain) return 1;
            return a.id - b.id;
        });
        
        return playersWithCaptain;
    } catch (err) {
        console.error("postPlayers error:", err);
        throw err;
    }
}

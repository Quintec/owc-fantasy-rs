import axios from "axios";
import type { PlayerProps, User } from "../types";

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

export async function getUserTeamByRound(userId: number, round: string, config = {}) {
    try {
        const url = `${API_BASE}/api/users/${userId}/teams/${round}`;
        const res = await axios.get(url, {
            ...defaultAxiosConfig,
            ...config,
        });
        return res.data as PlayerProps[];
    } catch (err) {
        console.error(`getUserTeams(${userId}) error:`, err);
        throw err;
    }
}

// To-do: handle captain_id separately?
// Create a team for a given userId and add the provided player IDs.
// This matches backend endpoints: POST /api/users/{id}/teams/{round}/create and POST /api/users/{id}/teams/{round} (one player at a time).
export async function postPlayers(userId: number, playerIds: number[], round: string, config = {}) {
    try {
        // Create the team for the user (requires same-id middleware on the server)
        await axios.post(`${API_BASE}/api/users/${userId}/teams/${round}/create`,
            {},
            {
                ...defaultAxiosConfig,
                ...config,
            }
        );

        // Add each player one-by-one (server expects { player_id })
        for (const pid of playerIds) {
            await axios.post(`${API_BASE}/api/users/${userId}/teams/${round}`,
                { player_id: pid },
                {
                    ...defaultAxiosConfig,
                    ...config,
                }
            );
        }

        // Return the created team for the round
        const team = await getUserTeamByRound(userId, round, config);
        return team as PlayerProps[];
    } catch (err) {
        console.error("postPlayers error:", err);
        throw err;
    }
}
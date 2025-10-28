import axios from "axios";
import type { PlayerProps, User, Team } from "../types";

const API_BASE = import.meta.env.VITE_API_URL || "http://localhost:8080";

const defaultAxiosConfig = {
    withCredentials: true, // include cookies for auth
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

export async function getUserTeams(userId: number, round?: string, config = {}) {
    try {
        const url = round
            ? `${API_BASE}/api/users/${userId}/teams/${round}`
            : `${API_BASE}/api/users/${userId}/teams`;
        const res = await axios.get(url, {
            ...defaultAxiosConfig,
            ...config,
        });
        return res.data as Team | Team[];
    } catch (err) {
        console.error(`getUserTeams(${userId}) error:`, err);
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

// Fetch all players for a given user (optionally for a specific round)
export async function getUserPlayers(userId: number, round?: string, config = {}) {
    try {
        const teams = await getUserTeams(userId, round, config);
        const teamsArr = Array.isArray(teams) ? teams : [teams];
        const playersNested = await Promise.all(teamsArr.map((t) => getTeamPlayers(t.id, config)));
        // flatten
        return playersNested.flat() as PlayerProps[];
    } catch (err) {
        console.error(`getUserPlayers(${userId}) error:`, err);
        throw err;
    }
}

export async function getPlayerPrice(playerId: number, round: string, config = {}) {
    try {
        const res = await axios.get(`${API_BASE}/api/players/${playerId}/price/${round}`, {
            ...defaultAxiosConfig,
            ...config,
        });
        // API returns a plain number
        return res.data as number;
    } catch (err) {
        console.error(`getPlayerPrice(${playerId}, ${round}) error:`, err);
        throw err;
    }
}

// Convenience: fetch users + their teams' players (optionally for a specific round) and remaining players
export async function fetchTeamsAndRemaining(round?: string) {
    try {
        // fetch users and remaining players in parallel
        const [users, remaining] = await Promise.all([getUsers(), getRemainingPlayers()]);

        // For each user fetch their team(s). Use parallel requests but limit concurrency if needed.
        const teamsByUserPromises = users.map(async (u) => {
            const teams = await getUserTeams(u.id, round);
            // teams may be a single Team (when round provided) or an array
            const teamsArr = Array.isArray(teams) ? teams : [teams];
            // fetch players for each team in parallel
            const teamsWithPlayers = await Promise.all(
                teamsArr.map(async (t) => ({ team: t, players: await getTeamPlayers(t.id) }))
            );
            return { user: u, teams: teamsWithPlayers };
        });

        const usersWithTeams = await Promise.all(teamsByUserPromises);

        return {
            users: usersWithTeams,
            remainingPlayers: remaining,
        };
    } catch (err) {
        console.error("fetchTeamsAndRemaining error:", err);
        throw err;
    }
}


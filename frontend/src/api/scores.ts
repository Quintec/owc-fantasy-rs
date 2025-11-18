import axios from "axios";
import type { PlayerProps } from "../types";

const API_BASE = import.meta.env.VITE_API_URL || "http://localhost:8080";

const defaultAxiosConfig = {
    withCredentials: true,
};

export async function getAllPlayersWithScores(config = {}) {
    try {
        const res = await axios.get(`${API_BASE}/api/players`, {
            ...defaultAxiosConfig,
            ...config,
        });
        return res.data as PlayerProps[];
    } catch (err) {
        console.error("getAllPlayersWithScores error:", err);
        throw err;
    }
}

export async function getPlayerScoresByRound(round: string, config = {}) {
    try {
        const res = await axios.get(`${API_BASE}/api/players/scores/${round}`, {
            ...defaultAxiosConfig,
            ...config,
        });
        return res.data as PlayerProps[];
    } catch (err) {
        console.error("getPlayerScoresByRound error:", err);
        throw err;
    }
}

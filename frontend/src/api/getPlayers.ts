import axios from "axios";
import type { PlayerProps } from "../types";

export async function getUserPlayers(params = {}, config = {}) {
    const apiBaseURL = import.meta.env.VITE_API_URL || "http://localhost:8080";
    try {
        const response = await axios.get(`${apiBaseURL}/api/players`, {
            params,      // query parameters
            credentials: "include", // Include cookies for authentication
            ...config,   // extra axios config like headers
        });
        return response.data; // return only the data payload
    } catch (error) {
        console.error("GET request error:", error);
        throw error; // rethrow so the caller can handle it
    }
}

export async function getAllPlayers(params = {}, config = {}) {
    const apiBaseURL = import.meta.env.VITE_API_URL || "http://localhost:8080";
    try {
        const response = await axios.get(`${apiBaseURL}/api/players`, {
            params,      // query parameters
            credentials: "include", // Include cookies for authentication
            ...config,   // extra axios config like headers
        });
        return response.data; // return only the data payload
    } catch (error) {
        console.error("GET request error:", error);
        throw error; // rethrow so the caller can handle it
    }
}


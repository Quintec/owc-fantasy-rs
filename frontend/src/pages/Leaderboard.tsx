import { useEffect, useState } from "react";
import UserList from "../components/UserList";
import type { User } from "../types";
import { getLeaderboard, getLeaderboardByRound } from "../api/users";

export default function Leaderboard() {

    const [users, setUsers] = useState<User[]>([]);
    const [loading, setLoading] = useState<boolean>(false);
    const [error, setError] = useState<string | null>(null);
    const [selectedRound, setSelectedRound] = useState<string>("all");

    const rounds = [
        { value: 'all', label: 'All Rounds' },
        { value: 'ro16', label: 'Round of 16' },
        { value: 'qf', label: 'Quarterfinals' },
        { value: 'sf', label: 'Semifinals' },
        { value: 'f', label: 'Finals' },
        { value: 'gf', label: 'Grand Finals' },
    ];

    useEffect(() => {
        const fetchData = async () => {
            if (!selectedRound) return;

            try {
                setLoading(true);
                // default aggregated view when "all" is chosen
                const data = selectedRound === "all" 
                    ? await getLeaderboard()
                    : await getLeaderboardByRound(selectedRound);
                setUsers(data);
                setError(null);
            } catch (err) {
                console.error("Error fetching leaderboard:", err);
                setError("Failed to load leaderboard. Please try again.");
            } finally {
                setLoading(false);
            }
        };
        fetchData();
    }, [selectedRound]);

    return (
        <div className="p-5 flex flex-col items-center">
            <div className="w-full max-w-4xl flex flex-col gap-4">
                <div className="flex items-center justify-between flex-wrap gap-3">
                    <h1 className="text-2xl font-bold text-white">Leaderboard</h1>
                    <select
                        className="px-4 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent transition-all duration-200"
                        value={selectedRound}
                        onChange={(e) => setSelectedRound(e.target.value)}
                    >
                        {rounds.map((round) => (
                            <option key={round.value} value={round.value}>
                                {round.label}
                            </option>
                        ))}
                    </select>
                </div>

                {loading ? (
                    <div className="text-white">Loading leaderboard...</div>
                ) : error ? (
                    <div className="text-red-400">{error}</div>
                ) : (
                    <div className="flex flex-col gap-4">
                        {users.map((u, index) => (
                            <UserList id={u.id} username={u.username} score={u.score} rank={index} key={u.id}/>
                        ))}
                    </div>
                )}
            </div>
        </div>
    );
    
}

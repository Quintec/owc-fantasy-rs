import { useState, useEffect } from "react";
import { getPlayerScoresByRound } from "../api/scores";
import type { PlayerProps } from "../types";
import { useRound } from "../contexts/RoundContext";

export default function Scores() {
    const [players, setPlayers] = useState<PlayerProps[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [searchQuery, setSearchQuery] = useState("");
    const { round: currentRound } = useRound();
    const [selectedRound, setSelectedRound] = useState<string>("");

    const rounds = [
        { value: 'ro16', label: 'Round of 16' },
        { value: 'qf', label: 'Quarterfinals' },
        { value: 'sf', label: 'Semifinals' },
        { value: 'f', label: 'Finals' },
        { value: 'gf', label: 'Grand Finals' },
    ];

    // Get display name for a round
    const getRoundDisplayName = (roundCode: string): string => {
        const roundObj = rounds.find(r => r.value === roundCode);
        return roundObj ? roundObj.label : roundCode.toUpperCase();
    };

    // Calculate previous round for initial selection
    const getPreviousRound = (currentRoundCode: string): string => {
        const roundMap: { [key: string]: string } = {
            'gf': 'f',
            'f': 'sf',
            'sf': 'qf',
            'qf': 'ro16',
            'ro16': 'ro32',
            'ro32': 'ro64',
            'ro64': 'ro64', // First round, no previous
        };
        return roundMap[currentRoundCode] || 'ro64';
    };

    // Initialize selected round to previous round on mount
    useEffect(() => {
        if (currentRound && !selectedRound) {
            setSelectedRound(getPreviousRound(currentRound));
        }
    }, [currentRound, selectedRound]);

    useEffect(() => {
        const fetchPlayers = async () => {
            if (!selectedRound) return;
            
            try {
                setLoading(true);
                const data = await getPlayerScoresByRound(selectedRound);
                // Sort by score descending (highest first)
                const sortedData = data.sort((a, b) => (b.score || 0) - (a.score || 0));
                setPlayers(sortedData);
                setError(null);
            } catch (err) {
                console.error("Failed to fetch players with scores:", err);
                setError("Failed to load player scores. Please try again later.");
            } finally {
                setLoading(false);
            }
        };

        fetchPlayers();
    }, [selectedRound]);

    const filteredPlayers = players.filter(player => 
        player.username.toLowerCase().includes(searchQuery.toLowerCase()) ||
        player.country.toLowerCase().includes(searchQuery.toLowerCase())
    );

    if (loading) {
        return (
            <div className="p-5 flex flex-col items-center">
                <h1 className="text-2xl font-bold text-white mb-5">Player Scores - Loading...</h1>
                <div className="text-white text-xl">Loading...</div>
            </div>
        );
    }

    if (error) {
        return (
            <div className="p-5 flex flex-col items-center">
                <h1 className="text-2xl font-bold text-white mb-5">Player Scores - {selectedRound ? getRoundDisplayName(selectedRound) : 'Loading...'}</h1>
                <div className="text-red-500 text-xl">{error}</div>
            </div>
        );
    }

    return (
        <div className="p-5 flex flex-col items-center">
            <h1 className="text-2xl font-bold text-white mb-5">Player Scores</h1>
            
            <div className="w-full max-w-6xl mb-5 flex gap-4">
                <div className="flex-1 relative">
                    <input 
                        type="search" 
                        className="w-full px-4 py-3 pl-12 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent transition-all duration-200" 
                        placeholder="Search by username or country code..."
                        onChange={e => setSearchQuery(e.target.value)}
                        value={searchQuery}
                    />
                    <div className="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
                        <svg className="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                        </svg>
                    </div>
                </div>
                
                <select 
                    className="px-4 py-3 bg-gray-800 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent transition-all duration-200"
                    value={selectedRound}
                    onChange={e => setSelectedRound(e.target.value)}
                >
                    {rounds.map(round => (
                        <option key={round.value} value={round.value}>
                            {round.label}
                        </option>
                    ))}
                </select>
            </div>

            <div className="w-full max-w-6xl bg-gray-800 rounded-lg shadow-lg overflow-hidden">
                <div className="overflow-x-auto">
                    <table className="w-full">
                        <thead className="bg-gray-700">
                            <tr>
                                <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                                    Rank
                                </th>
                                <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                                    Player
                                </th>
                                <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                                    Country
                                </th>
                                <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                                    Global Rank
                                </th>
                                <th className="px-6 py-3 text-right text-xs font-medium text-gray-300 uppercase tracking-wider">
                                    Score
                                </th>
                                <th className="px-6 py-3 text-center text-xs font-medium text-gray-300 uppercase tracking-wider">
                                    Status
                                </th>
                            </tr>
                        </thead>
                        <tbody className="divide-y divide-gray-700">
                            {filteredPlayers.map((player, index) => (
                                <tr key={player.id} className={`${index % 2 === 0 ? 'bg-gray-800' : 'bg-gray-750'} hover:bg-gray-700 transition-colors`}>
                                    <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-white">
                                        #{index + 1}
                                    </td>
                                    <td className="px-6 py-4 whitespace-nowrap">
                                        <div className="flex items-center">
                                            <img 
                                                src={player.avatar_url || `https://a.ppy.sh/${player.id}`} 
                                                alt={player.username}
                                                className="h-10 w-10 rounded-full mr-3"
                                            />
                                            <div className="text-sm font-medium text-white">
                                                {player.username}
                                            </div>
                                        </div>
                                    </td>
                                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-300">
                                        {player.country.toUpperCase()}
                                    </td>
                                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-300">
                                        #{player.rank.toLocaleString()}
                                    </td>
                                    <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-bold">
                                        <span className={`${
                                            (player.score || 0) > 0 
                                                ? 'text-green-400' 
                                                : 'text-gray-500'
                                        }`}>
                                            {(player.score || 0).toLocaleString()}
                                        </span>
                                    </td>
                                    <td className="px-6 py-4 whitespace-nowrap text-center">
                                        {player.eliminated ? (
                                            <span className="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-red-100 text-red-800">
                                                Eliminated
                                            </span>
                                        ) : (
                                            <span className="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-green-100 text-green-800">
                                                Active
                                            </span>
                                        )}
                                    </td>
                                </tr>
                            ))}
                        </tbody>
                    </table>
                </div>
            </div>

            {filteredPlayers.length === 0 && (
                <div className="text-gray-400 mt-5">No players found matching "{searchQuery}"</div>
            )}

            <div className="text-gray-400 mt-5 text-sm">
                Showing {filteredPlayers.length} of {players.length} players
            </div>
        </div>
    );
}

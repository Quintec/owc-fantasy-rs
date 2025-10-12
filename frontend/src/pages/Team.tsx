import { useState, useEffect, useRef } from "react";
import Player from "../components/Player";
import { getAllPlayers, getUserPlayers } from "../api/getPlayers";
import type { PlayerProps } from "../types";

export default function Team() {
    const [userPlayers, setUserPlayers] = useState<PlayerProps[]>([]);
    const [players, setAllPlayers] = useState<PlayerProps[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [drafting, setDraft] = useState(false);
    const [queryPlayer, setPlayerQuery] = useState("");

    // Manual test data - replace with real API call later
    useEffect(() => {
        const testPlayers: PlayerProps[] = [
            {
                id: 12835025,
                username: "txFPS",
                country: "CA",
                rank: 11412,
                price: 727,
                eliminated: false,
                captain: true
            },
            {
                id: 2,
                username: "012345678912345",
                country: "JP",
                rank: 2,
                price: 950000,
                eliminated: false,
                captain: false
            },
            {
                id: 3,
                username: "Player3",
                country: "KR",
                rank: 3,
                price: 900000,
                eliminated: false,
                captain: false
            },
            {
                id: 3,
                username: "Player3",
                country: "KR",
                rank: 3,
                price: 900000,
                eliminated: false,
                captain: false
            },
            {
                id: 3,
                username: "Player3",
                country: "KR",
                rank: 3,
                price: 900000,
                eliminated: false,
                captain: false
            },
            {
                id: 3,
                username: "Player3",
                country: "KR",
                rank: 3,
                price: 900000,
                eliminated: false,
                captain: false
            },
            {
                id: 3,
                username: "Player3",
                country: "KR",
                rank: 3,
                price: 900000,
                eliminated: false,
                captain: false
            },
            {
                id: 3,
                username: "Player3",
                country: "KR",
                rank: 3,
                price: 900000,
                eliminated: false,
                captain: false
            }
        ];
        setUserPlayers(testPlayers);
        setAllPlayers(testPlayers);
        setLoading(false);
    }, []);

    if (loading) {
        return (
            <div className="p-5 flex flex-col items-center">
                <h1 className="text-xl font-bold text-white mb-5 text-center">My Team</h1>
                <div className="text-white text-xl">Loading players...</div>
            </div>
        );
    }

    if (error) {
        return (
            <div className="p-5 flex flex-col items-center">
                <h1 className="text-xl font-bold text-white mb-5 text-center">My Team</h1>
                <div className="text-red-400 text-xl mb-4">{error}</div>
                <button 
                    className="bg-purple-500 text-white px-4 py-2 rounded-md"
                    onClick={() => window.location.reload()}
                >
                    Retry
                </button>
            </div>
        );
    }

    // useEffect(() => {
    //     const fetchPlayers = async () => {
    //         try {
    //             setLoading(true);
    //             const data = await getAllPlayers();
    //             setAllPlayers(data);
    //             players.filter(p => !p.eliminated)
    //             setError(null);
    //         } catch (err) {
    //             console.error("Failed to fetch players:", err);
    //             setError("Failed to load players. Please try again later.");
    //         } finally {
    //             setLoading(false);
    //         }
    //     };

    //     fetchPlayers();
    // }, []);

    if (loading) {
        return (
            <div className="p-5 flex flex-col items-center">
                <h1 className="text-xl font-bold text-white mb-5 text-center">My Team</h1>
                <div className="text-white text-xl">Loading players...</div>
            </div>
        );
    }

    // if (error) {
    //     return (
    //         <div className="p-5 flex flex-col items-center">
    //             <h1 className="text-xl font-bold text-white mb-5 text-center">My Team</h1>
    //             <div className="text-red-400 text-xl mb-4">{error}</div>
    //             <button 
    //                 className="bg-purple-500 text-white px-4 py-2 rounded-md"
    //                 onClick={() => window.location.reload()}
    //             >
    //                 Retry
    //             </button>
    //         </div>
    //     );
    // }


    if (drafting) {
        return (
            <div className="p-5 flex flex-col items-center">
                <div className="relative w-full max-w-md mb-6">
                    <input 
                        type="search" 
                        className="w-full px-4 py-3 pl-12 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent transition-all duration-200" 
                        placeholder="Search players..."
                        onChange={e => setPlayerQuery(e.target.value)}
                        value={queryPlayer}
                    />
                    <div className="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
                        <svg className="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                        </svg>
                    </div>
                    
                </div>
                
                <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
                {players.filter(p => p.username.toLowerCase().includes(queryPlayer) || p.country.toLowerCase().includes(queryPlayer)).map((player) => (
                    <Player 
                        id={player.id}
                        username={player.username}
                        country={player.country}
                        rank={player.rank}
                        price={player.price}
                        eliminated={player.eliminated}
                    />
                ))}
                </div>
                <button className="bg-purple-500 text-white px-4 py-2 my-5 rounded-md text-2xl min-w-1/4" onClick={() => {setDraft(false)}}>Done</button>    
            </div>
        )
    }


    return (
        <div className="p-5 flex flex-col items-center">
            <h1 className="text-xl font-bold text-white mb-5 text-center">My Team</h1>
            <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
                {userPlayers.map((player) => (
                    <Player 
                        id={player.id}
                        username={player.username}
                        country={player.country}
                        rank={player.rank}
                        price={player.price}
                        eliminated={player.eliminated}
                        captain={player.captain || false}
                    />
                ))}
            </div>
            <button className="bg-purple-500 text-white px-4 py-2 my-5 rounded-md text-2xl min-w-1/4" onClick={() => {setDraft(true)}}>Edit</button>
        </div>
    );
}

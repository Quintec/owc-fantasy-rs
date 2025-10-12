import { useState, useEffect } from "react";
import Player from "../components/Player";
import { getUserPlayers } from "../api/getPlayers";
import type { PlayerProps } from "../types";
import PlayerList from "../components/PlayerList";

export default function Team() {
    const [userPlayers, setUserPlayers] = useState<PlayerProps[]>([]);
    const [players, setAllPlayers] = useState<PlayerProps[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [drafting, setDraft] = useState(false);

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
    //             setPlayers(data);
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

    // if (loading) {
    //     return (
    //         <div className="p-5 flex flex-col items-center">
    //             <h1 className="text-xl font-bold text-white mb-5 text-center">My Team</h1>
    //             <div className="text-white text-xl">Loading players...</div>
    //         </div>
    //     );
    // }

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
                <input type="search" className="min-w-1/2 m-5 p-5 text-white" placeholder="Search player"/>
                <form>

                </form>

                {players.filter(p => !p.eliminated).map((player) => (
                    <PlayerList 
                        id={player.id}
                        username={player.username}
                        country={player.country}
                        rank={player.rank}
                        price={player.price}
                        eliminated={player.eliminated}
                    />
                ))}
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

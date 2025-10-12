import { useState, useEffect, useRef } from "react";
import Player from "../components/Player";
import { getAllPlayers, getUserPlayers } from "../api/getPlayers";
import type { PlayerProps } from "../types";
import PlayerList from "../components/PlayerList";

export default function Team() {
    const [userPlayers, setUserPlayers] = useState<PlayerProps[]>([]);
    const [players, setAllPlayers] = useState<PlayerProps[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [drafting, setDraft] = useState(false);
    const [queryPlayer, setPlayerQuery] = useState("");
    const [balance, setBalance] = useState(10000000); // replace with API call later
    const [notification, setNotification] = useState<{message: string, type: 'error' | 'success'} | null>(null);

    const showNotification = (message: string, type: 'error' | 'success') => {
        setNotification({ message, type });
        setTimeout(() => setNotification(null), 3000);
    };

    // Manual test data - replace with real API call later
    useEffect(() => {
        const testPlayers: PlayerProps[] = [
            { id: 124493, username: "123465789123456", country: "KR", rank: 1, price: 1000000, eliminated: false, captain: false, drafted: false },
            { id: 124494, username: "Vaxei", country: "US", rank: 2, price: 950000, eliminated: false, captain: false, drafted: false },
            { id: 124495, username: "WhiteCat", country: "PL", rank: 3, price: 900000, eliminated: false, captain: false, drafted: false },
            { id: 124496, username: "mrekk", country: "US", rank: 4, price: 850000, eliminated: false, captain: false, drafted: false },
            { id: 124497, username: "Aricin", country: "US", rank: 5, price: 800000, eliminated: false, captain: false, drafted: false },
            { id: 124498, username: "Bubbleman", country: "GB", rank: 6, price: 750000, eliminated: false, captain: false, drafted: false },
            { id: 124499, username: "Rafis", country: "PL", rank: 7, price: 700000, eliminated: false, captain: false, drafted: false },
            { id: 124500, username: "idke", country: "US", rank: 8, price: 650000, eliminated: false, captain: false, drafted: false },
            { id: 124501, username: "Azer", country: "US", rank: 9, price: 600000, eliminated: false, captain: false, drafted: false },
            { id: 124502, username: "Rohulk", country: "RO", rank: 10, price: 550000, eliminated: false, captain: false, drafted: false },
            { id: 124503, username: "Mathi", country: "FR", rank: 11, price: 500000, eliminated: false, captain: false, drafted: false },
            { id: 124504, username: "WubWoofWolf", country: "PL", rank: 12, price: 450000, eliminated: false, captain: false, drafted: false },
            { id: 124505, username: "Doomsday", country: "GB", rank: 13, price: 400000, eliminated: false, captain: false, drafted: false },
            { id: 124506, username: "Toy", country: "US", rank: 14, price: 380000, eliminated: false, captain: false, drafted: false },
            { id: 124507, username: "RyuK", country: "KR", rank: 15, price: 360000, eliminated: false, captain: false, drafted: false },
            { id: 124508, username: "Angelsim", country: "US", rank: 16, price: 340000, eliminated: false, captain: false, drafted: false },
            { id: 124509, username: "hvick225", country: "TW", rank: 17, price: 320000, eliminated: false, captain: false, drafted: false },
            { id: 124510, username: "Axarious", country: "US", rank: 18, price: 300000, eliminated: false, captain: false, drafted: false },
            { id: 124511, username: "Seouless", country: "KR", rank: 19, price: 280000, eliminated: false, captain: false, drafted: false },
            { id: 124512, username: "Emilia", country: "US", rank: 20, price: 260000, eliminated: false, captain: false, drafted: false },
            { id: 124513, username: "Filsdelama", country: "FR", rank: 21, price: 240000, eliminated: false, captain: false, drafted: false },
            { id: 124514, username: "Yaong", country: "KR", rank: 22, price: 220000, eliminated: false, captain: false, drafted: false },
            { id: 124515, username: "Karthy", country: "GB", rank: 23, price: 200000, eliminated: false, captain: false, drafted: false },
            { id: 124516, username: "Dustice", country: "DE", rank: 24, price: 180000, eliminated: false, captain: false, drafted: false },
            { id: 124517, username: "Rohi6", country: "JP", rank: 25, price: 160000, eliminated: false, captain: false, drafted: false },
            { id: 124518, username: "Pishifat", country: "US", rank: 26, price: 140000, eliminated: false, captain: false, drafted: false },
            { id: 124519, username: "Monstrata", country: "US", rank: 27, price: 120000, eliminated: false, captain: false, drafted: false },
            { id: 124520, username: "Naxess", country: "SE", rank: 28, price: 100000, eliminated: false, captain: false, drafted: false },
            { id: 124521, username: "ktgster", country: "US", rank: 29, price: 90000, eliminated: false, captain: false, drafted: false },
            { id: 124522, username: "HappyStick", country: "US", rank: 30, price: 80000, eliminated: false, captain: false, drafted: false },
            { id: 124523, username: "Guy", country: "US", rank: 31, price: 70000, eliminated: false, captain: false, drafted: false },
            { id: 124524, username: "Plaudible", country: "US", rank: 32, price: 60000, eliminated: false, captain: false, drafted: false },
            { id: 124525, username: "Niko", country: "US", rank: 33, price: 50000, eliminated: false, captain: false, drafted: false },
            { id: 124526, username: "Bikko", country: "JP", rank: 34, price: 45000, eliminated: false, captain: false, drafted: false },
            { id: 124527, username: "My Angel Haruna", country: "JP", rank: 35, price: 40000, eliminated: false, captain: false, drafted: false },
            { id: 124528, username: "Varvalian", country: "JP", rank: 36, price: 35000, eliminated: false, captain: false, drafted: false },
            { id: 124529, username: "Nakano-", country: "JP", rank: 37, price: 30000, eliminated: false, captain: false, drafted: false },
            { id: 124530, username: "Azerite", country: "US", rank: 38, price: 25000, eliminated: false, captain: false, drafted: false },
            { id: 124531, username: "Elysion", country: "US", rank: 39, price: 20000, eliminated: false, captain: false, drafted: false },
            { id: 124532, username: "Mismagius", country: "US", rank: 40, price: 15000, eliminated: false, captain: false, drafted: false },
            { id: 124533, username: "Woey", country: "US", rank: 41, price: 12000, eliminated: false, captain: false, drafted: false },
            { id: 124534, username: "Monko2k", country: "US", rank: 42, price: 10000, eliminated: false, captain: false, drafted: false },
            { id: 124535, username: "Dustice", country: "DE", rank: 43, price: 8000, eliminated: false, captain: false, drafted: false },
            { id: 124536, username: "Karthy", country: "GB", rank: 44, price: 6000, eliminated: false, captain: false, drafted: false },
            { id: 124537, username: "Rohulk", country: "RO", rank: 45, price: 4000, eliminated: false, captain: false, drafted: false },
            { id: 124538, username: "Mathi", country: "FR", rank: 46, price: 3000, eliminated: false, captain: false, drafted: false },
            { id: 124539, username: "WubWoofWolf", country: "PL", rank: 47, price: 2000, eliminated: false, captain: false, drafted: false },
            { id: 124540, username: "txFPS", country: "CA", rank: 48, price: 1500, eliminated: false, captain: false, drafted: false },
            { id: 124541, username: "Toy", country: "US", rank: 49, price: 1000, eliminated: false, captain: false, drafted: false },
            { id: 124542, username: "RyuK", country: "KR", rank: 50, price: 500, eliminated: false, captain: false, drafted: false }
        ];
        setAllPlayers(testPlayers);
        
        // Update drafted status based on userPlayers
        const updateDraftedStatus = (allPlayers: PlayerProps[], userPlayers: PlayerProps[]) => {
            return allPlayers.map(player => ({
                ...player,
                drafted: userPlayers.some(userPlayer => userPlayer.id === player.id)
            }));
        };
        
        // For now, set empty userPlayers (no one drafted yet)
        setUserPlayers([]);
        setAllPlayers(updateDraftedStatus(testPlayers, []));
        setLoading(false);
    }, []);

    // Real API calls - uncomment when ready to use
    // useEffect(() => {
    //     const fetchData = async () => {
    //         try {
    //             setLoading(true);
    //             const [allPlayersData, userPlayersData] = await Promise.all([
    //                 getAllPlayers(),
    //                 getUserPlayers()
    //             ]);
    //             
    //             // Update drafted status based on userPlayers
    //             const updateDraftedStatus = (allPlayers: PlayerProps[], userPlayers: PlayerProps[]) => {
    //                 return allPlayers.map(player => ({
    //                     ...player,
    //                     drafted: userPlayers.some(userPlayer => userPlayer.id === player.id)
    //                 }));
    //             };
    //             
    //             setUserPlayers(userPlayersData);
    //             setAllPlayers(updateDraftedStatus(allPlayersData, userPlayersData));
    //             setError(null);
    //         } catch (err) {
    //             console.error("Failed to fetch data:", err);
    //             setError("Failed to load data. Please try again later.");
    //         } finally {
    //             setLoading(false);
    //         }
    //     };
    //     
    //     fetchData();
    // }, []);

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

    const toggleSelect = (playerId: number) => {
        
        let player = players.find(p => p.id === playerId);

        if (!player) {
            return;
        }

        // If player is already drafted, undraft them
        if (player.drafted === true) {
            setAllPlayers(prevPlayers => 
                prevPlayers.map(player => 
                    player.id === playerId 
                        ? { ...player, drafted: !player.drafted }
                        : player
                )
            );
            setBalance((currentBalance) => currentBalance + player.price);
            return;
        }

        // Check if we can draft this player
        let draftedPlayers = players.filter(p => p.drafted === true);

        // Check team size limit
        if (draftedPlayers.length >= 8) {
            showNotification("Team is full! Maximum 8 players allowed.", 'error');
            return;
        }

        // Check balance
        if (balance < player.price) {
            showNotification("Insufficient balance!", 'error');
            return;
        }

        // Check country limit (max 2 players per country)
        const countryCount = draftedPlayers.filter(p => p.country === player.country).length;
        if (countryCount >= 2) {
            showNotification(`You already have 2 players from ${player.country}! Maximum 2 players per country allowed.`, 'error');
            return;
        }

        // All checks passed, draft the player
        setBalance((currentBalance) => currentBalance - player.price);

        setAllPlayers(prevPlayers => 
            prevPlayers.map(player => 
                player.id === playerId 
                    ? { ...player, drafted: !player.drafted }
                    : player
            )
        );
    }

    const finalizeDraft = () => {
        let draftedPlayers = players.filter(p => p.drafted === true);
        let draftCount = draftedPlayers.length;
        if (draftCount === 8) {
            setDraft(false);
            setUserPlayers(draftedPlayers);
        } else {
            showNotification("You have not drafted a full team", 'error');
        }
    }

    if (drafting) {
        return (
            <div className="p-5 flex flex-col items-center relative">
                {/* Notification Toast */}
                {notification && (
                    <div className={`fixed top-4 right-4 z-50 px-6 py-4 rounded-lg shadow-lg transition-all duration-300 ${
                        notification.type === 'error' 
                            ? 'bg-red-500 text-white' 
                            : 'bg-green-500 text-white'
                    }`}>
                        <div className="flex items-center gap-2">
                            {notification.type === 'error' ? (
                                <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                                    <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clipRule="evenodd" />
                                </svg>
                            ) : (
                                <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                                    <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd" />
                                </svg>
                            )}
                            <span className="font-medium">{notification.message}</span>
                        </div>
                    </div>
                )}
                <div className="flex items-center gap-4 w-full max-w-4xl">
                    <div className="relative flex-1 max-w-md">
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
                    
                    <div className="bg-gray-800 border border-gray-600 rounded-lg px-4 py-3 min-w-48">
                        <div className="text-gray-400 text-sm">Balance</div>
                        <div className="text-white text-xl font-bold">${balance.toLocaleString()}</div>
                    </div>

                    <div className="bg-gray-800 border border-gray-600 rounded-lg px-4 py-3 min-w-48">
                        <div className="text-gray-400 text-sm">Draft Count</div>
                        <div className="text-white text-xl font-bold">{players.filter(p => p.drafted === true).length.toLocaleString()} / 8</div>
                    </div>
                </div>
                
                <button className="bg-purple-500 text-white px-4 py-2 my-5 rounded-md text-2xl min-w-1/4" onClick={finalizeDraft}>Done</button>
                
                <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4 w-full max-w-6xl mx-auto">
                {players.filter(p => p.username.toLowerCase().includes(queryPlayer) || p.country.toLowerCase().includes(queryPlayer)).map((player) => (
                    <PlayerList 
                        key={player.id}
                        id={player.id}
                        username={player.username}
                        country={player.country}
                        rank={player.rank}
                        price={player.price}
                        eliminated={player.eliminated}
                        drafted={player.drafted}
                        playerSelected={toggleSelect}
                    />
                ))}
                </div>
                    
            </div>
        )
    }

    console.log(players)

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
            <button className="bg-purple-500 text-white px-4 py-2 my-5 rounded-md text-2xl min-w-1/4" onClick={() => setDraft(true)}>Edit</button>
        </div>
    );
}

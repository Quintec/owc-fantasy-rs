import { useState, useEffect } from "react";
import Player from "../components/Player";
import type { PlayerProps } from "../types";
import PlaceholderPlayer from "../components/PlaceholderPlayer";
import { getRemainingPlayersWithPrices } from "../api/players";
import { postPlayers, getUserTeamByRound, getTeamPlayersWithCaptain } from "../api/users";
import { useAuth } from "../contexts/AuthContext";
import { useRound } from "../contexts/RoundContext";

export default function Team() {
    const [userPlayers, setUserPlayers] = useState<PlayerProps[]>([]);
    const [players, setAllPlayers] = useState<PlayerProps[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [drafting, setDraft] = useState(false);
    const [queryPlayer, setPlayerQuery] = useState("");
    const [balance, setBalance] = useState(100000000); // 100M default budget
    const [notification, setNotification] = useState<{message: string, type: 'error' | 'success'} | null>(null);
    
    // drafting window is provided by RoundContext (Mon 00:00 UTC -> Fri 00:00 UTC)
    // useRound provides `isDraftOpen` for UI enable/disable

    const showNotification = (message: string, type: 'error' | 'success') => {
        setNotification({ message, type });
        setTimeout(() => setNotification(null), 3000);
    };

    const { user, loading: authLoading } = useAuth();
    const { round, isDraftOpen } = useRound();

    useEffect(() => {
        if (authLoading) {
            // still determining session; keep page-level loading until we know
            return;
        }

        if (!user) {
            // not logged in — stop data loading and show login prompt
            setLoading(false);
            setUserPlayers([]);
            setAllPlayers([]);
            setError(null);
            return;
        }
        const fetchData = async () => {
            try {
                setLoading(true);
                const allPlayersData = await getRemainingPlayersWithPrices(round);

                // Fetch the user's team for this round
                let userPlayersData: PlayerProps[] = [];
                try {
                    const team = await getUserTeamByRound(user.id, round);
                    if (team && team.id) {
                        userPlayersData = await getTeamPlayersWithCaptain(team.id, team.captain_id);
                    }
                } catch (err) {
                    // 404 or other error means no team exists yet - that's okay
                    console.log("No team found for user, starting with empty team");
                    userPlayersData = [];
                }
                
                // Update drafted status based on userPlayers
                const updateDraftedStatus = (allPlayers: PlayerProps[], userPlayers: PlayerProps[]) => {
                    // Create a map of user players with their order and captain status
                    const userPlayerMap = new Map(userPlayers.map((p, idx) => [p.id, { ...p, order: idx }]));
                    
                    return allPlayers.map(player => {
                        const userPlayer = userPlayerMap.get(player.id);
                        return {
                            ...player,
                            drafted: !!userPlayer,
                            captain: userPlayer?.captain || false,
                            // Store the order for sorting later
                            draftOrder: userPlayer?.order ?? 999999
                        };
                    });
                };
                
                setUserPlayers(userPlayersData);
                setAllPlayers(updateDraftedStatus(allPlayersData, userPlayersData));
                setError(null);
            } catch (err) {
                console.error("Failed to fetch data:", err);
                setError("Failed to load data. Please try again later.");
            } finally {
                setLoading(false);
            }
        };
        
        fetchData();
    }, [user?.id, round, authLoading]);

    if (authLoading) {
        return (
            <div className="p-5 flex flex-col items-center">
                <h1 className="text-xl font-bold text-white mb-5 text-center">My Team</h1>
                <div className="text-white text-xl">Loading session...</div>
            </div>
        );
    }

    if (!user) {
        return (
            <div className="p-5 flex flex-col items-center">
                <h1 className="text-xl font-bold text-white mb-5 text-center">Please log in</h1>
                <p className="text-gray-400 mb-4">You must be logged in to view and edit your team.</p>
                <button
                    className="bg-purple-500 text-white px-4 py-2 rounded-md"
                    onClick={() => window.location.href = `${import.meta.env.VITE_API_URL || 'http://localhost:8080'}/api/auth/login`}
                >
                    Login with osu!
                </button>
            </div>
        );
    }

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
            setAllPlayers(prevPlayers => {
                // First, undraft the player
                const withUndrafted = prevPlayers.map(p => 
                    p.id === playerId 
                        ? { ...p, drafted: false, captain: false, draftOrder: undefined }
                        : p
                );
                
                // Get remaining drafted players and reorder them
                const stillDrafted = withUndrafted
                    .filter(p => p.drafted)
                    .sort((a, b) => (a.draftOrder ?? 999999) - (b.draftOrder ?? 999999));
                
                // Create a map of updated draftOrder values
                const draftOrderMap = new Map(stillDrafted.map((p, i) => [p.id, { draftOrder: i, captain: i === 0 }]));
                
                // Update only the draftOrder and captain fields
                return withUndrafted.map(p => {
                    const update = draftOrderMap.get(p.id);
                    if (update) {
                        return { ...p, ...update };
                    }
                    return p;
                });
            });
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

        // Assign draftOrder based on current number of drafted players
        const newDraftOrder = draftedPlayers.length;
        const isCaptain = newDraftOrder === 0; // First player is captain

        setAllPlayers(prevPlayers => 
            prevPlayers.map(player => 
                player.id === playerId 
                    ? { ...player, drafted: !player.drafted, draftOrder: newDraftOrder, captain: isCaptain }
                    : player
            )
        );
    }

    const finalizeDraft = async () => {
        let draftedPlayers = players.filter(p => p.drafted === true).sort((a, b) => (a.draftOrder ?? 999999) - (b.draftOrder ?? 999999));
        let draftCount = draftedPlayers.length;
        if (draftCount !== 8) {
            showNotification("You have not drafted a full team", 'error');
            return;
        }

        if (!user) {
            showNotification('You must be logged in to submit your team', 'error');
            return;
        }

        // Prepare player IDs in the order they appear (captain is first)
        const playerIds = draftedPlayers.map(p => p.id);
        const captainId = playerIds[0]; // First player is the captain

        try {
            setLoading(true);
            // Call backend to create team and add players with captain
            const createdTeam = await postPlayers(user.id, playerIds, round, captainId);
            setUserPlayers(createdTeam);
            setDraft(false);
            showNotification('Team submitted successfully', 'success');
        } catch (err) {
            console.error('Failed to submit team', err);
            showNotification('Failed to submit team. See console for details.', 'error');
        } finally {
            setLoading(false);
        }
    }

    const resetDraft = () => {
        setAllPlayers(players.map((p) => ({ ...p, drafted: false })));
        setBalance(100000000);
    }

    const startDrafting = () => {
        // Calculate balance: 100M - sum of already drafted players
        const draftedPlayers = players.filter(p => p.drafted === true);
        const totalDraftedValue = draftedPlayers.reduce((sum, player) => sum + player.price, 0);
        const remainingBalance = 100000000 - totalDraftedValue;
        
        setBalance(remainingBalance);
        setDraft(true);
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

                <div className="flex items-center gap-4 w-full max-w-4xl justify-center">
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
                    
                    <div className="bg-gray-800 border border-gray-600 rounded-lg px-4 py-3 min-w-32">
                        <div className="text-gray-400 text-sm">Balance</div>
                        <div className="text-white text-xl font-bold">${balance.toLocaleString()}</div>
                    </div>

                    <div className="bg-gray-800 border border-gray-600 rounded-lg px-4 py-3 min-w-32">
                        <div className="text-gray-400 text-sm">Draft Count</div>
                        <div className="text-white text-xl font-bold">{players.filter(p => p.drafted === true).length.toLocaleString()} / 8</div>
                    </div>
                
                

                </div>
                    <h1 className="text-2xl text-white font-bold mt-5">Current Selection</h1>
                    <h2 className="text-gray-400 text-sm">Drag to select captain</h2>
                    <div className="w-full">
                    <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4 w-full max-w-6xl mx-auto mt-5 auto-rows-fr">
                        {players.filter(p => p.drafted).sort((a, b) => (a.draftOrder ?? 999999) - (b.draftOrder ?? 999999)).map((player, idx) => (
                            <div
                                key={player.id}
                                draggable
                                onDragStart={(e) => {
                                    e.dataTransfer.setData('text/plain', String(player.id));
                                }}
                                onDragOver={(e) => e.preventDefault()}
                                onDrop={(e) => {
                                    e.preventDefault();
                                    const draggedId = Number(e.dataTransfer.getData('text/plain'));
                                    if (!draggedId || draggedId === player.id) return;
                                    const draftedOrdered = players.filter(p => p.drafted).sort((a, b) => (a.draftOrder ?? 999999) - (b.draftOrder ?? 999999));
                                    const fromIdx = draftedOrdered.findIndex(p => p.id === draggedId);
                                    const toIdx = draftedOrdered.findIndex(p => p.id === player.id);
                                    if (fromIdx === -1 || toIdx === -1) return;
                                    const reordered = [...draftedOrdered];
                                    const [moved] = reordered.splice(fromIdx, 1);
                                    reordered.splice(toIdx, 0, moved);
                                    
                                    // Update only the draftOrder and captain fields, don't reorder the main array
                                    const draftOrderMap = new Map(reordered.map((p, i) => [p.id, { draftOrder: i, captain: i === 0 }]));
                                    
                                    setAllPlayers(prevPlayers => 
                                        prevPlayers.map(p => {
                                            const update = draftOrderMap.get(p.id);
                                            if (update) {
                                                return { ...p, ...update };
                                            }
                                            return p;
                                        })
                                    );
                                }}
                                className={`relative`}
                            >
                                {idx === 0 && (
                                    <div className="absolute -top-2 -right-2 bg-yellow-400 text-black text-xs font-bold px-2 py-1 rounded shadow">CAPTAIN</div>
                                )}
                                <div className={`${idx === 0 ? 'ring-2 ring-yellow-400' : ''} h-full`}>
                                    <Player 
                                        id={player.id}
                                        username={player.username}
                                        country={player.country}
                                        rank={player.rank}
                                        price={player.price}
                                        eliminated={player.eliminated}
                                        drafted={player.drafted}
                                        captain={player.captain}
                                        playerSelected={toggleSelect}
                                    />
                                </div>
                            </div>
                        ))}
                        {Array.from({length: Math.max(0, 8 - players.filter(p => p.drafted).length)}).map((_, i) => (
                            <div
                                key={`ph-${i}`}
                                onDragOver={(e) => e.preventDefault()}
                                onDrop={(e) => {
                                    e.preventDefault();
                                    const draggedId = Number(e.dataTransfer.getData('text/plain'));
                                    const draftedOrdered = players.filter(p => p.drafted).sort((a, b) => (a.draftOrder ?? 999999) - (b.draftOrder ?? 999999));
                                    const fromIdx = draftedOrdered.findIndex(p => p.id === draggedId);
                                    if (fromIdx === -1) return;
                                    const reordered = [...draftedOrdered];
                                    const [moved] = reordered.splice(fromIdx, 1);
                                    reordered.push(moved);
                                    
                                    // Update only the draftOrder and captain fields, don't reorder the main array
                                    const draftOrderMap = new Map(reordered.map((p, i) => [p.id, { draftOrder: i, captain: i === 0 }]));
                                    
                                    setAllPlayers(prevPlayers => 
                                        prevPlayers.map(p => {
                                            const update = draftOrderMap.get(p.id);
                                            if (update) {
                                                return { ...p, ...update };
                                            }
                                            return p;
                                        })
                                    );
                                }}
                                className="h-full"
                            >
                                <PlaceholderPlayer />
                            </div>
                        ))}
                    </div>
                    
                    </div>
                    <div className="flex justify-center w-full">
                        <button className="bg-green-500 text-white px-4 py-2 my-5 rounded-md text-2xl min-w-1/6" onClick={finalizeDraft}>Submit</button>
                        <button className="bg-red-700 text-white px-4 py-2 my-5 rounded-md text-2xl min-w-1/6 ml-5" onClick={resetDraft}>Reset</button>
                    </div>
                <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4 w-full max-w-6xl mx-auto">
                {players
                    .filter(p => p.username.toLowerCase().includes(queryPlayer) || p.country.toLowerCase().includes(queryPlayer))
                    .sort((a, b) => {
                        // Sort by country first
                        if (a.country !== b.country) {
                            return a.country.localeCompare(b.country);
                        }
                        // Then by rank for same country
                        return a.rank - b.rank;
                    })
                    .map((player) => (
                    <Player 
                        key={player.id}
                        id={player.id}
                        username={player.username}
                        country={player.country}
                        rank={player.rank}
                        price={player.price}
                        eliminated={player.eliminated}
                        drafted={player.drafted}
                        captain={player.captain}
                        playerSelected={toggleSelect}
                    />
                ))}
                </div>
                    
            </div>
        )
    }

    return (
        <div className="p-5 flex flex-col items-center">
            <h1 className="text-xl font-bold text-white mb-5 text-center">My Team</h1>
            
            <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4 w-full max-w-6xl mx-auto">
                {userPlayers.map((player) => (
                    <div key={player.id} className="relative">
                        {player.captain && (
                            <div className="absolute -top-2 -right-2 bg-yellow-400 text-black text-xs font-bold px-2 py-1 rounded shadow z-10">CAPTAIN</div>
                        )}
                        <Player 
                            id={player.id}
                            username={player.username}
                            country={player.country}
                            rank={player.rank}
                            price={player.price}
                            eliminated={player.eliminated}
                            captain={player.captain || false}
                        />
                    </div>
                ))}
                {Array.from({length: Math.max(0, 8 - userPlayers.length)}).map((_, i) => (
                    <PlaceholderPlayer key={`ph-${i}`} />
                ))}
            </div>
            {isDraftOpen ? (
                <div className="my-5 text-center w-full">
                    <button 
                        className="bg-purple-500 hover:bg-purple-600 text-white px-4 py-2 my-5 rounded-md text-2xl min-w-1/4 transition-colors" 
                        onClick={startDrafting}
                    >
                        Edit
                    </button>
                    <p className="text-gray-400 text-sm">
                            Drafting is currently open (Mon 00:00 UTC → Fri 00:00 UTC)
                    </p>
                </div>
            ) : (
                <div className="my-5 text-center w-full">
                    <button 
                        className="bg-gray-600 text-gray-400 px-4 py-2 rounded-md text-2xl min-w-1/4 cursor-not-allowed" 
                        disabled
                    >
                        Edit Locked
                    </button>
                    <p className="text-gray-400 text-sm mt-5">
                        Drafting is locked. Drafting windows open Mon 00:00 UTC and close Fri 00:00 UTC.
                    </p>
                </div>
            )}
        </div>
    );
}

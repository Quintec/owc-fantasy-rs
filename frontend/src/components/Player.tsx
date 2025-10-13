import type { PlayerProps } from "../types";

export default function Player(player: PlayerProps) {

    if (!player.username) {
        return (
            <div className="flex flex-col w-full min-w-0 h-full bg-pink-500 rounded-md p-3"> 
                <h1 className="text-white text-2xl text-bold">N/A</h1>
            </div>
        )
    }

    return (
        <div className={`flex flex-col w-full min-w-0 h-full rounded-md p-3 ${player.captain ? 'bg-yellow-500 ring-2 ring-yellow-300' : 'bg-pink-500'}`}> 
            <div className="flex flex-row mb-5 items-center">
                <img alt="avatar" src={`https://a.ppy.sh/${player.id}`} className="w-15 h-15 rounded-full mr-3"></img>
                <div className="flex flex-col">
                    <h1 className="text-white font-bold text-md">{player.username}</h1>
                    <h2 className="text-white text-xl">Rank: {player.rank}</h2>
                    <img alt="country" src={`https://osuflags.omkserver.nl/${player.country}.png`} className="w-10 h-10"></img>
                </div>
            </div>
            <h2 className="text-white text-xl"><span className="font-bold">Price:</span> ${player.price}</h2>
            {player.captain && (
                <div className="text-yellow-900 text-sm font-bold mt-2">CAPTAIN</div>
            )}

        </div>
    )
}
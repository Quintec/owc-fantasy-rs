import type { PlayerProps } from "../types";

export default function PlayerList(player: PlayerProps) {
    return (
        <div className="flex bg-pink-500 p-4 h-30 w-5/6"> 
            <img alt="avatar" src={`https://a.ppy.sh/${player.id}`} className="h-25 w-25 rounded-full mr-3"></img>
            <h1 className="text-white md:text-xl font-bold">{player.username}</h1>
            <h2 className="text-white text-xl">Rank: {player.rank}</h2>
            <img alt="country" src={`https://osuflags.omkserver.nl/${player.country}.png`} className="w-1/4"></img>
            <h2 className="text-white text-xl"><span className="font-bold">Price:</span> ${player.price}</h2>
        </div>
    )
}
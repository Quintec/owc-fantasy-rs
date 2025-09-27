import type { PlayerProps } from "../types";

export default function Player(player: PlayerProps) {

    if (!player.username) {
        return (
            <div className="bg-pink-500 rounded-md p-4 flex justify-center items-center"> 
                <h1 className="text-white text-2xl text-bold">N/A</h1>
            </div>
        )
    }

    return (
        <div className="flex flex-col bg-pink-500 rounded-md p-4"> 
            <div className="flex flex-row bg-white-850 mb-5">
                <img alt="avatar" src={`https://a.ppy.sh/${player.id}`} className="w-1/4 rounded-full mr-3"></img>
                <div className="flex flex-col">
                    <h1 className="text-white md:text-xl font-bold">{player.username}</h1>
                    <h2 className="text-white text-xl">Rank: {player.rank}</h2>
                    <img alt="country" src={`https://osuflags.omkserver.nl/${player.country}.png`} className="w-1/4"></img>
                </div>
            </div>
            <h2 className="text-white text-xl"><span className="font-bold">Price:</span> ${player.price}</h2>

        </div>
    )
}
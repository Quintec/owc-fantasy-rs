import type { PlayerProps } from "../types";

type PlayerListProps = PlayerProps & {
    playerSelected: (id: number) => void;
};

export default function PlayerList({ id, username, country, rank, price, drafted, captain, playerSelected }: PlayerListProps) {

    if (!username) {
        return (
            <div className="bg-pink-500 rounded-md p-4 flex justify-center items-center"> 
                <h1 className="text-white text-2xl font-bold">N/A</h1>
            </div>
        )
    }

    return (
        <div 
            className={`flex flex-col h-full w-full min-w-0 rounded-md p-3 cursor-pointer transition-all duration-200 ${
                drafted 
                    ? `${captain ? 'bg-yellow-500 ring-2 ring-yellow-300' : 'bg-green-500 ring-2 ring-green-300'}` 
                    : 'bg-pink-500 hover:bg-pink-400'
            }`} 
            onClick={() => playerSelected(id)}
        > 
           
            
            <div className="flex flex-row mb-5 items-center">
                <img alt="avatar" src={`https://a.ppy.sh/${id}`} className="w-15 h-15 rounded-full mr-3"></img>
                <div className="flex flex-col">
                    <h1 className="text-white font-bold text-md">{username}</h1>
                    <h2 className="text-white text-xl">Rank: {rank}</h2>
                    <img alt="country" src={`https://osuflags.omkserver.nl/${country}.png`} className="w-10 h-10"></img>
                </div>
            </div>
            <h2 className="text-white text-xl"><span className="font-bold">Price:</span> ${price}</h2>
            
            {/* Drafted text */}
            {drafted && (
                <div className={`text-sm font-bold mt-2 ${captain ? 'text-yellow-900' : 'text-green-200'}`}>{captain ? 'CAPTAIN' : 'DRAFTED'}</div>
            )}
        </div>
    )
}
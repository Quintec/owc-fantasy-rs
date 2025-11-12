import type { PlayerProps } from "../types";

type PlayerComponentProps = PlayerProps & {
    playerSelected?: (id: number) => void;
};

export default function Player({ id, username, country, rank, price, drafted, playerSelected }: PlayerComponentProps) {

    if (!username) {
        return (
            <div className="flex flex-col w-full min-w-0 h-full bg-pink-500 rounded-md p-3"> 
                <h1 className="text-white text-2xl text-bold">N/A</h1>
            </div>
        )
    }

    const isClickable = !!playerSelected;
    
    return (
        <div 
            className={`flex flex-col h-full w-full min-w-0 rounded-md p-3 ${
                isClickable ? 'cursor-pointer transition-all duration-200' : ''
            } ${
                drafted ? 'bg-green-600 ring-2 ring-green-300':
                isClickable ? 'bg-pink-600 hover:bg-pink-400 ring-2 ring-pink-300' : 'bg-pink-700 ring-2 ring-pink-300'
            }`}
            onClick={() => playerSelected?.(id)}
        > 
            <div className="flex flex-row mb-2 items-center min-w-0">
                <img alt="avatar" src={`https://a.ppy.sh/${id}`} className="w-12 h-12 md:w-15 md:h-15 rounded-full mr-3 flex-shrink-0"></img>
                <div className="flex flex-col min-w-0 flex-1">
                    <h1 className="text-white font-bold md:text-xl truncate">{username}</h1>
                    <h2 className="text-white text-xl">Rank: {rank}</h2>
                    <img alt="country" src={`https://osuflags.omkserver.nl/${country}.png`} className="w-10 h-10"></img>
                </div>
            </div>
            <h2 className="text-white text-xl"><span className="font-bold">Price:</span> ${price.toLocaleString()}</h2>
        </div>
    )
}
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
                drafted 
                    ? 'bg-green-500 ring-2 ring-green-300'
                    : isClickable 
                    ? 'bg-pink-500 hover:bg-pink-400' 
                    : 'bg-pink-500'
            }`}
            onClick={() => playerSelected?.(id)}
        > 
            <div className="flex flex-row mb-2 items-center">
                <img alt="avatar" src={`https://a.ppy.sh/${id}`} className="w-15 h-15 rounded-full mr-3"></img>
                <div className="flex flex-col">
                    <h1 className="text-white font-bold text-md">{username}</h1>
                    <h2 className="text-white text-xl">Rank: {rank}</h2>
                    <img alt="country" src={`https://osuflags.omkserver.nl/${country}.png`} className="w-10 h-10"></img>
                </div>
            </div>
            <h2 className="text-white text-xl"><span className="font-bold">Price:</span> ${price}</h2>
        </div>
    )
}
type PlayerProps = {
    id: number;
    username: string;
    country: string;
    rank: number;
    price: number;
    eliminated: boolean;
}

export default function Player(player: PlayerProps) {

    if (!player.username) {
        return (
            <div className="flex flex-col bg-pink-500 rounded-md p-5 max-w-1/4 m-5"> 
                <h1>test</h1>
            </div>
        )
    }

    return (
        <div className="flex flex-col bg-pink-500 rounded-md p-5"> 
            <div className="flex flex-row bg-white-850 justify-between items-center">
                <img alt="avatar" src={`https://a.ppy.sh/${player.id}`} className="w-1/4 rounded-full"></img>
                <h1 className="text-white text-2xl font-bold">{player.username}</h1>
                <img alt="country" src={`https://a.ppy.sh/${player.id}`} className="max-w-1/8"></img>
            </div>
            <h1 className="text-white text-xl font-bold">Price: ${player.price}</h1>
            <h1 className="text-white text-xl font-bold">Rank: {player.rank}</h1>
        </div>
    )
}
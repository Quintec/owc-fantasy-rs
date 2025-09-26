import Player from "../components/Player";

export default function Team() {
    return (
        <div className="p-5">
            <h1 className="text-xl font-bold text-white mb-5 text-center">My Team</h1>
            <div className="grid grid-cols-2 grid-rows-4 md:grid-cols-4 md:grid-rows-2 gap-4">
                <Player 
                    id={1}
                    username="Player1"
                    country="US"
                    rank={1}
                    price={1000000}
                    eliminated={false}
                />
                <Player 
                    id={2}
                    username="Player2"
                    country="JP"
                    rank={2}
                    price={950000}
                    eliminated={false}
                />
                <Player 
                    id={3}
                    username="123456789012345"
                    country="KR"
                    rank={3}
                    price={900000}
                    eliminated={false}
                />
                <Player 
                    id={3}
                    username="123456789012345"
                    country="KR"
                    rank={3}
                    price={900000}
                    eliminated={false}
                />
                <Player 
                    id={3}
                    username="123456789012345"
                    country="KR"
                    rank={3}
                    price={900000}
                    eliminated={false}
                />
                <Player 
                    id={3}
                    username="123456789012345"
                    country="KR"
                    rank={3}
                    price={900000}
                    eliminated={false}
                />
                <Player 
                    id={3}
                    username="123456789012345"
                    country="KR"
                    rank={3}
                    price={900000}
                    eliminated={false}
                />
            </div>
        </div>
    );
}

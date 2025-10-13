import type { User } from "../types";

type UserListProps = User & {
    rank: number;
}

export default function UserList(user: UserListProps) {

    

    return (
        <div className="bg-pink-500 rounded-md p-5 flex items-center justify-between">
            <div className="flex items-center">
                <span className="text-white text-2xl mr-5">#{user.rank + 1}</span>
                <img src={`https://a.ppy.sh/${user.id}`} className="rounded-full h-20 w-20 mr-5"></img>
                <span className="text-white text-bold text-xl">{user.username}</span>
            </div>
            <span className="text-white text-bold text-xl">Score: {user.score}</span>
        </div>
    )
}
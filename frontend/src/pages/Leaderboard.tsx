import UserList from "../components/UserList";
import type { User } from "../types";

export default function Leaderboard() {

    const testUsers: User[] = [
        {id: 12835025, username: "txFPS", score: 5},
        {id: 12058601, username: "Squink", score: 67},
        {id: 15458667, username: "QuintecX", score: 21},
        {id: 14681304, username: "MokaMilku", score: 727},
        {id: 16657449, username: "AndromedaX1", score: 1738},
    ]

    testUsers.sort((a, b) => b.score - a.score);

    return (
        <div className="flex flex-col gap-4 m-10">
        {testUsers.map((u, index) => (
            <UserList id={u.id} username={u.username} score={u.score} rank={index}/>
        ))}
        </div>
    );
    
}
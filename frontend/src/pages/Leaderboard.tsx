import { useEffect, useState } from "react";
import UserList from "../components/UserList";
import type { User } from "../types";
import { getLeaderboard } from "../api/users";

export default function Leaderboard() {

    const [users, setUsers] = useState<User[]>([]);

    useEffect(() => {
        const fetchData = async () => {
            try {
                const data = await getLeaderboard();
                setUsers(data);
            } catch (error) {
                console.error("Error fetching leaderboard:", error);
            }
        }
        fetchData();
    }, []);

    return (
        <div className="flex flex-col gap-4 m-10 justify-center">
        {users.map((u, index) => (
            <UserList id={u.id} username={u.username} score={u.score} rank={index} key={index}/>
        ))}
        </div>
    );
    
}
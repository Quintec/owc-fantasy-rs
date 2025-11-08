import { useEffect, useState } from "react";
import UserList from "../components/UserList";
import type { User } from "../types";
import { getUsers } from "../api/users";

export default function Leaderboard() {

    const [users, setUsers] = useState<User[]>([]);

    useEffect(() => {
        const fetchData = async () => {
            try {
                const data = await getUsers();
                setUsers(data);
                users.sort((a, b) => b.score - a.score);
            } catch (error) {
                console.error("Error fetching users:", error);
            }
        }
        fetchData();
    }, []);

    return (
        <div className="flex flex-col gap-4 m-10">
        {users.map((u, index) => (
            <UserList id={u.id} username={u.username} score={u.score} rank={index} key={index}/>
        ))}
        </div>
    );
    
}
export type PlayerProps = {
    id: number;
    username: string;
    country: string;
    rank: number;
    price: number;
    eliminated: boolean;
    drafted?: boolean;
    captain?: boolean;
    playerSelected?: Function;
};

export type User = {
    id: number;
    username: string;
    score: number;
    team?: PlayerProps[]
};

// export type Team = {
//     id: number;
//     user_id: number;
//     round: string;
//     captain_id?: number | null;
// };

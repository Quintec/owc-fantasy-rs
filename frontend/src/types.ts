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

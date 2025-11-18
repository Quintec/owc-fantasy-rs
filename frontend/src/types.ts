export type PlayerProps = {
    id: number;
    username: string;
    avatar_url?: string;
    country: string;
    rank: number;
    price: number;
    eliminated: boolean;
    score?: number;
    drafted?: boolean;
    captain?: boolean;
    draftOrder?: number;
    playerSelected?: Function;
};

export type User = {
    id: number;
    username: string;
    score: number;
    team?: PlayerProps[]
};

export type Team = {
    id: number;
    user_id: number;
    round: string;
    captain_id?: number | null;
};

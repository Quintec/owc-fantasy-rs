export type PlayerProps = {
    id: number;
    username: string;
    country: string;
    rank: number;
    price: number;
    eliminated: boolean;
    captain?: boolean;
};

export type User = {
    id: number;
    username: string;
    avatar_url?: string | null;
};

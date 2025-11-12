use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub avatar_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserWithScore {
    pub id: i32,
    pub username: String,
    pub avatar_url: String,
    pub score: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: i32,
    pub username: String,
    pub avatar_url: String,
    pub country: String,
    pub rank: i32,
    pub eliminated: i8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerWithScore {
    pub id: i32,
    pub username: String,
    pub avatar_url: String,
    pub country: String,
    pub rank: i32,
    pub eliminated: i8,
    pub score: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerWithPrice {
    pub id: i32,
    pub username: String,
    pub avatar_url: String,
    pub country: String,
    pub rank: i32,
    pub eliminated: i8,
    pub price: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub id: i32,
    pub user_id: i32,
    pub round: String,
    pub captain_id: Option<i32>,
}

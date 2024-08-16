use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub avatar_url: String,
}
#[derive(Debug, Serialize)]
pub struct Player {
    pub id: i32,
    pub username: String,
    pub avatar_url: String,
    pub country: String,
    pub rank: i32,
}
#[derive(Debug, Serialize)]
pub struct Team {
    pub id: i32,
    pub user_id: i32,
    pub round: String,
}

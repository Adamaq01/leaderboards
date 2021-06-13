use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct User {
    id: i32,
    display_name: String,
    username: String,
    salt: String,
    password: String,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct Category {
    id: i32,
    user_id: i32,
    display_name: String,
    description: String,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct Leaderboard {
    id: i32,
    user_id: i32,
    display_name: String,
    description: String,
    category_id: i32,
    ordering: i32,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct Score {
    id: i32,
    score: i32,
    user_id: i32,
    leaderboard_id: i32,
}

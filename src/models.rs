use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct CreateUserBody {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, FromRow)]
pub struct AuthUser {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, FromRow)]
pub struct UserNoPassword {
    pub id: i32,
    pub username: String,
}

#[derive(Serialize, FromRow)]
pub struct Deck {
    pub id: i32,
    pub deckname: String,
    pub created_by: i32,
}

#[derive(Deserialize)]
pub struct CreateDeck {
    pub deckname: String,
}

#[derive(Serialize, FromRow)]
pub struct Card {
    pub id: i32,
    pub front: String,
    pub back: String,
    pub deck_id: i32,
}

#[derive(Deserialize)]
pub struct CreateDeckCard {
    pub front: String,
    pub back: String,
}

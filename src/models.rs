use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Deck {
    pub id: i32,
    pub deckname: String,
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

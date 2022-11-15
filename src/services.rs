use crate::config::AppState;
use crate::models::{Card, CreateDeck, CreateDeckCard, Deck};
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};

#[get("/decklist")]
pub async fn fetch_decks(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, Deck>("SELECT id, deckname FROM deck")
        .fetch_all(&state.db)
        .await
    {
        Ok(decklist) => HttpResponse::Ok().json(decklist),
        Err(_) => HttpResponse::NotFound().json("No decks found"),
    }
}

#[get("/decklist/{id}/cards")]
pub async fn fetch_deck_cards(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();

    match sqlx::query_as::<_, Card>(
        r#"SELECT id, front, back, deck_id FROM card WHERE deck_id = $1"#,
    )
    .bind(id)
    .fetch_all(&state.db)
    .await
    {
        Ok(cards) => HttpResponse::Ok().json(cards),
        Err(_) => HttpResponse::NotFound().json("No cards found"),
    }
}

#[post("/decklist")]
pub async fn create_deck(state: Data<AppState>, body: Json<CreateDeck>) -> impl Responder {
    match sqlx::query_as::<_, Deck>(
        "INSERT INTO deck (deckname) VALUES ($1) RETURNING id, deckname",
    )
    .bind(body.deckname.to_string())
    .fetch_one(&state.db)
    .await
    {
        Ok(deck) => HttpResponse::Ok().json(deck),
        Err(_) => HttpResponse::InternalServerError().json("Error in creating a deck"),
    }
}

#[post("/decklist/{id}/cards")]
pub async fn create_deck_card(
    state: Data<AppState>,
    path: Path<i32>,
    body: Json<CreateDeckCard>,
) -> impl Responder {
    let id: i32 = path.into_inner();

    match sqlx::query_as::<_, Card>(
      "INSERT INTO card (front, back, deck_id) VALUES ($1, $2, $3) RETURNING id, front, back, deck_id"
    )
        .bind(body.front.to_string())
        .bind(body.back.to_string())
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
      Ok(card) => HttpResponse::Ok().json(card),
      Err(_) => HttpResponse::InternalServerError().json("Error in creating a deck card")
    }
}

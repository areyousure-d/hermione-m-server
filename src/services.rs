use crate::config::AppState;
use crate::models::{
    AuthUser, Card, CreateDeck, CreateDeckCard, CreateUserBody, Deck, UserNoPassword,
};
use crate::TokenClaims;
use actix_web::{
    get, post,
    web::{Data, Json, Path, ReqData},
    HttpResponse, Responder,
};
use actix_web_httpauth::extractors::basic::BasicAuth;
use argonautica::{Hasher, Verifier};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use sqlx::{self};

#[get("/decklist")]
pub async fn fetch_decks(
    state: Data<AppState>,
    req_user: Option<ReqData<TokenClaims>>,
) -> impl Responder {
    match req_user {
        Some(user) => {
            match sqlx::query_as::<_, Deck>(
                "SELECT id, deckname, created_by FROM deck WHERE created_by = $1",
            )
            .bind(user.id)
            .fetch_all(&state.db)
            .await
            {
                Ok(decklist) => HttpResponse::Ok().json(decklist),
                Err(_) => HttpResponse::NotFound().json("No decks found"),
            }
        }
        _ => HttpResponse::Unauthorized().json("Unable to verify identity"),
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
pub async fn create_deck(
    state: Data<AppState>,
    req_user: Option<ReqData<TokenClaims>>,
    body: Json<CreateDeck>,
) -> impl Responder {
    match req_user {
        Some(user) => {
            match sqlx::query_as::<_, Deck>(
                "INSERT INTO deck (deckname, created_by) VALUES ($1, $2) RETURNING id, deckname, created_by",
            )
            .bind(body.deckname.to_string())
            .bind(user.id)
            .fetch_one(&state.db)
            .await
            {
                Ok(deck) => HttpResponse::Ok().json(deck),
                //Err(_) => HttpResponse::InternalServerError().json("Error creating a deck"),
                Err(error) => HttpResponse::InternalServerError().json(error.to_string()),
            }
        }
        _ => HttpResponse::Unauthorized().json("Unable to verify identity"),
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

#[post("/user")]
async fn create_user(state: Data<AppState>, body: Json<CreateUserBody>) -> impl Responder {
    let user: CreateUserBody = body.into_inner();

    let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set");
    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(user.password)
        .with_secret_key(hash_secret)
        .hash()
        .unwrap();

    match sqlx::query_as::<_, UserNoPassword>(
        "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id, username",
    )
    .bind(user.username)
    .bind(hash)
    .fetch_one(&state.db)
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
    }
}

#[post("/auth")]
async fn basic_auth(state: Data<AppState>, credentials: BasicAuth) -> impl Responder {
    let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
        std::env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set")
            .as_bytes(),
    )
    .unwrap();

    let username = credentials.user_id();
    let password = credentials.password();

    match password {
        None => HttpResponse::Unauthorized().json("Must provide username and password"),
        Some(pass) => {
            match sqlx::query_as::<_, AuthUser>(
                "SELECT id, username, password FROM users WHERE username = $1",
            )
            .bind(username.to_string())
            .fetch_one(&state.db)
            .await
            {
                Ok(user) => {
                    let hash_secret =
                        std::env::var("HASH_SECRET").expect("HASH_SECRET must be set");
                    let mut verifier = Verifier::default();
                    let is_valid = verifier
                        .with_hash(user.password)
                        .with_password(pass)
                        .with_secret_key(hash_secret)
                        .verify()
                        .unwrap();

                    if is_valid {
                        let claims = TokenClaims { id: user.id };
                        let token_str = claims.sign_with_key(&jwt_secret).unwrap();
                        HttpResponse::Ok().json(token_str)
                    } else {
                        HttpResponse::Unauthorized().json("Incorrect username or password")
                    }
                }
                Err(error) => {
                    HttpResponse::InternalServerError().json(format!("Auth error: {:?}", error))
                }
            }
        }
    }
}

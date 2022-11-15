mod config;
mod models;
mod services;

use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use services::{create_deck, create_deck_card, fetch_deck_cards, fetch_decks};
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = crate::config::AppConfig::from_env().unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&format!(
            "postgres://{}:{}@{}:{}/{}",
            config.pg.user, config.pg.password, config.pg.host, config.pg.port, config.pg.dbname
        ))
        .await
        .expect("Error building a connection pool");

    println!(
        "Starting server at http://{}:{}/",
        config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(config::AppState { db: pool.clone() }))
            .service(fetch_decks)
            .service(fetch_deck_cards)
            .service(create_deck_card)
            .service(create_deck)
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}

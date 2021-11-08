use std::env;

use actix_cors::Cors;
use actix_web::http;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
mod discord;
mod oauth;
mod polls;
use discord::routes::get_mutual_guilds;
use oauth::routes::authenticate;
use oauth::routes::oauth_url;
use polls::routes::save_poll;

use crate::polls::routes::get_poll_channels;

pub fn panic_on_missing_env() {
    env::var("DISCORD_CLIENT_ID").expect("Expected DISCORD_CLIENT_ID in the environment");
    env::var("DISCORD_CLIENT_SECRET").expect("Expected DISCORD_CLIENT_SECRET in the environment");
    env::var("DISCORD_CLIENT_TOKEN").expect("Expected DISCORD_CLIENT_TOKEN in the environment");
    env::var("CLIENT_URI").expect("Expected CLIENT_URI in the environment");
    env::var("DB_HOST").expect("Expected DB_HOST in the environment");
    env::var("DB_NAME").expect("Expected DB_NAME in the environment");
    env::var("DB_PASSWORD").expect("Expected DB_PASSWORD in the environment");
    env::var("DB_USER").expect("Expected DB_USER in the environment");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to load .env file");
    panic_on_missing_env();

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_origin(
                        &env::var("CLIENT_URI").expect("Expected CLIENT_URI in the environment"),
                    )
                    .allowed_header(http::header::CONTENT_TYPE)
                    .allowed_header(http::header::AUTHORIZATION),
            )
            .route("/api/polls/save", web::post().to(save_poll))
            .route("/api/oauth/url", web::get().to(oauth_url))
            .route(
                "/api/oauth/authenticate/{code}",
                web::get().to(authenticate),
            )
            .route(
                "/api/discord/get_mutual_guilds",
                web::get().to(get_mutual_guilds),
            )
            .route(
                "/api/polls/{guild_id}/allowed_channels",
                web::get().to(get_poll_channels),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

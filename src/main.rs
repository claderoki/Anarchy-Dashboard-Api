use std::env;

use actix_cors::Cors;
use actix_web::http;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
mod discord;
mod helpers;
mod oauth;
mod polls;
use discord::routes::get_mutual_guilds;
use oauth::routes::authenticate;
use oauth::routes::oauth_url;
use polls::routes::get_available_poll_changes;
use polls::routes::get_poll_channels;
use polls::routes::save_poll;

use crate::discord::routes::get_all_members;
use crate::discord::routes::get_all_roles;
use crate::discord::routes::get_all_text_channels;
use crate::helpers::caching::base::get_connection_redis;

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
    get_connection_redis().expect("Failed to load redis");
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
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/discord")
                            .service(get_mutual_guilds)
                            .service(get_all_members)
                            .service(get_all_text_channels)
                            .service(get_all_roles),
                    )
                    .service(
                        web::scope("/oauth")
                            .service(oauth_url)
                            .service(authenticate),
                    )
                    .service(
                        web::scope("/polls")
                            .service(save_poll)
                            .service(get_poll_channels)
                            .service(get_available_poll_changes),
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

use actix_cors::Cors;
use actix_web::web;
use actix_web::App;
use actix_web::http;
use actix_web::HttpServer;
mod polls;
mod oauth;
use polls::routes::save_poll;
use oauth::routes::oauth_url;
use oauth::routes::authenticate;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default()
                .allowed_methods(vec!["GET", "POST"])
                .allowed_origin("http://127.0.0.1:3000")
                .allowed_header(http::header::CONTENT_TYPE)
            )
            .route("/api/polls/save", web::post().to(save_poll))
            .route("/api/oauth/url", web::get().to(oauth_url))
            .route("/api/oauth/authenticate/{code}", web::get().to(authenticate))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

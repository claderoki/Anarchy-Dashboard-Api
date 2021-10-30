use actix_cors::Cors;
use actix_web::{web, App, http, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct PollOption {
    positive: bool,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Poll {
    id: i32,
    question: String,
    channel_id: u64,
    result_channel_id: Option<u64>,
    pin: bool,
    mention_role: bool,
    delete_after_results: bool,
    custom: bool,
    role_id_needed: Option<u64>,
    vote_percentage_needed_to_pass: i16,
    max_votes_per_user: i16,
    options: Vec<PollOption>,
}

async fn save_poll(poll: web::Json<Poll>) -> impl Responder {
    println!("{:?}", poll);
    format!("OK")
}

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
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

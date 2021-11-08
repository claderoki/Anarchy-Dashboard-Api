use super::models::Poll;
use actix_web::web;
use actix_web::Responder;

pub async fn save_poll(poll: web::Json<Poll>) -> impl Responder {
    println!("{:?}", poll);
    format!("OK")
}

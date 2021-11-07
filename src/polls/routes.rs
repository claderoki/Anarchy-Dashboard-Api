use actix_web::Responder;
use actix_web::web;
use super::models::Poll;

pub async fn save_poll(poll: web::Json<Poll>) -> impl Responder {
    println!("{:?}", poll);
    format!("OK")
}

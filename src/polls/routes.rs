use crate::discord::calls::ChannelKind;
use crate::discord::models::Channel;
use crate::helpers::validator::Validator;

use super::models::Poll;
use actix_web::get;
use actix_web::post;
use actix_web::web;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;

#[post("/save")]
pub async fn save_poll(poll: web::Json<Poll>) -> impl Responder {
    println!("{:?}", poll);
    format!("OK")
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct PollSettings {
    allowed_channels: Vec<String>,
    allowed_changes: Vec<String>,
}

#[post("/save_settings")]
pub async fn save_poll_settings(poll: web::Json<PollSettings>) -> impl Responder {
    println!("{:?}", poll);
    format!("OK")
}

#[get("/{guild_id}/get_settings")]
pub async fn get_poll_settings(req: HttpRequest) -> HttpResponse {
    match Validator::new().validate(&req).await {
        Ok(_info) => HttpResponse::Ok().json(PollSettings {
            allowed_channels: vec![
                "906898585302499369".into(),
                "912721488187117578".into(),
                "906898600162906162".into(),
            ],
            allowed_changes: vec![
                "create_channel".into(),
                "delete_channel".into(),
                "assign_role".into(),
            ],
        }),
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum ChangeKeyKind {
    Member,
    Channel,
    String,
    Role,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum ChangeValueKind {
    Member,
    String,
    None,
    Role,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ChangeIdentifier {
    pub value: String,
    pub name: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ChangeInfo {
    pub identifier: ChangeIdentifier,
    pub key_kind: ChangeKeyKind,
    pub value_kind: ChangeValueKind,
}

#[get("/get_available_changes")]
pub async fn get_available_poll_changes(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(vec![
        ChangeInfo {
            identifier: ChangeIdentifier {
                value: "create_channel".into(),
                name: "Create channel".into(),
            },
            key_kind: ChangeKeyKind::String,
            value_kind: ChangeValueKind::None,
        },
        ChangeInfo {
            identifier: ChangeIdentifier {
                value: "delete_channel".into(),
                name: "Delete channel".into(),
            },
            key_kind: ChangeKeyKind::Channel,
            value_kind: ChangeValueKind::None,
        },
        ChangeInfo {
            identifier: ChangeIdentifier {
                value: "assign_role".into(),
                name: "Assign role".into(),
            },
            key_kind: ChangeKeyKind::Role,
            value_kind: ChangeValueKind::Member,
        },
    ])
}

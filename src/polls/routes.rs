use crate::discord::calls::ChannelKind;
use crate::helpers::validator::Validator;

use super::models::Poll;
use actix_web::get;
use actix_web::post;
use actix_web::web;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;

async fn get_allowed_channels(_guild_id: u64) -> Vec<Channel> {
    vec![
        Channel {
            id: 906898585302499369,
            name: "polls".into(),
            kind: ChannelKind::GuildText,
        },
        Channel {
            id: 906898600162906162,
            name: "data".into(),
            kind: ChannelKind::GuildText,
        },
    ]
}

async fn _get_allowed_roles(_guild_id: u64) -> Vec<Role> {
    vec![
        Role {
            id: 907341985365512323,
            name: "experimentation".into(),
        },
        Role {
            id: 907342012364226600,
            name: "Earthling".into(),
        },
    ]
}

#[post("/save")]
pub async fn save_poll(poll: web::Json<Poll>) -> impl Responder {
    println!("{:?}", poll);
    format!("OK")
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Channel {
    pub id: u64,
    pub name: String,
    pub kind: ChannelKind,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Member {
    pub id: u64,
    pub username: String,
    pub discriminator: u16,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Role {
    pub id: u64,
    pub name: String,
}

async fn get_poll_channels_result(req: &HttpRequest) -> Result<Vec<Channel>, String> {
    match Validator::new().validate(&req).await {
        Ok(info) => Ok(get_allowed_channels(info.guild_id).await),
        Err(err) => Err(format!("Validate failed: {}", err)),
    }
}

#[get("/{guild_id}/allowed_channels")]
pub async fn get_poll_channels(req: HttpRequest) -> HttpResponse {
    match get_poll_channels_result(&req).await {
        Ok(allowed_channels) => {
            return HttpResponse::Ok().json(allowed_channels);
        }
        Err(_err) => {
            println!("{}", _err);
            return HttpResponse::Unauthorized().finish();
        }
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

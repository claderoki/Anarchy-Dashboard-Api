use crate::helpers::repositories::discord::ChannelRepository;
use crate::helpers::repositories::discord::ChannelRepositoryOptions;
use crate::helpers::repositories::discord::MemberRepository;
use crate::helpers::repositories::discord::Repository;
use crate::helpers::repositories::discord::RoleRepository;

use crate::helpers::repositories::discord::SharedRepositoryOptions;
use crate::helpers::validator::get_allowed_guilds;
use crate::helpers::validator::parse_access_token;
use crate::helpers::validator::Validator;

use super::calls::ChannelKind;
use actix_web::get;
use actix_web::HttpRequest;
use actix_web::HttpResponse;

#[get("/get_mutual_guilds")]
pub async fn get_mutual_guilds(req: HttpRequest) -> HttpResponse {
    let access_token = parse_access_token(&req);

    match get_allowed_guilds(&access_token.as_ref().unwrap()).await {
        Ok(guilds) => HttpResponse::Ok().json(guilds),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[get("/{guild_id}/get_all_text_channels")]
pub async fn get_all_text_channels(req: HttpRequest) -> HttpResponse {
    match Validator::new().validate(&req).await {
        Ok(validation) => {
            match ChannelRepository::get(&ChannelRepositoryOptions(
                validation.guild_id,
                ChannelKind::GuildText,
            ))
            .await
            {
                Ok(channels) => HttpResponse::Ok().json(channels),
                Err(err) => HttpResponse::BadRequest().body(err),
            }
        }
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

#[get("/{guild_id}/get_all_roles")]
pub async fn get_all_roles(req: HttpRequest) -> HttpResponse {
    match Validator::new().validate(&req).await {
        Ok(validation) => {
            match RoleRepository::get(&SharedRepositoryOptions(validation.guild_id)).await {
                Ok(roles) => HttpResponse::Ok().json(roles),
                Err(err) => HttpResponse::BadRequest().body(err),
            }
        }
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

#[get("/{guild_id}/get_all_members")]
pub async fn get_all_members(req: HttpRequest) -> HttpResponse {
    match Validator::new().validate(&req).await {
        Ok(validation) => {
            match MemberRepository::get(&SharedRepositoryOptions(validation.guild_id)).await {
                Ok(members) => HttpResponse::Ok().json(members),
                Err(err) => HttpResponse::BadRequest().body(err),
            }
        }
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

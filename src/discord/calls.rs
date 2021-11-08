use super::base_api::Endpoint;

#[derive(serde::Deserialize, Debug)]
pub struct MeResponse {
    pub id: String,
}

pub struct GetMe;
impl Endpoint<MeResponse> for GetMe {
    fn get_endpoint(&self) -> String {
        "/users/@me".into()
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct GuildResponse {
    pub id: String,
    pub name: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(transparent)]
pub struct GuildsResponse {
    pub guilds: Vec<GuildResponse>,
}

pub struct GetGuilds;
impl Endpoint<GuildsResponse> for GetGuilds {
    fn get_endpoint(&self) -> String {
        "/users/@me/guilds".into()
    }
}
use serde_repr::*;
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ChannelKind {
    GuildText = 0,
    Dm = 1,
    GuildVoice = 2,
    GroupDm = 3,
    GuildCategory = 4,
    GuildNews = 5,
    GuildStore = 6,
    GuildNewsThread = 10,
    GuildPublicThread = 11,
    GuildPrivateThread = 12,
    GuildStageVoice = 13,
}

#[derive(serde::Deserialize, Debug)]
pub struct ChannelResponse {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: ChannelKind,
}

#[derive(serde::Deserialize, Debug)]
#[serde(transparent)]
pub struct ChannelsResponse {
    pub channels: Vec<ChannelResponse>,
}

pub struct GetChannels {
    pub guild_id: u64,
}

impl Endpoint<ChannelsResponse> for GetChannels {
    fn get_endpoint(&self) -> String {
        format!("/guilds/{}/channels", self.guild_id)
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct RoleResponse {
    pub id: String,
    pub name: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(transparent)]
pub struct RolesResponse {
    pub roles: Vec<ChannelResponse>,
}

pub struct GetRoles {
    pub guild_id: u64,
}

impl Endpoint<RolesResponse> for GetRoles {
    fn get_endpoint(&self) -> String {
        format!("/guilds/{}/roles", self.guild_id)
    }
}

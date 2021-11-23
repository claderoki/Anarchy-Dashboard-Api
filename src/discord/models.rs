use super::calls::ChannelKind;

#[warn(unused_imports)]
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub kind: ChannelKind,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Member {
    pub id: String,
    pub username: String,
    pub discriminator: u16,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Role {
    pub id: String,
    pub name: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Guild {
    pub id: String,
    pub name: String,
}

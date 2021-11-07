use crate::discord::discord_base::AccessToken;

use super::base_api::Endpoint;
use super::discord_base::DiscordCall;
use super::base_api::Callable;

#[derive(serde::Deserialize, Debug)]
pub struct MeResponse {
    id: String,
}

struct GetMe;
impl Endpoint<MeResponse> for GetMe {
    fn get_endpoint(&self) -> &str {
        "/users/@me"
    }
}

impl GetMe {
    pub fn new() -> Self {
        Self {}
    }
}

pub async fn test_get_me_call(access_token: &str) {
    let call = DiscordCall::new(AccessToken::bearer(access_token));
    let endpoint = GetMe::new();
    let result = call.call(endpoint).await;
    println!("{:?}", result);
}

use actix_web::HttpRequest;
use actix_web::Responder;
use serde::Deserialize;
use serde::Serialize;
use strum_macros::Display;

#[derive(Display)]
#[strum(serialize_all = "snake_case")]
enum OauthScope {
    Identify,
}

#[derive(Display)]
#[strum(serialize_all = "snake_case")]
enum ResponseType {
    Code,
}

struct OauthUrlSettings {
    pub scopes: Vec<OauthScope>,
    pub client_id: u64,
    pub redirect_uri: String,
    pub response_type: ResponseType,
}

struct OauthController;
impl OauthController {
    pub fn create_url(settings: OauthUrlSettings) -> String {
        format!("https://discord.com/oauth2/authorize?client_id={}&redirect_uri={}&response_type={}&scope={}",
            settings.client_id,
            settings.redirect_uri,
            settings.response_type,
            settings.scopes.iter().map(|s|format!("{}", s)).collect::<Vec<String>>().join(",")
        )
    }
}

#[derive(Display, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all    = "snake_case")]
enum GrantType {
    AuthorizationCode,
}

#[derive(Deserialize, Serialize)]
struct OauthBody {
    code: String,
    client_id: u64,
    client_secret: String,
    grant_type: GrantType,
    redirect_uri: String,
}

#[derive(Deserialize, Serialize)]
struct OauthResponse {}

pub async fn authenticate(req: HttpRequest) -> impl Responder {
    match req.match_info().get("code") {
        Some(code) => {
            let body = OauthBody {
                code: code.to_string(),
                client_id: 906651425961046046,
                client_secret: format!(""),
                grant_type: GrantType::AuthorizationCode,
                redirect_uri: format!("http://127.0.0.1:3000/authenticate"),
            };

            let client = reqwest::Client::new();
            let a = client
                .post("https://discord.com/api/oauth2/token")
                .json(&body);

            println!("{:?}", a);
        }
        None => todo!(),
    }

    format!("OK")
}

pub async fn oauth_url() -> impl Responder {
    OauthController::create_url(OauthUrlSettings {
        scopes: vec![OauthScope::Identify],
        client_id: 906651425961046046,
        redirect_uri: format!("http://127.0.0.1:3000/authenticate"),
        response_type: ResponseType::Code,
    })
}

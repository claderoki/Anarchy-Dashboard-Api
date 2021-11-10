// use actix_web::HttpRequest;
// use async_trait::async_trait;

// #[async_trait]
// pub trait Validator<D: Sync, F: Send> {
//     async fn validate(d: &D) -> F;
// }

// pub struct RequestValidationInfo;
// pub type RequestValidationResult = Result<RequestValidationInfo, String>;

// pub struct RequestValidator;
// #[async_trait]
// impl Validator<HttpRequest, RequestValidationResult> for RequestValidator {
//     async fn validate(req: &HttpRequest) -> RequestValidationResult {
//         todo!()
//     }
// }

use actix_web::{
    dev::ServiceRequest, http::StatusCode,  HttpResponse, ResponseError
};
use actix_web_httpauth::extractors::basic::BasicAuth;

use serde_json::json;
use thiserror::Error;
use tracing::{instrument, info};

#[instrument(skip(req, _credentials))]
pub async fn basic_auth_validator(
    req: ServiceRequest,
    _credentials: BasicAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    info!("start validator");
    info!("credencials: {:?}", _credentials);
    info!("user_id: {:?}", _credentials.user_id());
    info!("password: {:?}", _credentials.password());

    let check = _credentials.user_id() == "aura" && _credentials.password() == Some("frieren");
    if !check {
        return Err((RuntimeError::InvalidCredential.into(), req))
    }
    info!("basic auth success");
    Ok(req)
}



#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Invalid")]
    InvalidCredential,
}

impl ResponseError for RuntimeError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use StatusCode as S;

        match self {
            RuntimeError::InvalidCredential => S::BAD_REQUEST,
        }

    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let mut res = HttpResponse::build(self.status_code());

        res.content_type(mime::APPLICATION_JSON).json(json!({
            "error": self.to_string()
        }))
    }
    

}


// #[instrument(skip(req, next))]
// pub async fn basic_auth(
//     req: ServiceRequest,
//     next: Next<impl MessageBody>,
// ) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
//     info!("start basic_auth_handler");

//     let auth = req.headers().get("Authorization");
//     info!("auth: {:?}", auth);
//     // {
//     //     // ユーザーエージェントを取得
//     //     let user_agent = req
//     //         .headers()
//     //         .get("User-Agent")
//     //         .unwrap()
//     //         .to_str()
//     //         .unwrap()
//     //         .to_string();
//     //     // UAを保存
//     //     let device_os = DeviceOs::new(user_agent);

//     //     let _a = req.extensions_mut().insert(device_os);
//     //     let a = req.extensions();
//     //     let a = a.get::<DeviceOs>();
//     //     // a.insert("device_os");
//     //     info!("lab capture os{:?}", a)

//     // }

//     let res = next.call(req).await?;
//     info!("end basic_auth_handler");
//     Ok(res)
// }

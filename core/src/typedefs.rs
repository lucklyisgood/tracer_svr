use std::fmt::Display;

use serde::Serialize;
use serde_json::to_string;
use actix_web::{error, body::BoxBody, Responder, http::header::ContentType, HttpRequest, HttpResponse};

#[derive(Serialize)]
pub struct Response<T> {
    pub code: u32,
    pub msg: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> Default for Response<T> {
    fn default() -> Self {
        Self { code: 0, msg: "succ".into(), data: None }
    }
}

impl<T> std::convert::From<T> for Response<T> {
    fn from(v: T) -> Self {
        let mut resp = Response::default();
        resp.data = Some(v);
        resp
    }
}

impl<T> Responder for Response<T> where T : serde::Serialize {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = to_string(&self).unwrap();
        HttpResponse::Ok().content_type(ContentType::json()).body(body)
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorRespon {
    pub code: u32,
    pub msg: String,
}

impl Display for ErrorRespon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_str = to_string(&self).unwrap();
        f.write_str(&format!("ErrorRespon: {}", display_str))
    }
}

impl error::ResponseError for ErrorRespon {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = to_string(&self).unwrap();
        HttpResponse::Ok().content_type(ContentType::json()).body(body)
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}
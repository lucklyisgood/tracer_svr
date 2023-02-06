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

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("base err: {0}")]
    Err(String),

    #[error("internal error")]
    InternalError,

    #[error("bad request params")]
    BadClientData,

    #[error("db fail: {0}")]
    DBError(#[from] rbatis::rbdc::Error),
}

#[derive(Debug, Serialize)]
pub struct _ErrorRespon {
    code: u32,
    msg: String,
}

use self::Error::*;
impl Error {
    pub fn to_body(&self) -> _ErrorRespon {
        match self {
            #[allow(unreachable_patterns)]
            _ => _ErrorRespon {
                code: self.status_code(),
                msg: "server undefind error".into(),
            }
        }
    }

    pub fn status_code(&self) -> u32 {
        match self {
            Err(_) => 1001,
            InternalError => 1002,
            BadClientData => 1003,
            DBError(_) => 2001,

            #[allow(unreachable_patterns)]
            _ => 1001,
        }
    }
}

impl Responder for Error {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = to_string(&self.to_body()).unwrap();
        HttpResponse::Ok().content_type(ContentType::json()).body(body)
    }
}

impl error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = to_string(&self.to_body()).unwrap();
        HttpResponse::Ok().content_type(ContentType::json()).body(body)
    }
}
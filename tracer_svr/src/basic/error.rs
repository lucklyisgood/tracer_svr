
use serde::Serialize;
use serde_json::to_string;
use actix_web::{body::BoxBody, Responder, http::header::ContentType, HttpRequest, HttpResponse};

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("base err: {0}")]
    Err(String),

    #[error("internal error")]
    InternalError,
}

#[derive(Debug, Serialize)]
pub struct _ErrorRespon {
    code: u32,
    msg: String,
}

// use self::Error::*;
impl Error {
    pub fn to_body(&self) -> _ErrorRespon {
        match self {
            #[allow(unreachable_patterns)]
            _ => _ErrorRespon {
                code: 1001,
                msg: "server undefind error".into(),
            }
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
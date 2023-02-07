use actix_web::{error, body::BoxBody};
use core::typedefs::ErrorRespon;

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[allow(dead_code)]
    #[error("base err: {0}")]
    Err(String),

    #[allow(dead_code)]
    #[error("internal error")]
    InternalError,

    #[allow(dead_code)]
    #[error("bad request params: {0}")]
    BadClientData(String),

    #[error("db fail: {0}")]
    DBError(#[from] rbatis::rbdc::Error),

    #[error("create proj fail: {0}")]
    CreateProjError(String),
}

use self::Error::*;
impl Error {
    pub fn to_body(&self) -> ErrorRespon {
        match self {
            DBError(_) => ErrorRespon { code: self.status_code(), msg: "internal error".into() },
            #[allow(unreachable_patterns)]
            _ => ErrorRespon {
                code: self.status_code(),
                msg: format!("{}", self),
            }
        }
    }

    pub fn status_code(&self) -> u32 {
        match self {
            DBError(_) => 1002,
            InternalError => 1002,
            BadClientData(_) => 1003,

            CreateProjError(_) => 2001,

            #[allow(unreachable_patterns)]
            _ => 1001,
        }
    }
}

impl error::ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::OK
    }

    fn error_response(&self) -> actix_web::HttpResponse<BoxBody> {
        self.to_body().error_response()
    }
}


#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("base err: {0}")]
    Err(String),
}
use std::result::Result as StdResult;
use thiserror::Error;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

// #[derive(Debug, Error)]
// pub enum Error {
//     #[error("missed on sending api request")]
//     SendRequestError(reqwest::Error),
//     #[error("cannot deserialize response")]
//     ResponseDeserializeError(reqwest::Error),
// }

// impl From<reqwest::Error> for Error {
//     fn from(err: reqwest::Error) -> Error {
//         if err.is_builder() {
//             return Error::SendRequestError(err);
//         }
//         return Error::ResponseDeserializeError(err);
//     }
// }

use chacha20poly1305::aead;
use std::io;

#[derive(Debug)]
pub enum Error {
    TokenMustBeUnlocked,
    SavingUnlockedConfiguration,
    TokenMustHavePassword,
    TokenRequired,
    UnusedPassword,
    UserNameNotProvided,
    UserExists,
    UserDoesNotExist,
    Io(io::Error),
    Aead(aead::Error),
    Json(serde_json::error::Error),
    Reqwest(reqwest::Error),
}

impl From<aead::Error> for Error {
    fn from(e: aead::Error) -> Error {
        Error::Aead(e)
    }
}
impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Error {
        Error::Json(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::Reqwest(e)
    }
}

use poise::serenity_prelude as serenity;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An error occured within the Serenity framework.
    #[error("Serenity error: {0}")]
    SerenityError(#[from] serenity::Error),
    /// A permission error occured.
    #[error("{0}")]
    PermissionError(String),
    // /// A SurrealDB error occured.
    // #[error("SurrealDB error: {0}")]
    // SurrealError(surrealdb::Error),
    /// A Songbird input error occured.
    #[error("Songbird input error: {0}")]
    SongbirdInputError(songbird::input::AudioStreamError),
    /// A Songbird join error occured.
    #[error("Songbird join error: {0}")]
    SongbirdJoinError(songbird::error::JoinError),
    /// A URL parsing error occured.
    #[error("URL parsing error: {0}")]
    UrlParseError(#[from] url::ParseError),
    /// A session manager error occured.
    #[error("{0}")]
    SessionManagerError(#[from] crate::session::SessionError),
    /// A custom error occured.
    #[error("{0}")]
    Custom(&'static str),
}

impl Error {
    /// Returns true if this error can be shown to the user.
    pub fn is_user_facing(&self) -> bool {
        match self {
            Error::SessionManagerError(e) => e.is_user_facing(),
            Error::PermissionError(_) | Error::Custom(_) => true,
            _ => false,
        }
    }

    /// Converts this error into a boxed error.
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

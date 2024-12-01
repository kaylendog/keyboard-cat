//! The context module contains the context struct and error type used throughout the bot.

use std::collections::HashMap;

use poise::serenity_prelude as serenity;
use reqwest::Client as HttpClient;
use songbird::tracks::TrackHandle;
// use surrealdb::{
//     engine::local::{Db, Mem},
//     Surreal,
// };
use tokio::sync::RwLock;

use crate::session::SessionManager;

pub struct InstanceContext {
    // pub db: RwLock<Surreal<Db>>,
    pub http: HttpClient,
    pub track_handles: RwLock<HashMap<serenity::GuildId, TrackHandle>>,
    pub session_manager: SessionManager,
}

impl InstanceContext {
    pub async fn init() -> Result<Self, Box<Error>> {
        // let db = Surreal::new::<Mem>(()).await.map_err(Error::SurrealError)?;
        Ok(Self {
            // db: db.into(),
            http: HttpClient::default(),
            track_handles: Default::default(),
            session_manager: SessionManager::default(),
        })
    }
}

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
    /// A custom error occured.
    #[error("{0}")]
    Custom(&'static str),
}

impl Error {
    /// Returns true if this error can be shown to the user.
    pub fn is_user_facing(&self) -> bool {
        matches!(self, Error::PermissionError(_) | Error::Custom(_))
    }

    /// Converts this error into a boxed error.
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

pub type Context<'a> = poise::Context<'a, InstanceContext, Box<Error>>;

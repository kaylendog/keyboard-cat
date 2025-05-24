//! The context module contains the context struct and error type used throughout the bot.

use reqwest::Client as HttpClient;

use crate::session::SessionManager;

mod error;

pub use error::Error;

pub struct Context {
    // pub db: RwLock<Surreal<Db>>,
    pub http: HttpClient,
    pub session_manager: SessionManager,
}

impl Context {
    pub async fn init() -> Result<Self, Box<Error>> {
        Ok(Self {
            // db: db.into(),
            http: HttpClient::default(),
            session_manager: SessionManager::default(),
        })
    }
}

/// The context type used throughout the bot.
pub type Ctx<'a> = poise::Context<'a, Context, Box<Error>>;

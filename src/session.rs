use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use poise::serenity_prelude as serenity;
use songbird::tracks::TrackHandle;
use tokio::sync::{Mutex, RwLock};

use crate::context::Ctx;

/// A manager for all active listening sessions.
#[derive(Default)]
pub struct SessionManager {
    /// A map of guild IDs to their respective sessions.
    sessions: RwLock<HashMap<serenity::GuildId, Arc<Session>>>,
}

impl SessionManager {
    /// Fetches the session for the current guild, if it exists.
    pub async fn get(&self, ctx: &Ctx<'_>) -> Option<Arc<Session>> {
        self.sessions
            .read()
            .await
            .get(&ctx.guild_id().unwrap())
            .cloned()
    }

    /// Gets the session for the current guild, or creates a new one if it doesn't exist.
    pub async fn get_or_create(
        &self,
        ctx: &Ctx<'_>,
        channel: Option<serenity::ChannelId>,
    ) -> Result<Arc<Session>, SessionError> {
        if let Some(session) = self.get(ctx).await {
            return Ok(session);
        }

        // get user voice channel
        let channel = channel.map(|c| Ok(c)).unwrap_or_else(|| {
            match ctx
                .guild()
                .unwrap()
                .voice_states
                .get(&ctx.author().id)
                .and_then(|voice_state| voice_state.channel_id)
            {
                Some(channel) => Ok(channel),
                None => Err(SessionError::MissingVoiceChannel),
            }
        })?;

        // convert to guild channel
        let channel = channel
            .to_channel(ctx.serenity_context())
            .await
            .map_err(|e| SessionError::SerenityError(e))?
            .guild()
            .ok_or(SessionError::MissingVoiceChannel)?;

        Session::create(ctx, channel)
            .await
            .map(|session| Arc::new(session))
    }

    /// Destroys the session for the current guild, if it exists.
    pub async fn destroy(&self, ctx: &Ctx<'_>) -> Result<(), SessionError> {
        if let Some(session) = { self.sessions.write().await.remove(&ctx.guild_id().unwrap()) } {
            session.destroy().await;
        }
        Ok(())
    }
}

/// A listening session is a fancy way of saying "a queue of tracks", with some additional instantiation and tidy-up logic.
///
/// The session is immediately destroyed when there are no users left in the voice channel.
pub struct Session {
    /// A reference to Songbird's [`Call`], which is used to manage the voice connection.
    pub call: Arc<Mutex<songbird::Call>>,
    /// The voice channel this session is bound to.
    pub channel: serenity::GuildChannel,
    /// A queue of input sources to play.
    pub tracks: RwLock<VecDeque<TrackInfo>>,
    /// A handle to the currently playing track.
    pub playing: Option<TrackHandle>,
}

impl Session {
    /// Creates a new session with the given voice channel, or uses the author's voice channel if none is provided.
    pub async fn create(
        ctx: &Ctx<'_>,
        channel: serenity::GuildChannel,
    ) -> Result<Self, SessionError> {
        // instantiate call
        let manager = songbird::get(ctx.serenity_context()).await.unwrap();
        let call = manager
            .join(ctx.guild_id().unwrap(), channel.id)
            .await
            .map_err(|e| SessionError::CallJoinError(e))?;

        Ok(Self {
            call,
            channel,
            tracks: RwLock::new(VecDeque::new()),
            playing: None,
        })
    }

    //// Append a track to the end of the queue.
    async fn append(&self, source: String, requester: serenity::UserId) {
        self.tracks
            .write()
            .await
            .push_back(TrackInfo { source, requester });
    }

    //// Play the next track in the queue. This will skip any currently playing track.
    async fn next(&self) -> Result<(), SessionError> {
        if let Some(track) = self.tracks.write().await.pop_front() {
            todo!()
        }

        Ok(())
    }

    /// Stop the current track without advancing to the next track.
    async fn stop(&self) -> Result<(), SessionError> {
        if let Some(handle) = &self.playing {
            handle.stop().map_err(SessionError::SongbirdControlError)?;
        }
        Ok(())
    }

    /// Gracefully destroy the session, leaving the voice channel and stopping the current track.
    async fn destroy(&self) {
        let _ = self.stop().await;
        let _ = self.call.lock().await.leave().await;
    }
}

/// An enumeration of possible errors that can occur while managing a session.
#[derive(thiserror::Error, Debug)]
pub enum SessionError {
    /// No voice channel was specified and the author is not in a voice channel.
    #[error("No voice channel specified and author is not in a voice channel")]
    MissingVoiceChannel,
    /// An error occurred while trying to join the voice channel.
    #[error("Failed to join voice channel")]
    CallJoinError(#[from] songbird::error::JoinError),
    /// An error occured upstream in the Serenity library.
    #[error("Serenity error: {0}")]
    SerenityError(#[from] serenity::Error),
    /// Session already exists for this guild.
    #[error("Session already exists for this guild")]
    SessionExists(serenity::ChannelId),
    /// An error occurred while performing a Songbird operation.
    #[error("Songbird error: {0}")]
    SongbirdControlError(#[from] songbird::error::ControlError),
}

impl SessionError {
    pub fn is_user_facing(&self) -> bool {
        matches!(
            self,
            SessionError::MissingVoiceChannel | SessionError::SessionExists(_)
        )
    }
}

/// A queue entry is a request to play a track from a source URL.
pub struct TrackInfo {
    source: String,
    requester: serenity::UserId,
}

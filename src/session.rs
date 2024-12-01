use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use poise::serenity_prelude as serenity;
use songbird::tracks::TrackHandle;
use tokio::sync::{Mutex, RwLock};

use crate::context::Context;

/// A manager for all active listening sessions.
#[derive(Default)]
pub struct SessionManager {
    /// A map of guild IDs to their respective sessions.
    sessions: RwLock<HashMap<serenity::GuildId, Arc<Session>>>,
}

impl SessionManager {
    /// Creates a new session, failing if one already exists for the guild.
    pub async fn create(
        &self,
        ctx: &Context<'_>,
        channel: Option<serenity::ChannelId>,
    ) -> Result<Arc<Session>, SessionError> {
        let exists = self
            .sessions
            .read()
            .await
            .contains_key(&ctx.guild_id().unwrap());

        if exists {
            // get the channel ID of the existing session
            let id = self.sessions.read().await[&ctx.guild_id().unwrap()]
                .channel
                .id;
            return Err(SessionError::SessionExists(id));
        }

        let session = Arc::new(Session::create(ctx, channel).await?);
        self.sessions
            .write()
            .await
            .insert(ctx.guild_id().unwrap(), session.clone());

        Ok(session)
    }

    /// Plays a track in the session for the given guild, appending it to the queue.
    pub async fn play(
        &self,
        guild_id: serenity::GuildId,
        source: String,
        requester: serenity::UserId,
    ) {
        if let Some(session) = self.sessions.read().await.get(&guild_id) {
            session.append(source, requester).await;
        }
    }

    /// Destroys the session for the given guild, if it exists.
    pub async fn destroy(&self, guild_id: serenity::GuildId) {
        if let Some(session) = self.sessions.write().await.remove(&guild_id) {
            session.destroy().await;
        }
    }
}

/// A listening session is a fancy way of saying "a queue of tracks", with some additional instantiation and tidy-up logic.
///
/// The session is immediately destroyed when there are no users left in the voice channel.
pub struct Session {
    /// A reference to Songbird's [`Call`], which is used to manage the voice connection.
    call: Arc<Mutex<songbird::Call>>,
    /// The voice channel this session is bound to.
    channel: serenity::GuildChannel,
    /// A queue of input sources to play.
    tracks: RwLock<VecDeque<TrackInfo>>,
    /// A handle to the currently playing track.
    playing: Option<TrackHandle>,
}

impl Session {
    /// Creates a new session with the given voice channel, or uses the author's voice channel if none is provided.
    pub async fn create(
        ctx: &Context<'_>,
        channel: Option<serenity::ChannelId>,
    ) -> Result<Self, SessionError> {
        // big ass monad fun
        let channel = channel
            .or_else(|| {
                ctx.guild()
                    .unwrap()
                    .voice_states
                    .get(&ctx.author().id)
                    .and_then(|voice_state| voice_state.channel_id)
            })
            .ok_or(SessionError::MissingVoiceChannel)?
            .to_channel(ctx.serenity_context())
            .await
            .map_err(|e| SessionError::SerenityError(e))?
            .guild()
            .ok_or(SessionError::MissingVoiceChannel)?;

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

/// A queue entry is a request to play a track from a source URL.
pub struct TrackInfo {
    source: String,
    requester: serenity::UserId,
}

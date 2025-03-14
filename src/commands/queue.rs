use poise::serenity_prelude as serenity;

use crate::context::{Context, Error};

/// Search for a song.
#[poise::command(slash_command, guild_only)]
pub async fn queue(
    ctx: Context<'_>,
    #[description = "Youtube URL or query"] input: String,
    #[description = "Voice channel to join"] channel: Option<serenity::ChannelId>,
) -> Result<(), Box<Error>> {
    // get the session
    let session = ctx
        .data()
        .session_manager
        .create(&ctx, channel)
        .await
        .map_err(Error::SessionManagerError)?;

    // inspect the queue
    let tracks = session.tracks.read().await;

    Ok(())
}

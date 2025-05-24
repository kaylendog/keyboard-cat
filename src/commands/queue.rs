use poise::serenity_prelude as serenity;

use crate::context::{Ctx, Error};

/// Search for a song.
#[poise::command(slash_command, guild_only)]
pub async fn queue(ctx: Ctx<'_>) -> Result<(), Box<Error>> {
    // get the session
    let session = ctx
        .data()
        .session_manager
        .get_or_create(&ctx, None)
        .await
        .map_err(Error::SessionManagerError)?;

    // inspect the queue
    let tracks = session.tracks.read().await;

    Ok(())
}

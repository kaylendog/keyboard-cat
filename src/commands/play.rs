use poise::serenity_prelude as serenity;

use crate::context::{Ctx, Error};

/// Play a video from a Youtube URL.
#[poise::command(slash_command, guild_only)]
pub async fn play(
    ctx: Ctx<'_>,
    #[description = "Youtube URL or query"] input: String,
    #[description = "Voice channel to join"] channel: Option<serenity::ChannelId>,
) -> Result<(), Box<Error>> {
    let session = ctx
        .data()
        .session_manager
        .get_or_create(&ctx, channel)
        .await
        .map_err(Error::SessionManagerError)?;

    Ok(())
}

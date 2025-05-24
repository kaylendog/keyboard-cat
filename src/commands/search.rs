use poise::serenity_prelude as serenity;

use crate::context::{Ctx, Error};

/// Search for a song.
#[poise::command(slash_command, guild_only)]
pub async fn search(
    ctx: Ctx<'_>,
    #[description = "Youtube URL or query"] input: String,
    #[description = "Voice channel to join"] channel: Option<serenity::ChannelId>,
) -> Result<(), Box<Error>> {
    let session = ctx
        .data()
        .session_manager
        .get_or_create(&ctx, channel)
        .await
        .unwrap();

    Ok(())
}

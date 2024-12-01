use poise::serenity_prelude as serenity;

use crate::context::{Context, Error};

/// Play a video from a Youtube URL.
#[poise::command(slash_command, guild_only)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Youtube URL or query"] input: String,
    #[description = "Voice channel to join"] channel: Option<serenity::ChannelId>,
) -> Result<(), Box<Error>> {
    let session = ctx
        .data()
        .session_manager
        .create(&ctx, channel)
        .await
        .unwrap();

    Ok(())
}

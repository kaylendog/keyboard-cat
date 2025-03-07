use poise::serenity_prelude as serenity;

use crate::context::{Context, Error};

/// Search for a song.
#[poise::command(slash_command, guild_only)]
pub async fn search(
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

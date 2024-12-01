use crate::context::{Context, Error};

//// Leave the voice channel.
#[poise::command(slash_command, guild_only)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Box<Error>> {
    let manager = songbird::get(ctx.serenity_context()).await.unwrap();
    let guild_id = ctx.guild_id().unwrap();

    // leave the voice channel
    let call = manager
        .get(guild_id)
        .ok_or(Error::Custom(":x: Not in a voice channel.").boxed())?;
    call.lock()
        .await
        .leave()
        .await
        .map_err(|e| Error::SongbirdJoinError(e).boxed())?;

    // remove the track handle
    ctx.data().track_handles.write().await.remove(&guild_id);

    ctx.reply(":white_check_mark: Left voice channel.")
        .await
        .map_err(|e| Error::SerenityError(e).boxed())?;

    Ok(())
}

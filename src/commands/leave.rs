use crate::context::{Ctx, Error};

//// Leave the voice channel.
#[poise::command(slash_command, guild_only)]
pub async fn leave(ctx: Ctx<'_>) -> Result<(), Box<Error>> {
    // destroy session
    ctx.data()
        .session_manager
        .destroy(&ctx)
        .await
        .map_err(Error::SessionManagerError)?;

    ctx.reply(":white_check_mark: Left voice channel.")
        .await
        .map_err(|e| Error::SerenityError(e).boxed())?;

    Ok(())
}

use poise::serenity_prelude::{self as serenity, CacheHttp};

use crate::context::{Ctx, Error};

/// Utility trait for [`serenity::ChannelId`].
pub trait ChannelIdExt {
    /// Returns the kind of channel this ID represents, returning None if the channel does not exist or is not in the same guild as the context.
    async fn kind(&self, ctx: &Ctx<'_>) -> Result<Option<serenity::ChannelType>, Box<Error>>;
}

impl ChannelIdExt for serenity::ChannelId {
    async fn kind(&self, ctx: &Ctx<'_>) -> Result<Option<serenity::ChannelType>, Box<Error>> {
        self.to_channel(ctx.http())
            .await
            .map(|channel| match channel {
                serenity::Channel::Guild(channel) => {
                    // prevent cross-guild shenanigans
                    if channel.guild_id == ctx.guild_id().unwrap() {
                        Some(channel.kind)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .map_err(|e| Box::new(Error::SerenityError(e)))
    }
}

use std::sync::Arc;
use songbird::Songbird;
use crate::{Context, StdError};
use crate::types::playground::Playground;
use crate::utils::{connect_to_channel_from_ctx, extract_from_ctx};

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn join(ctx: Context<'_>) -> Result<(), StdError> {
    connect_to_channel_from_ctx(ctx).await
}

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn leave(ctx: Context<'_>) -> Result<(), StdError> {
    let (playground, _, manager): (Playground, _, &Arc<Songbird>) = extract_from_ctx(ctx);

    if manager.get(playground.guild_id).is_some() {
        if let Err(e) = manager.remove(playground.guild_id).await {
            ctx.reply("Could not leave channel.").await?;
            println!("{:?}", e);
            return Ok(());
        }
        ctx.reply("Left voice channel.").await?;
    } else {
        ctx.reply("Not in a voice channel.").await?;
    }

    Ok(())
}

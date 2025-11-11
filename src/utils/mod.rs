use std::sync::Arc;
use songbird::{Songbird, TrackEvent};
use crate::{Context, Data, StdError, TrackErrorNotifier};
use crate::types::playground::Playground;

pub fn extract_from_ctx(ctx: Context<'_>) -> (Playground, &Data, &Arc<Songbird>) {
    let playground: Playground = ctx.into();
    let data = ctx.data();
    (playground, data, &data.songbird)
}

pub async fn connect_to_channel_from_ctx(ctx: Context<'_>) -> Result<(), StdError> {
    let (playground, _, manager): (Playground, _, &Arc<Songbird>) = extract_from_ctx(ctx);

    let channel_id = match playground.channel_id {
        Some(id) => id,
        None => {
            ctx.reply("You are not in a voice channel.").await?;
            return Ok(());
        }
    };

    if let Ok(handler_lock) = manager.join(playground.guild_id, channel_id).await {
        ctx.reply("Joined!").await?;
        let mut handler = handler_lock.lock().await;
        handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier)
    }

    Ok(())
}

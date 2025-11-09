use std::sync::Arc;
use std::time::Duration;
use songbird::Songbird;
use crate::{Context, Error};
use crate::types::format_duration::FormatDuration;
use crate::types::playground::Playground;
use crate::utils::extract_from_ctx;

#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn seek(ctx: Context<'_>, seconds: u64) -> Result<(), Error> {
    let (playground, _, manager): (Playground, _, &Arc<Songbird>) = extract_from_ctx(ctx);

    if let Some(handler_lock) = manager.get(playground.guild_id) {
        let handler = handler_lock.lock().await;

        if let Some(current_track) = handler.queue().current() {
            let duration = Duration::from_secs(seconds);
            // cant backseek idk why
            if current_track.get_info().await?.position < duration {
                match current_track.seek_async(duration).await {
                    Ok(..) => { ctx.say(format!("Track successfully seeked to {}", FormatDuration::from(duration))).await?; }
                    Err(err) => { ctx.say(format!("Failed to seek to {}", FormatDuration::from(duration))).await?; }
                }
            } else {
                ctx.say("Can't seek back in the track.").await?;
            }
        }
    }

    Ok(())
}
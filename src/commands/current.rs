use std::sync::Arc;
use std::time::Duration;
use poise::CreateReply;
use songbird::Songbird;
use songbird::tracks::PlayMode;
use crate::{Context, StdError};
use crate::types::format_duration::FormatDuration;
use crate::types::metadata_queue::MetadataObject;
use crate::types::playground::Playground;
use crate::utils::extract_from_ctx;

#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn seek(ctx: Context<'_>, seconds: u64) -> Result<(), StdError> {
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

#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn pause(ctx: Context<'_>) -> Result<(), StdError> {
    let (playground, _, manager): (Playground, _, &Arc<Songbird>) = extract_from_ctx(ctx);

    if let Some(handler_lock) = manager.get(playground.guild_id) {
        let handler = handler_lock.lock().await;
        if let Some(current_track) = handler.queue().current() {
            match current_track.get_info().await?.playing {
                PlayMode::Play => {
                    current_track.pause()?;
                    ctx.say("Track paused!").await?;
                }
                PlayMode::Pause => {
                    current_track.play()?;
                    ctx.say("Track unpaused!").await?;
                }
                _ => {
                    ctx.say("idk").await?;
                }
            }
        } else {
            ctx.say("No song currently playing.").await?;
        }
    }

    Ok(())
}

#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn now_playing(ctx: Context<'_>) -> Result<(), StdError> {
    let (playground, _, manager): (Playground, _, &Arc<Songbird>) = extract_from_ctx(ctx);

    if let Some(handler_lock) = manager.get(playground.guild_id) {
        let handler = handler_lock.lock().await;
        if let Some(current_track) = handler.queue().current() {
            let data = current_track.data::<MetadataObject>();
            ctx.send(CreateReply::default().embed(data.to_embed())).await?;
        } else {
            ctx.say("No song currently playing").await?;
        }
    }

    Ok(())
}
use std::sync::Arc;
use poise::CreateReply;
use serenity::all::CreateEmbed;
use songbird::Songbird;
use crate::{Context, Error};
use crate::{Context, StdError};
use crate::types::playground::Playground;
use crate::utils::extract_from_ctx;

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn queue(ctx: Context<'_>) -> Result<(), StdError> {
    let (playground, _, manager): (Playground, _, &Arc<Songbird>) = extract_from_ctx(ctx);

    if let Some(handler_lock) = manager.get(playground.guild_id) {
        let handler = handler_lock.lock().await;

        let queue = handler.queue();

        if queue.is_empty() {
            ctx.say("Queue is empty.").await?;
        }

        let embed = CreateEmbed::new().title("Music queue")
            .description("List of musics in the queue.");

        let mut counter = 0;
        let mut description = "".to_string();

        for track in queue.current_queue() {
            counter += 1;
            let data = track.data::<MetadataObject>();
            description.push_str(format!("{counter}. {data}\n").as_str());
        }

        ctx.send(CreateReply::default().embed(embed.description(description))).await?;
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn skip(ctx: Context<'_>) -> Result<(), StdError> {
    let (playground, _, manager): (Playground, _, &Arc<Songbird>) = extract_from_ctx(ctx);

    if let Some(handler_lock) = manager.get(playground.guild_id) {
        let handler = handler_lock.lock().await;

        let song_to_skip = handler.queue().current();
        if song_to_skip.is_none() {
            ctx.reply("There is currently no song playing.").await?;
            return Ok(());
        }

        ctx.reply(match handler.queue().skip() {
            Ok(()) => "Skipped song.",
            Err(_) => "Could not skip song.",
        })
            .await?;
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn remove(ctx: Context<'_>, position: usize) -> Result<(), StdError> {
    let (playground, _, manager): (Playground, _, &Arc<Songbird>) = extract_from_ctx(ctx);

    if let Some(handler_lock) = manager.get(playground.guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();

        if let Some(track) = queue.dequeue(position - 1) {
            let metadata = track.data::<MetadataObject>();
            ctx.send(CreateReply::default().content("Removed song").embed(metadata.to_embed())).await?;
        } else {
            ctx.say("Queue is empty or not that long.").await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn clear(ctx: Context<'_>) -> Result<(), StdError> {
    let (playground, _, manager): (Playground, _, &Arc<Songbird>) = extract_from_ctx(ctx);

    if let Some(handler_lock) = manager.get(playground.guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();

        while !queue.is_empty() {
            queue.skip()?;
        }
    }

    Ok(())
}
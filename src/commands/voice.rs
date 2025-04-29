use poise::reply;
use songbird::TrackEvent;
use songbird::input::Compose;
use songbird::input::YoutubeDl;

use crate::Context;
use crate::Error;
use crate::TrackErrorNotifier;
use crate::types::playground::Playground;

fn extract_metadata(metadata: Option<String>, replacement: String) -> String {
    match metadata {
        Some(value) => {
            let mut chars = value.chars();
            chars.next();
            chars.next_back();
            chars.as_str().to_string()
        }
        None => replacement,
    }
}

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn connect(ctx: Context<'_>) -> Result<(), Error> {
    let playground = Playground::from(ctx);

    let channel_id = match playground.channel_id {
        Some(id) => id,
        None => {
            ctx.reply("You are not in a voice channel.").await?;
            return Ok(());
        }
    };

    let manager = &ctx.data().songbird;

    if let Ok(handler_lock) = manager.join(playground.guild_id, channel_id).await {
        ctx.reply("Joined!").await?;
        let mut handler = handler_lock.lock().await;
        handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier)
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let playground = Playground::from(ctx);

    let manager = &ctx.data().songbird;
    let has_handler = manager.get(playground.guild_id).is_some();

    if has_handler {
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

#[poise::command(prefix_command, guild_only)]
pub async fn play(ctx: Context<'_>, url: String) -> Result<(), Error> {
    let playground = Playground::from(ctx);

    let do_search = !url.starts_with("http");

    let data = ctx.data();

    let manager = &data.songbird;

    if let Some(handler_lock) = manager.get(playground.guild_id) {
        let mut handler = handler_lock.lock().await;

        let src = if do_search {
            YoutubeDl::new_search(data.http.clone(), url)
        } else {
            YoutubeDl::new(data.http.clone(), url)
        };
        let metadata = src.clone().aux_metadata().await?;
        let _ = handler.enqueue_input(src.into()).await;

        ctx.reply(format!(
            "Added song [**\"{}\"**]({}) from **{}**.",
            metadata.title.unwrap_or("Unknown media".to_string()),
            metadata.source_url.unwrap_or("Unknown source".to_string()),
            metadata.channel.unwrap_or("Unknown channel".to_string())
        ))
        .await?;
    } else {
        ctx.reply("Not in a voice channel to play in.").await?;
    }

    Ok(())
}

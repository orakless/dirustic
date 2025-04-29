use songbird::TrackEvent;
use songbird::input::YoutubeDl;

use crate::Context;
use crate::Error;
use crate::TrackErrorNotifier;
use crate::types::playground::Playground;

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
        let _ = handler.play_input(src.into());

        ctx.reply("Playing song").await?;
    } else {
        ctx.reply("Not in a voice channel to play in").await?;
    }

    Ok(())
}

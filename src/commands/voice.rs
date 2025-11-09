use std::sync::Arc;
use poise::CreateReply;
use serenity::all::{CreateEmbed, EditMessage};
use songbird::Songbird;
use songbird::input::Compose;
use songbird::input::YoutubeDl;
use songbird::tracks::Track;
use crate::{Context, Data};
use crate::Error;
use crate::types::metadata_queue::MetadataObject;
use crate::types::playground::Playground;
use crate::utils::{connect_to_channel_from_ctx, extract_from_ctx};

const ITEMS_PER_QUEUE_PAGES: usize = 10;

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn play(ctx: Context<'_>, url: String) -> Result<(), Error> {
    let (playground, data, manager): (Playground, &Data, &Arc<Songbird>) = extract_from_ctx(ctx);


    if manager.get(playground.guild_id).is_none() {
        connect_to_channel_from_ctx(ctx).await?;
    }

    let do_search = !url.starts_with("http");
    let answer = ctx.say("searching...").await?;

    if let Some(handler_lock) = manager.get(playground.guild_id) {
        let mut handler = handler_lock.lock().await;

        let src = if do_search {
            YoutubeDl::new_search(data.http.clone(), url)
        } else {
            YoutubeDl::new(data.http.clone(), url)
        };
        let metadata: Arc<MetadataObject> = Arc::new(src.clone().aux_metadata().await?.into());
        let input = Track::new_with_data(src.into(), metadata.clone());
        let _ = handler.enqueue(input).await;

        let edit_builder = EditMessage::new().content("Added").embed(metadata.to_embed());

        answer.into_message().await?.edit(ctx, edit_builder).await?;
    } else {
        answer
            .into_message()
            .await?
            .edit(
                ctx,
                EditMessage::new().content("Not in a voice channel to play in."),
            )
            .await?;
    }

    Ok(())
}

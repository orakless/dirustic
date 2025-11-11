use std::io::{Error, ErrorKind};
use std::sync::Arc;
use poise::CreateReply;
use poise::builtins::paginate;
use reqwest::Client;
use serenity::all::EditMessage;
use serenity::Error::Other;
use songbird::Songbird;
use songbird::input::Compose;
use songbird::input::YoutubeDl;
use songbird::tracks::Track;
use crate::{Context, Data};
use crate::StdError;
use crate::types::metadata_queue::{MetadataObject, ToEmbed, ToEmbedPageContent};
use crate::types::playground::Playground;
use crate::types::playlist_parser::{get_items_from_playlist, is_playlist};
use crate::utils::{connect_to_channel_from_ctx, extract_from_ctx};

pub async fn create_track(client: Client, url: String, do_search: bool) -> Result<(Track, Arc<MetadataObject>), StdError> {
    let src = if do_search {
        YoutubeDl::new_search(client, url)
    } else {
        YoutubeDl::new(client, url)
    };
    let metadata: Arc<MetadataObject> = Arc::new(src.clone().aux_metadata().await?.into());
    let input = Track::new_with_data(src.into(), metadata.clone());

    Ok((input, metadata))
}

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn play(ctx: Context<'_>, url: String) -> Result<(), StdError> {
    let (playground, data, manager): (Playground, &Data, &Arc<Songbird>) = extract_from_ctx(ctx);


    if manager.get(playground.guild_id).is_none() {
        connect_to_channel_from_ctx(ctx).await?;
    }

    let do_search = !url.starts_with("http");
    let answer = ctx.say("Searching...").await?;

    if let Some(handler_lock) = manager.get(playground.guild_id) {
        let mut handler = handler_lock.lock().await;

        let (input, metadata) = create_track(data.http.clone(), url, do_search).await?;
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

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn playlist(ctx: Context<'_>, url: String) -> Result<(), StdError> {
    let (playground, data, manager): (Playground, &Data, &Arc<Songbird>) = extract_from_ctx(ctx);

    if manager.get(playground.guild_id).is_none() {
        connect_to_channel_from_ctx(ctx).await?;
    }

    let answer = ctx.say("Getting URLs...").await?;

    let mut metadatas: Vec<Arc<MetadataObject>> = Vec::new();
    let mut inputs: Vec<Track> = Vec::new();

    if is_playlist(&url) {
        let res_urls = get_items_from_playlist(&url).await;
        answer.edit(ctx, CreateReply::default().content("Converting URLs into tracks...")).await?;
        match res_urls {
            Ok(urls) => {
                for url in urls {
                    let (input, metadata) = create_track(data.http.clone(), url, false).await?;
                    metadatas.push(metadata);
                    inputs.push(input);
                }

            },
            Err(_) => { ctx.say("error").await?; }
        }
    } else {
        ctx.say("Not a playlist").await?;
        return Ok(());
    }

    let mut fetch_count: u64 = 0;
    let inputs_len = inputs.len();
    for input in inputs {
        let data = if let Some(handler_lock) = manager.get(playground.guild_id) {
            let mut handler = handler_lock.lock().await;
            let track = handler.enqueue(input).await;
            track.data::<MetadataObject>()
        } else { return Err(Error::new(ErrorKind::Other, Other("Could not get handler_lock")).into()) };
        fetch_count+=1;
        answer.edit(ctx, CreateReply::default()
            .content(format!("Adding [{}](<{}>) ({}/{})", data.title(), data.source_url(), fetch_count, inputs_len))
            .embed(data.to_embed())
        ).await?;
    }

    answer.edit(ctx, CreateReply::default()
        .content("Fetched")).await?;

    let embeds = metadatas.to_paged_embed();
    let embeds_str = embeds.iter().map(|x| &**x).collect::<Vec<_>>();

    paginate(ctx, &embeds_str).await?;

    Ok(())
}
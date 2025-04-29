mod commands;
mod types;

use ::serenity::{all::GatewayIntents, prelude::TypeMapKey};
use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use songbird::events::{Event, EventContext, EventHandler};
use std::{env, sync::Arc};

use reqwest::Client as HttpClient;

struct Data {
    http: HttpClient,
    songbird: Arc<songbird::Songbird>,
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

struct Handler;
struct TrackErrorNotifier;

#[serenity::async_trait]
impl serenity::EventHandler for Handler {
    async fn ready(&self, _: serenity::Context, ready: serenity::Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[serenity::async_trait]
impl EventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                println!(
                    "Track {:?} encountered an error: {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        }

        None
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN in the environment");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = serenity::GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let manager = songbird::Songbird::serenity();
    let manager_clone = Arc::clone(&manager);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::voice::connect(),
                commands::voice::leave(),
                commands::voice::play(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                case_insensitive_commands: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    http: HttpClient::new(),
                    songbird: manager_clone,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .voice_manager_arc(manager)
        .event_handler(Handler)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}

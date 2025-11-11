mod commands;
mod types;
mod utils;
mod dirustic_error;

use ::serenity::all::GatewayIntents;
use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use songbird::events::{Event, EventContext, EventHandler};
use std::{env, sync::Arc};
use reqwest::Client as HttpClient;

struct Data {
    http: HttpClient,
    songbird: Arc<songbird::Songbird>,
} // User data, which is stored and accessible in all command invocations
type StdError = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, StdError>;

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
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let manager = songbird::Songbird::serenity();
    let manager_clone = Arc::clone(&manager);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::channel_management::join(),
                commands::channel_management::leave(),
                commands::voice::play(),
                commands::voice::playlist(),
                commands::queue_management::skip(),
                commands::queue_management::queue(),
                commands::queue_management::remove(),
                commands::queue_management::clear(),
                commands::current::seek(),
                commands::current::now_playing(),
                commands::current::pause(),
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

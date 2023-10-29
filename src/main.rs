#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used)]
#![allow(clippy::cast_possible_wrap)]

mod commands;
mod database;
mod event_handler;
mod log;
mod plotting;
mod time_period;

use commands::{logs, ping};
use dotenvy::dotenv;
use poise::serenity_prelude::GatewayIntents;
use tracing::{error, info};

use crate::database::App;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, App, Error>;

/// How many seconds between each log.
///
/// Eg: a resolution of 30 would count the amount of messages in spans of 30 seconds.
const RESOLUTION: i64 = 30;

#[tokio::main]
async fn main() -> Result<(), poise::serenity_prelude::Error> {
	dotenv().ok();
	tracing_subscriber::fmt().with_test_writer().init();

	let app = App::new().await.expect("Failed to initialise app state");

	let options = poise::FrameworkOptions {
		commands: vec![ping(), logs()],
		command_check: Some(|ctx| {
			Box::pin(async move { Ok(ctx.author().id.0 == 366_491_742_679_072_768) })
		}),
		event_handler: |ctx, event, framework, data| {
			Box::pin(event_handler::event_handler(ctx, event, framework, data))
		},
		pre_command: |ctx| {
			Box::pin(async move {
				info!("Executing command {}...", ctx.command().qualified_name);
			})
		},
		post_command: |ctx| {
			Box::pin(async move {
				info!("Executed command {}!", ctx.command().qualified_name);
			})
		},
		on_error: |err| {
			Box::pin(async move {
				error!("Error while executing command: {}", err.to_string());
			})
		},
		..Default::default()
	};

	let framework = poise::Framework::builder()
		.options(options)
		.token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
		.intents(GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT)
		.setup(|ctx, ready, framework| {
			Box::pin(async move {
				info!("Logged in as {}", ready.user.name);
				poise::builtins::register_globally(ctx, &framework.options().commands).await?;
				Ok(app)
			})
		});

	framework.run().await
}

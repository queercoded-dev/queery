use poise::command;
use poise::serenity_prelude::Timestamp;

use crate::{Context, Error};

#[command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
	let before_timestamp = Timestamp::now().timestamp_millis();
	let reply = ctx.say("Measuring latency!").await?;
	let after_timestamp = Timestamp::now().timestamp_millis();

	let latency = ctx.ping().await.as_millis();

	reply
		.edit(ctx, |reply| {
			reply.content(format!(
				"Pong!\nDiscord Latency: {}ms\nBot Latency: {}ms",
				latency,
				after_timestamp - before_timestamp
			))
		})
		.await?;

	Ok(())
}

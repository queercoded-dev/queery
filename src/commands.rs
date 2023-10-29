use poise::{command, serenity_prelude::AttachmentType};
use poise::serenity_prelude::{Timestamp, Channel};

use crate::plotting::create_log_graph;
use crate::time_period::TimePeriod;
use crate::{Context, Error};

/// Test connection speed.
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

/// Fetch the logs for the current channel.
#[command(slash_command)]
pub async fn logs(ctx: Context<'_>, time_period: TimePeriod, channel: Option<Channel>) -> Result<(), Error> {
	let channel_id = channel.map_or_else(|| ctx.channel_id(), |channel| channel.id());

	let logs = ctx
		.data()
		.fetch_logs(channel_id.0 as i64, i64::MIN, i64::MAX)
		.await?;

	if logs.is_empty() {
		ctx.say("No logs found for this period").await?;
		return Ok(());
	}

	let channel_name = channel_id.name(ctx.cache()).await.unwrap_or_else(|| "Current Channel".to_string());

	let end_timestamp = Timestamp::now().timestamp();
	let start_timestamp = time_period.relative_timestamp_from(end_timestamp);

	let graph = create_log_graph(logs, &channel_name, start_timestamp, end_timestamp)?;

	ctx.send(|reply| {
		reply.attachment(AttachmentType::Bytes { data: graph.into(), filename: "graph.png".to_string() })
	}).await?;

	Ok(())
}

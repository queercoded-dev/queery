use poise::command;
use poise::serenity_prelude::{AttachmentType, Channel, Timestamp};
use tracing::{error, info};

use crate::log::ChangeResolution;
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
#[command(slash_command, required_permissions="MANAGE_CHANNELS")]
pub async fn logs(
	ctx: Context<'_>,
	#[description="How far back to search."]
	time_period: TimePeriod,
	#[description="The channel to fetch the logs for, defaults to the current channel."]
	channel: Option<Channel>,
) -> Result<(), Error> {
	let channel_id = channel.map_or_else(|| ctx.channel_id(), |channel| channel.id());

	let now_timestamp = Timestamp::now();
	let search_start_timestamp = time_period.relative_timestamp_from(now_timestamp.timestamp());

	let logs = ctx
		.data()
		.fetch_logs_between(channel_id.0 as i64, search_start_timestamp, now_timestamp.timestamp())
		.await?;

	// Combign logs together to make displaying on graph easier
	// Todo: This has not been fully tested yet
	let logs = match time_period {
		TimePeriod::Hour => logs,
		TimePeriod::HalfDay => logs.change_resolution(360),
		TimePeriod::Day => logs.change_resolution(720),
		TimePeriod::Week => logs.change_resolution(2520),
	};

	if logs.is_empty() {
		ctx.say("No logs found for this period").await?;
		return Ok(());
	}

	let channel_name = channel_id
		.name(ctx.cache())
		.await
		.unwrap_or_else(|| "Current Channel".to_string());

	let graph = match create_log_graph(logs, &channel_name, search_start_timestamp, now_timestamp.timestamp(), time_period) {
		Ok(graph) => graph,
		Err(err) => {
			error!("Failed to generate log graph: {}", err);
			return Err(err)
		}
	};

	ctx.send(|reply| {
		reply.attachment(AttachmentType::Bytes {
			data:     graph.into(),
			filename: "graph.png".to_string(),
		})
	})
	.await?;


	// Time it took for command to be run.
	let finished_timestamp = Timestamp::now().timestamp_millis() - now_timestamp.timestamp_millis();

	info!("Command took {}ms", finished_timestamp);

	Ok(())
}

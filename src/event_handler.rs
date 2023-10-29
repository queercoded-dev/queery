use poise::serenity_prelude::Message;
use poise::{serenity_prelude as serenity, Event};
use tracing::{error, info};

use crate::database::App;
use crate::{Error, RESOLUTION};

/// Runs for each event recieved in the discord server.
pub async fn event_handler(
	_ctx: &serenity::Context,
	event: &Event<'_>,
	_framework: poise::FrameworkContext<'_, App, Error>,
	data: &App,
) -> Result<(), Error> {
	match event {
		Event::Ready { .. } => {
			info!("Event handler active");
		}
		Event::Message { new_message } => {
			// Doesnt detect itself
			if new_message.author.id == 848_902_037_957_115_916 {
				return Ok(());
			}

			log(data, new_message).await;
		}
		_ => {}
	}
	Ok(())
}

/// Logs the message recieved for a message event in `event_handler`.
async fn log(data: &App, message: &Message) {
	let timestamp = message.timestamp.timestamp();
	#[allow(clippy::cast_possible_wrap)]
	let channel_id = *message.channel_id.as_u64() as i64;

	let normalized_timestamp = timestamp - timestamp % RESOLUTION;

	info!("Logging message");

	let result = if let Some(log) = data.fetch_log(channel_id, normalized_timestamp).await {
		info!("Recent log found, updating");
		data.update_log(log.id, log.count + 1).await
	} else {
		info!("Could not find recent log, creating new");
		data.new_log(normalized_timestamp, channel_id).await
	};

	if let Err(error) = result {
		error!("Log failed: {error}");
	}
}

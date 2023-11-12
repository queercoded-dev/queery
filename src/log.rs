use sqlx::FromRow;
use uuid::Uuid;

/// Represents the data accumulated from `RESOLUTION` seconds of time
/// within a channel.
#[derive(Debug, Clone, Copy, FromRow)]
pub struct Log {
	/// ID used within database.
	pub id:         Uuid,
	/// Discord channel ID for log.
	pub channel_id: i64,
	/// Amount of messages sent during pas `RESOLUTION` seconds
	pub count:      i32,
	/// Time of log period.
	/// Must be a multiple of `RESOLUTION`
	pub time:       i64,
}

pub trait ChangeResolution {
	fn change_resolution(self, new_resolution: i64) -> Vec<Log>;
}

impl ChangeResolution for Vec<Log> {
	fn change_resolution(self, new_resolution: i64) -> Vec<Log> {
		if self.is_empty() {
			return self;
		}

		let mut updated_logs = Self::new();

		let mut current_log = Log {
			count: 0,
			..self[0]
		};

		let mut next_timestamp = current_log.time + new_resolution;

		for log in self {
			if log.time < next_timestamp {
				current_log.count += log.count;
			} else {
				updated_logs.push(current_log);
				next_timestamp = log.time + new_resolution;
				current_log = log;
			}
		}

		updated_logs.push(current_log);

		updated_logs
	}
}

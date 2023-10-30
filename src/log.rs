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

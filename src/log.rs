use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, FromRow)]
pub struct Log {
	pub id:         Uuid,
	pub channel_id: i64,
	pub count:      i32,
	pub time:       i64,
}

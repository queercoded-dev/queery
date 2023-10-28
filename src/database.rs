use sqlx::{postgres::PgPoolOptions, Pool, Postgres, migrate::MigrateDatabase, query_as, query};
use tracing::info;
use uuid::Uuid;

use crate::{Error, log::Log};

/// Core data within bot, containing a
/// connection to its corresponding database
pub struct App {
	pool: Pool<Postgres>,
}

impl App {
	/// Create a postgres connection, creating database if nessessary.
	///
	/// # Errors
	/// This function requires that the environment variable `DATABASE_URL`
	/// is set to a url to a postgres database.
	#[allow(clippy::cognitive_complexity)]
	pub async fn new() -> Result<Self, Error> {
		let db_url = std::env::var("DATABASE_URL")?;

		if !Postgres::database_exists(&db_url).await.unwrap_or(false) {
			info!("Database `queery` does not exist. Creating database");
			Postgres::create_database(&db_url).await?;
			info!("Database created");
		}

        info!("Attempting to connect to database");

		let pool = PgPoolOptions::new()
			.max_connections(5)
			.connect(&db_url)
			.await?;

        info!("Connected to database");

		Ok(Self { pool })
	}

	/// Creates a new log for a channel at the given timstamp
	///
	/// # Assumptions
	/// This function assumes that `timestamp` is a UNIX timestamp
	pub async fn new_log(&self, timestamp: i64, channel_id: i64) -> Result<(), Error> {
		query!(r#"
				INSERT INTO logs (id, channel_id, count, time)
				VALUES ($1, $2 , 1, $3)
			"#,
			Uuid::new_v4(),
			channel_id,
			timestamp
		).execute(&self.pool).await?;

		info!("New log created");

		Ok(())
	}

	/// Updates a given log with a new count.
	///
	/// # Errors
	/// This functions assumes that there is an entry with `log_id`
	pub async fn update_log(&self, log_id: Uuid, new_count: i32) -> Result<(), Error> {
		query!(r#"
				UPDATE logs
				SET count = $1
				WHERE id = $2;
				"#,
				new_count,
				log_id
		).execute(&self.pool).await?;

		info!("Log {log_id} updated");

		Ok(())
	}

	/// Fetches a log for a channel at `timestamp`.
	///
	/// # Assumptions
	/// This function assumes that `timestamp` is a factor of `RESOLUTION`.
	/// If it is not, this function will never return Some.
	///
	/// `timestamp` is expected to be a UNIX timestamp
	pub async fn fetch_log(&self, channel_id: i64, timestamp: i64) -> Option<Log> {
		let log = query_as! (Log,
			r#"SELECT *
			   FROM logs
			   WHERE channel_id = $1
			   AND time = $2"#,
			   channel_id,
			   timestamp
		).fetch_one(&self.pool).await.ok();

		info!("Recent log fetched");

		log
	}

	/// Fetch all of the logs between two UNIX timestamps.
	pub async fn fetch_logs(&self, channel_id: i64, lower_time_bound: i64, upper_time_bound: i64) -> Result<Vec<Log>, Error> {
		#[allow(clippy::cast_possible_wrap)]
		let logs: Vec<Log> = query_as! (Log,
			r#"SELECT *
			   FROM logs
			   WHERE channel_id = $1
			   AND time BETWEEN $2 AND $3"#,
			   channel_id,
			   lower_time_bound,
			   upper_time_bound
		).fetch_all(&self.pool).await?;

		info!("Logs between `{lower_time_bound}` and `{upper_time_bound}` fetched");

		Ok(logs)
	}
}

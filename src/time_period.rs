/// Represent a period of time.
#[derive(Debug, Clone, Copy, poise::ChoiceParameter)]
pub enum TimePeriod {
	Hour,
	#[name = "Half Day"]
	HalfDay,
	Day,
	Week,
}

impl TimePeriod {
	const HOUR: i64 = 3600;
	const HALFDAY: i64 = Self::HOUR * 6;
	const DAY: i64 = Self::HOUR * 12;
	const WEEK: i64 = Self::DAY * 7;

	/// Gets the timestamp that is the cases before the `timestamp`
	///
	/// # Example
	///
	/// ```
	/// let now = Timestamp::now().timestamp(); // Eg 1698569658
	/// let period = TimePeriod::Hour;
	/// let an_hour_ago = 1698566058 // 3600 less than now
	///
	/// assert!(period.relative_timestamp_from(now), an_hour_ago)
	/// ```
	pub const fn relative_timestamp_from(self, timestamp: i64) -> i64 {
		match self {
			Self::Hour => timestamp - Self::HOUR,
			Self::HalfDay => timestamp - Self::HALFDAY,
			Self::Day => timestamp - Self::DAY,
			Self::Week => timestamp - Self::WEEK,
		}
	}
}

#[derive(Debug, Clone, Copy, poise::ChoiceParameter)]
pub enum TimePeriod {
	Hour,
	#[name = "Half Day"]
	HalfDay,
	Day,
}

impl TimePeriod {
	const HOUR: i64 = 3600;
	const HALFDAY: i64 = Self::HOUR * 6;
	const DAY: i64 = Self::HOUR * 12;
	/// Gets the timestamp that is the cases before the `timestamp`
	///
	/// # Example
	///
	/// ```
	/// let now = Timestamp::now().timestamp(); // Eg 1698569658
	/// let period = TimePeriod::Hour;
	/// let an_hour_ago = 1698566058
	///
	/// assert!(period.relative_timestamp_from(now), an_hour_ago)
	/// ```
	pub const fn relative_timestamp_from(self, timestamp: i64) -> i64 {
		match self {
			Self::Hour => timestamp - Self::HOUR,
			Self::HalfDay => timestamp - Self::HALFDAY,
			Self::Day => timestamp - Self::DAY,
		}
	}
}

use chrono::{DateTime, Utc};

// https://stackoverflow.com/questions/25413201/how-do-i-implement-a-trait-i-dont-own-for-a-type-i-dont-own
#[derive(Debug)]
pub struct UtcDateTime(pub DateTime<Utc>);

impl Into<UtcDateTime> for String {
	fn into(self) -> UtcDateTime {
		let datetime_utc = self.parse().unwrap_or_default();

		return UtcDateTime(datetime_utc);
	}
}

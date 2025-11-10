use chrono::{TimeZone, Utc};
use serde::{Deserialize, Serialize};

/// Tournament round enum. Use this when interacting with round values in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Round {
	#[serde(rename = "ro16")]
	Ro16,
	#[serde(rename = "qf")]
	Qf,
	#[serde(rename = "sf")]
	Sf,
	#[serde(rename = "f")]
	F,
	#[serde(rename = "gf")]
	Gf,
}

impl Round {
	/// Return the canonical short string used throughout the app (matches frontend values).
	pub fn as_str(&self) -> &'static str {
		match self {
			Round::Ro16 => "ro16",
			Round::Qf => "qf",
			Round::Sf => "sf",
			Round::F => "f",
			Round::Gf => "gf",
		}
	}
}

impl std::fmt::Display for Round {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

/// Ordered rounds used by the tournament. The index corresponds to how many
/// drafting periods have opened since the start date.
pub const ROUNDS: [Round; 5] = [
	Round::Ro16,
	Round::Qf,
	Round::Sf,
	Round::F,
	Round::Gf,
];

/// Compute the current round according to UTC time.
pub fn compute_round() -> Round {
	// Start of the first drafting period: 2025-11-10T00:00:00Z (UTC)
	let start = Utc.ymd(2025, 11, 10).and_hms(0, 0, 0);
	let now = Utc::now();

	let seconds_per_week: i64 = 7 * 24 * 60 * 60;
	let secs_elapsed = now.signed_duration_since(start).num_seconds();

	let weeks_elapsed = if secs_elapsed <= 0 {
		0usize
	} else {
		(secs_elapsed / seconds_per_week) as usize
	};

	let index = std::cmp::min(weeks_elapsed, ROUNDS.len() - 1);
	ROUNDS[index]
}
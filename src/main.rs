use std::io;

use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Turnover {
	#[serde(with = "custom_date_format")]
	account_day: NaiveDate,
	#[serde(with = "custom_date_format")]
	value_date: NaiveDate,
	// TODO: Convert to custom struct
	operation: String,
	// TODO: Convert to custom struct
	description: String,
	#[serde(with ="custom_float_format")]
	value: f64,
}

fn main() {
	// TODO: Format csv file
    // Create a CSV parser that reads data from stdin.
    let mut reader = csv::ReaderBuilder::new()
		.has_headers(false)
		.delimiter(b';')
		.from_reader(io::stdin());

	// Get all turnovers
	let turnovers: Vec<Turnover> = reader.deserialize()
		.map(|t| t.unwrap())
		.collect();

	dbg!(turnovers);
}

// Module for deserialization of custom day format (e.g. 24.12.2004)
mod custom_date_format {
    use chrono::NaiveDate;
    use serde::{Deserializer, Deserialize};

	// Custom date format
	const FORMAT: &'static str = "%d.%m.%Y";

	pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;

		// Apply custom date format
		NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

// Module for deserialization of custom (European) float format (e.g.: 24.016,56)
mod custom_float_format {
	use serde::{Deserializer, Deserialize};

	pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
	where D: Deserializer<'de> {
		let s = String::deserialize(deserializer)?;

		// Remove dots and replace commas with dots
		s
			.replace('.', "")
			.replace(',', ".")
			.parse()
			.map_err(serde::de::Error::custom)
	}
}

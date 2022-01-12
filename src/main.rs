use std::io;

use serde::{de, Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
struct Turnover {
	// TODO: Convert to dates
	day: String,
	day_2: String,
	operation: String,
	// TODO: Convert to custom struct
	description: String,
	#[serde(deserialize_with ="deserialize_comma_float")]
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

// Custom deserializer for floats in European float format (e.g.: 24.016,56)
fn deserialize_comma_float<'de, D>(deserializer: D) -> Result<f64, D::Error> where D: Deserializer<'de> {
	let buf = String::deserialize(deserializer)?;

	// Remove dots and replace commas with dots
	buf
		.replace('.', "")
		.replace(',', ".")
		.parse()
		.map_err(de::Error::custom)
}

#[cfg(feature = "crossterm")]
mod crossterm;

use std::{io::BufReader, fs::File};

use anyhow::Context;
use chrono::{NaiveDate};
use clap::Parser;
use serde::Deserialize;

#[cfg(feature = "crossterm")]
use crate::crossterm::run;

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

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
	/// .csv file containing all turnovers of the first account
	file1: std::path::PathBuf,
	/// .csv file containing all turnovers of the second account
	file2: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// === Parameter Handling ===
	let args = Cli::parse();
	
	// Read both files and handle errors
	let first_account_file = File::open(&args.file1)
	.with_context(|| format!("could not read file {:?}", &args.file1))?;
	let first_account_reader = BufReader::new(first_account_file);
	
	let second_account_file = File::open(&args.file2)
	.with_context(|| format!("could not read file {:?}", &args.file2))?;
	let second_account_reader = BufReader::new(second_account_file);
	
	// TODO: Convert file from ANSI to UTF-8 encoding
	// TODO: Preformat csv file
	// TODO: Add (f)ilter
	// TODO: Add (a)nalyze functionality

	run()?;

	// let (tx, rx) = mpsc::channel();
	// let tick_rate = Duration::from_millis(200);
	// thread::spawn(move || {
    //     let mut last_tick = Instant::now();
    //     loop {
    //         let timeout = tick_rate
    //             .checked_sub(last_tick.elapsed())
    //             .unwrap_or_else(|| Duration::from_secs(0));

    //         if event::poll(timeout).expect("poll works") {
    //             if let CEvent::Key(key) = event::read().expect("can read events") {
    //                 tx.send(Event::Input(key)).expect("can send events");
    //             }
    //         }

    //         if last_tick.elapsed() >= tick_rate {
    //             if let Ok(_) = tx.send(Event::Tick) {
    //                 last_tick = Instant::now();
    //             }
    //         }
    //     }
    // });

    // Create a CSV parser that reads data from stdin.
    let mut reader = csv::ReaderBuilder::new()
		.has_headers(false)
		.delimiter(b';')
		.from_reader(first_account_reader);

	// Get all turnovers
	let first_account_turnovers: Vec<Turnover> = reader.deserialize()
		.map(|t| t.unwrap())
		.collect();
	
	let second_account_turnovers: Vec<Turnover> = reader.deserialize()
		.map(|t| t.unwrap())
		.collect();

	//dbg!(first_account_turnovers);
	//dbg!(second_account_turnovers);

	Ok(())
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

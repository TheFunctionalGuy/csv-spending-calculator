mod app;
mod crossterm;
mod custom_formats;
mod ui;

use core::time;
use std::{fs::File, io::BufReader};

use anyhow::Context;
use chrono::NaiveDate;
use clap::Parser;
use encoding_rs::ISO_8859_15;
use encoding_rs_io::DecodeReaderBytesBuilder;
use serde::Deserialize;

use custom_formats::{custom_date_format, custom_float_format};

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
    #[serde(with = "custom_float_format")]
    value: f64,
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// .csv file containing all turnovers of the first account
    file1: std::path::PathBuf,
    /// .csv file containing all turnovers of the second account
    file2: std::path::PathBuf,
    /// Milliseconds between each TUI refresh
    #[clap(default_value_t = 250)]
    tick_rate: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // === Parameter Handling ===
    let args = Cli::parse();

    // Create tick rate
    let tick_rate = time::Duration::from_millis(args.tick_rate);

    // Read both files and handle errors
    let first_account_file = File::open(&args.file1)
        .with_context(|| format!("could not read file {:?}", &args.file1))?;
    let first_account_reader = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(ISO_8859_15))
            .build(first_account_file),
    );

    let second_account_file = File::open(&args.file2)
        .with_context(|| format!("could not read file {:?}", &args.file2))?;
    let second_account_reader = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(ISO_8859_15))
            .build(second_account_file),
    );

    // Create a CSV parser that reads data from stdin.
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_reader(first_account_reader);

    // Get all turnovers
    //let first_account_turnovers: Vec<Turnover> = reader.deserialize().map(|t| t.unwrap()).collect();

    //let second_account_turnovers: Vec<Turnover> =
    //    reader.deserialize().map(|t| t.unwrap()).collect();

    //dbg!(first_account_turnovers);
    //dbg!(second_account_turnovers);

    // TODO: Preformat csv file
    // TODO: Add (f)ilter
    // TODO: Add (a)nalyze functionality

    //run(tick_rate)?;

    Ok(())
}

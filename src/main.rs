use std::process::ExitCode;

use anyhow::{Context, Result};
use clap::Parser;
use evalexpr::{context_map, eval_boolean_with_context};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    condition: String,
}

fn main() -> Result<ExitCode> {
    let args = Args::parse();

    let now = match time::OffsetDateTime::now_local() {
        Ok(datetime) => datetime,
        Err(e) => {
            eprintln!("Failed to get local timestamp: {e}");
            eprintln!("Falling back to UTC.");
            time::OffsetDateTime::now_utc()
        }
    };

    let context = context_map! {
        "year" => now.year() as i64,
        "month" => now.month() as i64,
        "iso_week" => now.iso_week() as i64,
        "day" => now.day() as i64,
        "hour" => now.hour() as i64,
        "minute" => now.minute() as i64,
        "week_day" => now.weekday().number_from_monday() as i64,
    }
    .context("failed to create context map with variables from timestamp")?;

    let evaluation_result = eval_boolean_with_context(&args.condition, &context)
        .context("evaluation of time condition failed")?;

    Ok(if evaluation_result {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

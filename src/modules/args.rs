use anyhow::{anyhow, Result};
use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;

#[derive(Parser)]
#[command(author, version, about, long_about = None, next_line_help = true)]
pub struct Cli {
	/// Address to check the weather
	pub address: Option<String>,

	/// [e.g.: -f w,d]
	#[arg(long, short, use_value_delimiter = true, value_name = "FORECAST,...")]
	pub forecast: Vec<Forecast>,

	/// [e.g.: -u f,12h,in]
	#[arg(long, short, use_value_delimiter = true, value_name = "UNIT,...")]
	pub units: Vec<UnitArg>,

	/// Output language [e.g.: en_US]
	#[arg(short, long, value_parser = has_min_length)]
	pub language: Option<String>,

	/// Save the supplied values as default
	#[arg(short, long, action, group = "config_file_action")]
	pub save: bool,

	/// Wipe wthrr's configuration data
	#[arg(short, long, action, group = "config_file_action")]
	pub reset: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, AsRefStr, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Forecast {
	disable,
	#[value(name = "(w)eek", aliases = ["w", "week"])]
	week,
	#[value(name = "(d)ay", aliases = ["d", "day", "today"])]
	day,
	// #[value(alias = "monday")]
	// mo,
	// #[value(alias = "tuesday")]
	// tu,
	// #[value(alias = "wednesday")]
	// we,
	// #[value(alias = "thursday")]
	// th,
	// #[value(alias = "friday")]
	// fr,
	// #[value(alias = "saturday")]
	// sa,
	// #[value(alias = "sunday")]
	// su,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, AsRefStr, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub enum UnitArg {
	// -- Temperature
	#[value(name = "(c)elsius", aliases = ["c", "celsius"])]
	Celsius,
	#[value(name = "(f)ahrenheit", aliases = ["f", "fahrenheit"])]
	Fahrenheit,
	// -- Windspeed
	Kmh,
	Mph,
	#[value(name = "(kn)ots", aliases = ["kn", "knots"])]
	// serialize as kn for open-meteo api call
	#[strum(serialize = "kn")]
	Knots,
	Ms,
	#[value(name = "12h", alias = "am_pm")]
	AmPm,
	#[value(name = "24h", alias = "military")]
	Military,
	#[value(name = "%", alias = "probability")]
	Probability,
	Mm,
	#[value(name = "(in)ch", alias = "in")]
	Inch,
}

fn has_min_length(s: &str) -> Result<String> {
	match s.len() < 2 {
		true => Err(anyhow!("\n  The language code must be at least two characters long.")),
		_ => Ok(s.to_string()),
	}
}

// use clap::{Args, Parser, Subcommand, ValueEnum};
use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
	/// Address to check the weather for
	pub address: Option<String>,

	/// Include the weather forecast
	#[arg(long, short, next_line_help = false, use_value_delimiter = true)]
	pub forecast: Vec<Forecast>,

	/// Units for temperature and/or speed
	#[arg(long, short, next_line_help = false, use_value_delimiter = true)]
	pub units: Vec<ArgUnits>,

	/// Output language
	#[arg(short, long, global = true)]
	pub language: Option<String>,

	/// Toggle greeting message
	#[arg(short, long, action)]
	pub greeting: bool,

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
	#[value(name = "(w)eek", aliases = ["w", "week"])]
	week,
	#[value(name = "to(d)ay", aliases = ["d", "day", "today"])]
	day,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, AsRefStr, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub enum ArgUnits {
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
	Mm,
	Inch,
}

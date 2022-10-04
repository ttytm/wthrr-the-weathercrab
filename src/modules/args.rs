use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
	/// Address to check the weather for
	pub address: Option<String>,

	/// Include the forecast for one week
	#[arg(short, long, action)]
	pub forecast: bool,

	/// Units for temperature and/or speed
	#[arg(long, short, next_line_help = false, use_value_delimiter = true, num_args(..=2))]
	pub units: Vec<ArgUnits>,

	/// Output language
	#[arg(short, long)]
	pub language: Option<String>,

	/// Toggle greeting message
	#[arg(short, long, action)]
	pub greeting: bool,

	/// Save the supplied values as default
	#[arg(short, long, action, group = "config_file_action", global = true)]
	pub save: bool,

	/// Wipe wthrr's configuration data
	#[arg(short, long, action, group = "config_file_action", global = true)]
	pub reset: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, ValueEnum, AsRefStr, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub enum ArgUnits {
	None,
	// -- Temperature
	#[value(name = "(c)elsius", aliases = ["c", "celsius"])]
	Celsius,
	#[value(name = "(f)ahrenheit", aliases = ["f", "fahrenheit"])]
	Fahrenheit,
	// -- Windspeed
	Kmh,
	Mph,
	#[value(name = "(kn)ots", aliases = ["kn", "knots"])]
	#[strum(serialize = "kn")]
	Knots,
	Ms,
}

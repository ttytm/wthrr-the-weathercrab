use clap::{Args, Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
	/// Address to check the weather for
	pub address: Option<String>,

	#[command(subcommand)]
	pub commands: Option<Commands>,

	/// Units for temperature and/or speed
	#[arg(long, short, next_line_help = false, use_value_delimiter = true)]
	pub units: Vec<ArgUnits>,

	/// Output language
	#[arg(short, long, global = true)]
	pub language: Option<String>,

	/// Toggle greeting message
	#[arg(short, long, action, global = true)]
	pub greeting: bool,

	/// Save the supplied values as default
	#[arg(short, long, action, group = "config_file_action", global = true)]
	pub save: bool,

	/// Wipe wthrr's configuration data
	#[arg(short, long, action, group = "config_file_action", global = true)]
	pub reset: bool,
}

#[derive(Subcommand)]
pub enum Commands {
	/// Include the weather forecast
	#[clap(short_flag = 'f')]
	Forecast(Forecast),
}

#[derive(Debug, Args)]
pub struct Forecast {
	/// Show the seven day forecast
	#[arg(short, value_parser, action)]
	pub week: bool,
	/// Show the forecast for the day
	#[arg(short, value_parser, action)]
	pub day: bool,
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
	#[strum(serialize = "kn")]
	Knots,
	Ms,
}

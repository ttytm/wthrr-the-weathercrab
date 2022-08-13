use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
	/// Address to check the weather for
	#[clap(value_parser)]
	pub address: Option<String>,

	/// Unit of measurement ['c' (°Celsius) | 'f' (°Fahrenheit)]
	#[clap(short, long, value_parser, default_value_t = String::from("c"))]
	pub unit: String,

	/// Include the forecast for one week
	#[clap(short, long, value_parser, action)]
	pub forecast: bool,

	/// Save the supplied values as default
	#[clap(short, long, value_parser, action, groups = &["config changes"])]
	pub save_config: bool,

	/// Wipe wthrr's configuration data
	#[clap(short, long, value_parser, action, groups = &["config changes"])]
	pub reset_config: bool,
}

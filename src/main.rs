use anyhow::Result;
use clap::Parser;

use modules::*;
mod modules;

use args::Args;
use config::Config;
use location::Geolocation;
use weather::Weather;

#[tokio::main]
async fn main() -> Result<()> {
	let args = Args::parse();
	let config: Config = confy::lib::load("weathercrab", "wthrr")?;

	if args.reset_config {
		Config::reset()?;
		return Ok(());
	}

	run(&args, config).await?;

	Ok(())
}

pub async fn run(args: &Args, config: Config) -> Result<()> {
	println!("  🦀  Hey friend. I'm glad you are asking.");

	let params = params::get(args, &config).await?;

	let loc = Geolocation::search(params.address.as_ref().unwrap()).await?;
	let (lat, lon) = (loc[0].lat.parse::<f64>().unwrap(), loc[0].lon.parse::<f64>().unwrap());

	let weather = Weather::get(lat, lon, params.unit.as_ref().unwrap()).await?;

	display::render(&weather, loc[0].display_name.to_string(), args.forecast)?;

	Config::handle_next(args, params, config)?;

	Ok(())
}

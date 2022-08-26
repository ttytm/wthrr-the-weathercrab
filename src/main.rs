use anyhow::Result;
use clap::Parser;

use modules::*;
mod modules;

use {args::Args, config::Config, location::Geolocation, weather::Weather};

pub struct Product {
	weather: Weather,
	address: String,
}

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

	let loc = Geolocation::search(&params.address).await?;
	let (lat, lon) = (loc[0].lat.parse::<f64>().unwrap(), loc[0].lon.parse::<f64>().unwrap());

	let weather = Weather::get(lat, lon, params.unit).await?;
	let product = Product {
		weather,
		address: loc[0].display_name.to_string(),
	};

	display::render(&product, args.forecast)?;

	Config::handle_next(args, config, product)?;

	Ok(())
}

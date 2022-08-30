use anyhow::Result;
use clap::Parser;

use modules::*;
mod modules;

use {args::Args, config::Config, location::Geolocation, params::Params, weather::Weather};

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

	greeting(&config)?;

	let params = params::get(&args, &config).await?;
	let product = run(params).await?;
	display::render(&product, args.forecast)?;

	Config::handle_next(&args, config, product)?;

	Ok(())
}

pub async fn run(params: Params) -> Result<Product> {
	let loc = Geolocation::search(&params.address).await?;
	let (lat, lon) = (loc[0].lat.parse::<f64>().unwrap(), loc[0].lon.parse::<f64>().unwrap());

	let product = Product {
		weather: Weather::get(lat, lon, params.unit).await?,
		address: loc[0].display_name.to_string(),
	};

	Ok(product)
}

fn greeting(config: &Config) -> Result<()> {
	if config.greeting == Some(false) {
		return Ok(());
	}

	println!("  🦀  Hey friend. I'm glad you are asking.");

	Ok(())
}

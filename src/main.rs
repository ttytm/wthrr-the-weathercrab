use anyhow::Result;
use clap::Parser;

use modules::config::Config;
use modules::*;
use modules::{args::Cli, display::Product, location::Geolocation, params::Params, weather::Weather};

mod modules;

#[tokio::main]
async fn main() -> Result<()> {
	let args = Cli::parse();
	let config: Config = confy::load("weathercrab", "wthrr")?;
	let params = Params::get(&args, config.clone()).await?;

	let product = run(&params).await?;
	product
		.render(&params.forecast, &params.units, &params.gui, &params.language)
		.await?;

	config.handle_next(args, params).await?;

	Ok(())
}

pub async fn run(params: &Params) -> Result<Product> {
	let loc = Geolocation::search(&params.address, &params.language).await?;
	let (lat, lon) = (loc[0].lat.parse::<f64>().unwrap(), loc[0].lon.parse::<f64>().unwrap());

	let address = loc[0].display_name.to_string();
	let weather = Weather::get(lat, lon, &params.units).await?;

	Ok(Product { address, weather })
}

mod modules;

use anyhow::Result;
use clap::Parser;

use modules::{args::Cli, display::product::Product, location::Geolocation, params::Params, weather::Weather};

#[tokio::main]
async fn main() -> Result<()> {
	let args = Cli::parse();
	let config = Params::get_config_file();
	let params = config.clone().merge(&args).await?;

	let product = run(&params).await?;
	product
		.render(&params.forecast, &params.units, &params.gui, &params.language)
		.await?;

	params.handle_next(args, config).await?;

	Ok(())
}

pub async fn run(params: &Params) -> Result<Product> {
	let loc = Geolocation::search(&params.address, &params.language).await?;
	let (lat, lon) = (loc.lat.parse::<f64>().unwrap(), loc.lon.parse::<f64>().unwrap());

	let address = loc.display_name.to_string();
	let weather = Weather::get(lat, lon, &params.units).await?;

	Ok(Product { address, weather })
}

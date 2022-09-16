use anyhow::Result;
use clap::Parser;

use modules::*;
use modules::{args::Args, display::Product, location::Geolocation, params::Params, weather::Weather};

mod modules;

#[tokio::main]
async fn main() -> Result<()> {
	let args = Args::parse();
	let config = confy::load("weathercrab", "wthrr")?;
	let params = Params::get(&args, &config).await?;

	greeting::handle_greeting(params.greeting, &params.language).await?;

	let product = run(&params).await?;

	product.render(args.forecast, &params.language).await?;

	config.handle_next(args, params).await?;

	Ok(())
}

pub async fn run(params: &Params) -> Result<Product> {
	let loc = Geolocation::search(&params.address, &params.language).await?;
	let (lat, lon) = (loc[0].lat.parse::<f64>().unwrap(), loc[0].lon.parse::<f64>().unwrap());

	let address = loc[0].display_name.to_string();
	let weather = Weather::get(lat, lon, &params.temp_unit).await?;

	Ok(Product { address, weather })
}

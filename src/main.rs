mod modules;

use anyhow::Result;
use clap::Parser;

use modules::{
	args::Cli, config::Config, display::product::Product, location::Location, params::Params, weather::Weather,
};

#[tokio::main]
async fn main() -> Result<()> {
	let args = Cli::parse();
	let config = Config::get();
	let params = Params::merge(&config, &args).await?;

	run(&params).await?.render(&params).await?;
	params.handle_next(args, config).await?;

	Ok(())
}

pub async fn run(params: &Params) -> Result<Product> {
	let loc = Location::get(&params.config.address, &params.config.language).await?;
	let weather = Weather::get(loc.lat, loc.lon, &params.config.units).await?;
	let historical_weather = if !params.historical_weather.is_empty() {
		Some(Weather::get_dates(&params.historical_weather, loc.lat, loc.lon, &params.config.units).await?)
	} else {
		None
	};

	Ok(Product {
		address: loc.name.to_string(),
		weather,
		historical_weather,
	})
}

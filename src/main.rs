#![warn(
	clippy::semicolon_if_nothing_returned,
	clippy::wildcard_imports,
	clippy::cloned_instead_of_copied,
	clippy::single_match_else,
	clippy::match_wildcard_for_single_variants,
	clippy::match_bool,
	clippy::if_not_else
)]

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
	let historical_weather = if params.historical_weather.is_empty() {
		None
	} else {
		Some(Weather::get_dates(&params.historical_weather, loc.lat, loc.lon, &params.config.units).await?)
	};

	Ok(Product {
		address: loc.name.to_string(),
		weather,
		historical_weather,
	})
}

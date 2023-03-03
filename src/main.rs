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

	let product = run(&params).await?;
	product
		.render(
			&params.config.forecast,
			&params.config.units,
			&params.config.gui,
			&params.config.language,
			&params.texts.weather,
		)
		.await?;

	params.handle_next(args, config).await?;

	Ok(())
}

pub async fn run(params: &Params) -> Result<Product> {
	let loc = Location::get(&params.config.address, &params.config.language).await?;
	let weather = Weather::get(loc.lat, loc.lon, &params.config.units).await?;

	Ok(Product {
		address: loc.name.to_string(),
		weather,
	})
}

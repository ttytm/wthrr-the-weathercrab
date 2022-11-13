use anyhow::Result;

use crate::{args::Cli, config::Config};

use self::{forecast::Forecast, units::Units};

use super::args::Commands;

mod address;
pub mod forecast;
mod greeting;
mod language;
pub mod units;

pub struct Params {
	pub address: String,
	pub units: Units,
	pub greeting: bool,
	pub language: String,
	pub forecast: Forecast,
}

impl Params {
	pub async fn get(args: &Cli, config: &Config) -> Result<Self> {
		let language = language::get(
			args.language.as_deref().unwrap_or_default(),
			config.language.as_deref().unwrap_or_default(),
		)?;

		let forecast = forecast::get(
			match &args.commands {
				Some(Commands::Forecast(args_forecast)) => Some(Forecast {
					day: Some(args_forecast.day),
					week: Some(args_forecast.week),
				}),
				_ => None,
			},
			config.forecast,
		)?;

		if args.reset {
			Config::reset(&language).await?;
			std::process::exit(1);
		}

		let address = address::get(
			args.address.as_deref().unwrap_or_default(),
			config.address.as_deref().unwrap_or_default(),
			&language,
		)
		.await?;

		let units = units::get(&args.units, &config.units.unwrap_or_default())?;

		let greeting = greeting::get(args.greeting, config.greeting)?;

		Ok(Params {
			address,
			units,
			language,
			greeting,
			forecast,
		})
	}
}

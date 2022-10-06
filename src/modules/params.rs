use anyhow::Result;

use crate::{
	args::{Cli, Commands, Forecast},
	config::Config,
};

use self::unit::Units;

mod address;
mod greeting;
mod language;
pub mod unit;

pub struct Params {
	pub address: String,
	pub units: Units,
	pub greeting: bool,
	pub language: String,
	pub forecast: Option<Forecast>,
}

impl Params {
	pub async fn get(args: &Cli, config: &Config) -> Result<Self> {
		let language = language::get(
			args.language.as_deref().unwrap_or_default(),
			config.language.as_deref().unwrap_or_default(),
		)?;

		let forecast = if let Some(Commands::Forecast(forecast)) = &args.commands {
			Some(Forecast {
				week: forecast.week,
				day: forecast.day,
			})
		} else {
			None
		};

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

		let units = unit::get(&args.units, config.units.as_deref().unwrap_or_default())?;

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

use anyhow::Result;

use crate::{
	args::{Cli, Forecast},
	config::{Config, Gui},
};

use self::units::Units;

mod address;
pub mod forecast;
mod language;
pub mod units;

pub struct Params {
	pub address: String,
	pub units: Units,
	pub language: String,
	pub forecast: Vec<Forecast>,
	pub gui: Gui,
}

impl Params {
	pub async fn get(args: &Cli) -> Result<Self> {
		let config: Config = confy::load("weathercrab", "wthrr")?;

		let language = language::get(
			args.language.as_deref().unwrap_or_default(),
			config.language.as_deref().unwrap_or_default(),
		)?;

		let forecast = forecast::get(&args.forecast, config.forecast)?;

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

		let gui = config.gui.unwrap_or_default();

		Ok(Params {
			address,
			units,
			language,
			forecast,
			gui,
		})
	}
}

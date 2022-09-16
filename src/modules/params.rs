use anyhow::Result;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString};

use crate::{args::Args, config::Config};

mod address;
mod greeting;
mod language;
mod unit;

pub struct Params {
	pub address: String,
	pub temp_unit: TempUnit,
	pub greeting: bool,
	pub language: String,
}

#[derive(Display, EnumString, Serialize, Deserialize, Debug, PartialEq, Clone, AsRefStr)]
pub enum TempUnit {
	#[strum(serialize = "celsius", serialize = "c")]
	Celsius,
	#[strum(serialize = "fahrenheit", serialize = "f")]
	Fahrenheit,
}

impl Default for TempUnit {
	fn default() -> Self {
		Self::Celsius
	}
}

impl Params {
	pub async fn get(args: &Args, config: &Config) -> Result<Self> {
		let language = language::get(
			args.language.as_deref().unwrap_or_default(),
			config.language.as_deref().unwrap_or_default(),
		)?;

		if args.reset_config {
			Config::reset(&language).await?;
			std::process::exit(1);
		}

		let address = address::get(
			args.address.as_deref().unwrap_or_default(),
			config.address.as_deref().unwrap_or_default(),
			&language,
		)
		.await?;

		let temp_unit = unit::get(
			args.unit.as_deref().unwrap_or_default(),
			config.unit.as_deref().unwrap_or_default(),
		)?;

		let greeting = greeting::get(args.greeting, config.greeting)?;

		Ok(Params {
			address,
			temp_unit,
			language,
			greeting,
		})
	}
}

use anyhow::Result;
use optional_struct::*;
use serde::{Deserialize, Serialize};

use crate::modules::{
	args::{Cli, Forecast},
	translation::translate,
};

use self::{
	gui::{ConfigFileGui, Gui},
	units::{ConfigFileUnits, Units},
};

mod address;
pub mod gui;
pub mod units;

#[optional_struct(ConfigFile)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Params {
	pub address: String,
	pub language: String,
	pub forecast: Vec<Forecast>,
	#[optional_rename(ConfigFileUnits)]
	pub units: Units,
	#[optional_rename(ConfigFileGui)]
	pub gui: Gui,
}

impl Default for Params {
	fn default() -> Self {
		Self {
			address: "".to_string(),
			forecast: vec![],
			language: "en_US".to_string(),
			units: Units::default(),
			gui: Gui::default(),
		}
	}
}

impl Params {
	pub async fn get(args: &Cli) -> Result<Self> {
		let mut params = Self::get_config_file();

		if let Some(language) = &args.language {
			params.language = language.to_string()
		}

		if args.reset {
			Self::reset(&params.language).await?;
			std::process::exit(1);
		}

		if !args.forecast.is_empty() {
			params.forecast = args.forecast.to_vec()
		}

		if params.gui.greeting {
			let greeting = translate(&params.language, "Hey friend. I'm glad you are asking.").await?;
			println!("  🦀  {greeting}");
		}

		params.address = address::get(
			args.address.as_deref().unwrap_or_default(),
			&params.address,
			&params.language,
		)
		.await?;

		params.units = Units::get(&args.units, &params.units);

		Ok(params)
	}
}

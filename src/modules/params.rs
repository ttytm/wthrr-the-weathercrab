use anyhow::Result;
use optional_struct::*;
use serde::{Deserialize, Serialize};

use crate::modules::args::{Cli, Forecast};

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
	pub async fn merge(mut self, args: &Cli) -> Result<Self> {
		if let Some(language) = &args.language {
			self.language = language.to_string()
		}

		if args.reset {
			Self::reset(&self.language).await?;
			std::process::exit(1);
		}

		if !args.forecast.is_empty() {
			self.forecast = args.forecast.to_vec()
		}

		self.units = Units::get(&args.units, &self.units);

		self.resolve_address(args.address.as_deref().unwrap_or_default())
			.await?;

		Ok(self)
	}
}

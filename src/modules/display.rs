use anyhow::Result;
use colored::Color::BrightBlack;
use regex::Regex;

use crate::modules::{
	args::Forecast as ForecastParams,
	params::{
		gui::{ColorOption, Gui},
		units::Units,
	},
	weather::Weather,
};

use self::{current::Current, forecast::Forecast};

pub mod border;
pub mod graph;
pub mod greeting;
pub mod hourly;

mod current;
mod forecast;
mod utils;
mod weathercode;
mod wind;

pub struct Product {
	pub address: String,
	pub weather: Weather,
}

pub const MIN_WIDTH: usize = 34;

impl Product {
	pub async fn render(&self, forecast: &[ForecastParams], units: &Units, gui: &Gui, lang: &str) -> Result<()> {
		if !forecast.is_empty() {
			Forecast::render(self, forecast, units, gui, lang).await?;
		} else {
			Current::render(self, false, units, gui, lang).await?;
		}

		// Disclaimer
		println!(
			" {}",
			"Weather data by Open-Meteo.com\n".color_option(BrightBlack, &gui.color)
		);

		Ok(())
	}

	pub fn trunc_address(mut address: String, max_width: usize) -> Result<String> {
		let address_len = address.chars().count();

		address = if address_len > max_width {
			// For most locations with overly long addresses, the results seem to be better if
			// truncated between the first and second comma instead the penultimate and last comma.
			// let last_comma = title.matches(',').count();
			let prep_re = format!("^((?:[^,]*,){{{}}})[^,]*,(.*)", 1);
			let re = Regex::new(&prep_re).unwrap();

			re.replace(&address, "$1$2").to_string()
		} else {
			address
		};

		if address_len > max_width {
			address = Self::trunc_address(address, max_width)?;
		}

		Ok(address)
	}
}

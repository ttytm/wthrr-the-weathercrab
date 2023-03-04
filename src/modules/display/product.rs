use anyhow::Result;
use colored::Color::BrightBlack;
use regex::Regex;

use crate::modules::{params::Params, weather::Weather};

use super::{current::Current, forecast::Forecast, gui_config::ColorOption};

pub struct Product {
	pub address: String,
	pub weather: Weather,
}

pub const MIN_WIDTH: usize = 34;

impl Product {
	pub async fn render(&self, params: &Params) -> Result<()> {
		if !params.config.forecast.is_empty() {
			Forecast::render(self, params)?;
		} else {
			// render only current weather without hourly forecast
			Current::render(self, params, false)?;
		}

		// Disclaimer
		println!(
			" {}",
			"Weather data by Open-Meteo.com\n".color_option(BrightBlack, &params.config.gui.color)
		);

		Ok(())
	}

	pub fn trunc_address(mut address: String, max_width: usize) -> String {
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
			address = Self::trunc_address(address, max_width);
		}

		address
	}
}

use anyhow::Result;
use colored::Color::BrightBlack;

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
}

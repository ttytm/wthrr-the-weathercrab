use anyhow::Result;
use colored::Color::BrightBlack;

use crate::modules::{forecast::get_indices, params::Params, weather::Weather};

use super::{current::Current, day::Day, forecast::Forecast, gui_config::ColorOption};

pub struct Product {
	pub address: String,
	pub weather: Weather,
}

pub const MIN_WIDTH: usize = 34;

impl Product {
	pub async fn render(&self, params: &Params) -> Result<()> {
		if params.config.forecast.is_empty() {
			// Today without hours
			Current::render(self, params, false)?;
			return Ok(());
		}

		let forecast_indices = get_indices(&params.config.forecast);

		if forecast_indices.contains(&0) && forecast_indices.contains(&7) {
			// Today with hours & weekly overview
			Forecast::render(self, params, Some(Current::render(self, params, true)?))?;
		} else if forecast_indices.contains(&7) {
			// Weekly overview
			Forecast::render(self, params, None)?;
		} else if forecast_indices.contains(&0) {
			// Today with hours
			Current::render(self, params, true)?;
		};

		for i in forecast_indices {
			// Weekdays
			if i < 7 && i > 0 {
				Day::render(self, params, i)?;
			}
		}

		// Disclaimer
		println!(
			"{}",
			"Weather data by Open-Meteo.com\n".color_option(BrightBlack, &params.config.gui.color)
		);

		Ok(())
	}
}

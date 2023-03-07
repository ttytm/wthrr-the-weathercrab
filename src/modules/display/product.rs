use anyhow::Result;
use colored::Color::BrightBlack;

use crate::modules::{params::Params, weather::Weather};

use super::{current::Current, day::Day, forecast::Forecast, gui_config::ColorOption};

pub struct Product {
	pub address: String,
	pub weather: Weather,
}

pub const MIN_WIDTH: usize = 34;

impl Product {
	pub async fn render(&self, params: &Params) -> Result<()> {
		let mut has_forecast = false;

		if params.forecast_indices[0] && params.forecast_indices[7] {
			// Today with hours & weekly overview
			Forecast::render(self, params, Some(Current::render(self, params, true)?))?;
			has_forecast = true
		} else if params.forecast_indices[7] {
			// Weekly overview
			Forecast::render(self, params, None)?;
			has_forecast = true;
		} else if params.forecast_indices[0] {
			// Today with hours
			Current::render(self, params, true)?;
			has_forecast = true;
		};

		// Weekdays
		for (i, include_day) in params.forecast_indices.iter().enumerate().skip(1).take(6) {
			if *include_day {
				Day::render(self, params, i)?;
				has_forecast = true;
			}
		}

		// Today without hours
		if !has_forecast {
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

use anyhow::Result;
use chrono::NaiveDate;
use scopeguard::defer;
use std::collections::HashMap;

use crate::modules::{
	forecast::get_forecast_indices,
	params::Params,
	weather::{OptionalWeather, Weather},
};

use super::{current, day, gui_config::ConfigurableColor, historical, week};

#[derive(Debug)]
pub struct Product<'a> {
	pub address: String,
	pub weather: Weather,
	pub historical_weather: Option<HashMap<&'a NaiveDate, OptionalWeather>>,
}

pub const MIN_WIDTH: usize = 34;
pub const MIN_CELL_WIDTH: usize = MIN_WIDTH / 2 - 2;
pub const TOTAL_BORDER_PADDING: usize = 2;

impl Product<'_> {
	pub fn render(&self, params: &Params) -> Result<()> {
		defer! {
			// Disclaimer
			println!(" {}", "Weather data by Open-Meteo.com\n".plain_or_bright_black(&params.config.gui.color))
		}

		if params.config.forecast.is_empty() && params.historical_weather.is_empty() {
			// Current day without hours
			let (lines, _) = current::prep(self, params, false)?;
			Self::print_lines(lines);
			return Ok(());
		}

		for date in &params.historical_weather {
			Self::print_lines(historical::prep(self, params, date)?);
		}

		if params.config.forecast.is_empty() {
			return Ok(());
		}

		let forecast_indices = get_forecast_indices(&params.config.forecast);

		if forecast_indices.contains(&0) && forecast_indices.contains(&7) {
			// Current day with hours & weekly overview
			let (lines, dimensions) = current::prep(self, params, true)?;
			Self::print_lines(lines);
			Self::print_lines(week::prep(self, params, Some(dimensions))?);
		} else if forecast_indices.contains(&0) {
			// Current day with hours
			let (lines, _) = current::prep(self, params, true)?;
			Self::print_lines(lines);
		} else if forecast_indices.contains(&7) {
			// Weekly overview only
			Self::print_lines(week::prep(self, params, None)?);
		};

		for i in forecast_indices {
			// Weekdays
			if i < 7 && i > 0 {
				Self::print_lines(day::prep(self, params, i)?);
			}
		}

		Ok(())
	}

	fn print_lines(lines: Vec<String>) {
		lines.iter().for_each(|line| println!("{line}"));
	}
}

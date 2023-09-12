use anyhow::Result;
use chrono::NaiveDate;
use scopeguard::defer;
use std::collections::HashMap;

use crate::modules::{
	forecast::get_forecast_indices,
	params::Params,
	weather::{OptionalWeather, Weather},
};

use super::{current::Current, day::Day, gui_config::ConfigurableColor, historical::HistoricalWeather, week::Week};

pub struct Product<'a> {
	pub address: String,
	pub weather: Weather,
	pub historical_weather: Option<HashMap<&'a NaiveDate, OptionalWeather>>,
}

pub const MIN_WIDTH: usize = 34;
pub const MIN_CELL_WIDTH: usize = MIN_WIDTH / 2 - 2;

impl Product<'_> {
	pub fn render(&self, params: &Params) -> Result<()> {
		defer! {
			// Disclaimer
			println!(" {}", "Weather data by Open-Meteo.com\n".plain_or_bright_black(&params.config.gui.color))
		}

		if params.config.forecast.is_empty() && params.historical_weather.is_empty() {
			// Current day without hours
			Current::prep(self, params, false)?.render(params);
			return Ok(());
		}

		for date in &params.historical_weather {
			HistoricalWeather::prep(self, params, date)?.render(params);
		}

		if params.config.forecast.is_empty() {
			return Ok(());
		}

		let forecast_indices = get_forecast_indices(&params.config.forecast);

		if forecast_indices.contains(&0) && forecast_indices.contains(&7) {
			// Current day with hours & weekly overview
			Week::prep(self, params)?.render(params, Some(Current::prep(self, params, true)?.render(params)));
		} else if forecast_indices.contains(&0) {
			// Current day with hours
			Current::prep(self, params, true)?.render(params);
		} else if forecast_indices.contains(&7) {
			// Weekly overview only
			Week::prep(self, params)?.render(params, None);
		};

		for i in forecast_indices {
			// Weekdays
			if i < 7 && i > 0 {
				Day::prep(self, params, i)?.render(params);
			}
		}

		Ok(())
	}
}

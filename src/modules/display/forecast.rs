use anyhow::Result;
use chrono::offset::TimeZone;
use chrono::prelude::*;
use colored::Color::BrightBlack;
use serde::{Deserialize, Serialize};

use crate::modules::{
	args::Forecast as ForecastParams,
	localization::{Locales, WeatherLocales},
	params::Params,
};

use super::{
	border::*,
	current::Current,
	gui_config::ColorOption,
	product::{Product, MIN_WIDTH},
	utils::lang_len_diff,
	weathercode::WeatherCode,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Forecast {
	pub days: Vec<ForecastDay>,
	pub width: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForecastDay {
	pub date: String,
	pub weather: String,
	pub interpretation: String,
}

impl Forecast {
	pub fn render(product: &Product, params: &Params) -> Result<()> {
		let (mut include_day, mut include_week) = (false, false);

		for val in &params.config.forecast {
			if ForecastParams::disable == *val {
				Current::render(product, params, false)?;
				return Ok(());
			}
			if ForecastParams::day == *val {
				include_day = true;
			}
			if ForecastParams::week == *val {
				include_week = true;
			}
		}

		// Hourly forecast only
		if include_day && !include_week {
			Current::render(product, params, true)?;
			return Ok(());
		}

		// Weekly forecast - potentially including hourly forecast
		let forecast = Self::prepare(product, &params.config.language, &params.texts.weather)?;
		let (mut width, mut cell_width) = (forecast.width + 10, MIN_WIDTH / 2);
		let (gui, lang) = (&params.config.gui, &params.config.language);

		if include_day {
			let dimensions_current = Current::render(product, params, true)?;
			if dimensions_current.cell_width > cell_width {
				cell_width = dimensions_current.cell_width
			}
			if dimensions_current.width > width {
				width = dimensions_current.width
			}
		}

		// Border Top
		println!(
			"{}",
			&Edge::Top.fmt(width, &gui.border).color_option(BrightBlack, &gui.color)
		);

		let mut chunks = forecast.days.chunks(1).peekable();

		let mut n = 0;
		let date_len = forecast.days[n].date.len();
		while let Some(_) = chunks.next() {
			let forecast_day = format!(
				"{: <cell_width$}{}{: >width$}",
				forecast.days[n].date,
				forecast.days[n].weather,
				forecast.days[n].interpretation,
				width = width
					- if date_len < 11 { 11 } else { date_len }
					- forecast.days[n].weather.len()
					- lang_len_diff(&forecast.days[n].interpretation, lang)
					- if lang.contains("zh") { 1 } else { 0 }
					- if cell_width == MIN_WIDTH / 2 {
						4
					} else {
						4 + cell_width - MIN_WIDTH / 2
					}
			);
			println!(
				"{} {: <width$} {}",
				&Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
				forecast_day,
				&Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color),
				width = width
					- lang_len_diff(&forecast.days[n].interpretation, lang)
					- lang_len_diff(&forecast.days[n].date, lang)
					- 2,
			);
			if chunks.peek().is_some() {
				println!(
					"{}",
					&match &gui.border {
						BorderStyle::double => Separator::Double.fmt(width, &gui.border),
						BorderStyle::solid => Separator::Solid.fmt(width, &gui.border),
						_ => Separator::Dashed.fmt(width, &gui.border),
					}
					.color_option(BrightBlack, &gui.color)
				)
			}

			n += 1;
		}

		// Border Bottom
		println!(
			"{}",
			Edge::Bottom
				.fmt(width, &gui.border)
				.color_option(BrightBlack, &gui.color)
		);

		Ok(())
	}

	fn prepare(product: &Product, lang: &str, t: &WeatherLocales) -> Result<Self> {
		let mut days = Vec::new();
		let mut width: usize = 0;

		for (i, _) in product.weather.daily.time.iter().enumerate() {
			let time = &product.weather.daily.time[i];
			let dt = Utc
				.with_ymd_and_hms(
					time[0..4].parse::<i32>().unwrap_or_default(),
					time[5..7].parse::<u32>().unwrap_or_default(),
					time[8..10].parse::<u32>().unwrap_or_default(),
					0,
					0,
					0,
				)
				.unwrap();

			let date = if !(lang == "en_US" || lang == "en") {
				Locales::localize_date(dt, lang)?
			} else {
				dt.format("%a, %e %b").to_string()
			};

			let weather_code = WeatherCode::resolve(product.weather.daily.weathercode[i], false, &t.weather_code)?;
			let weather = format!(
				"{} {}{}/{}{}",
				weather_code.icon,
				product.weather.daily.temperature_2m_max[i],
				product.weather.daily_units.temperature_2m_max,
				product.weather.daily.temperature_2m_min[i],
				product.weather.daily_units.temperature_2m_min,
			);
			let day_width = format!("{}{}{}", date, weather, weather_code.interpretation).len();
			if day_width > width {
				width = day_width;
			}

			let day: ForecastDay = {
				ForecastDay {
					date: date.to_string(),
					weather,
					interpretation: weather_code.interpretation,
				}
			};

			days.push(day);
		}

		Ok(Forecast { width, days })
	}
}

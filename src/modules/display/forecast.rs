use std::{
	fs::File,
	io::{BufRead, BufReader},
};

use anyhow::Result;
use chrono::offset::TimeZone;
use chrono::prelude::*;
use colored::Color::BrightBlack;
use serde::{Deserialize, Serialize};

use crate::modules::{
	args::Forecast as ForecastParams,
	params::{
		gui::{ColorOption, Gui},
		units::Units,
	},
};

use super::{
	border::*,
	current::Current,
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
	pub async fn render(
		product: &Product,
		forecast_params: &[ForecastParams],
		units: &Units,
		gui: &Gui,
		lang: &str,
	) -> Result<()> {
		let (mut include_day, mut include_week) = (false, false);

		for val in forecast_params {
			if ForecastParams::disable == *val {
				Current::render(product, false, units, gui, lang).await?;
				return Ok(());
			}
			if ForecastParams::day == *val {
				include_day = true;
			}
			if ForecastParams::week == *val {
				include_week = true;
			}
		}

		// hourly forecast only
		if include_day && !include_week {
			Current::render(product, true, units, gui, lang).await?;
			return Ok(());
		}

		// weekly forecast - potentially including hourly forecast
		let forecast = Self::prepare(product, lang).await?;
		let (mut width, mut cell_width) = (forecast.width + 10, MIN_WIDTH / 2);

		if include_day {
			let dimensions_current = Current::render(product, true, units, gui, lang).await?;
			if dimensions_current.cell_width > cell_width {
				cell_width = dimensions_current.cell_width
			}
			if dimensions_current.width > width {
				width = dimensions_current.width
			}
		}

		let (cfg_border, cfg_color) = (gui.border, gui.color);

		// Border Top
		println!(
			"{}",
			&Edge::Top.fmt(width, &cfg_border).color_option(BrightBlack, &cfg_color)
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
					- if cell_width == MIN_WIDTH / 2 {
						4
					} else {
						4 + cell_width - MIN_WIDTH / 2
					}
			);
			println!(
				"{} {: <width$} {}",
				&Border::L.fmt(&cfg_border).color_option(BrightBlack, &cfg_color),
				forecast_day,
				&Border::R.fmt(&cfg_border).color_option(BrightBlack, &cfg_color),
				width = width
					- lang_len_diff(&forecast.days[n].interpretation, lang)
					- lang_len_diff(&forecast.days[n].date, lang)
					- 2,
			);
			if chunks.peek().is_some() {
				println!(
					"{}",
					&match &cfg_border {
						BorderStyle::double => Separator::Double.fmt(width, &cfg_border),
						BorderStyle::solid => Separator::Solid.fmt(width, &cfg_border),
						_ => Separator::Dashed.fmt(width, &cfg_border),
					}
					.color_option(BrightBlack, &cfg_color)
				)
			}

			n += 1;
		}

		// Border Bottom
		println!(
			"{}",
			Edge::Bottom
				.fmt(width, &cfg_border)
				.color_option(BrightBlack, &cfg_color)
		);

		Ok(())
	}

	async fn prepare(product: &Product, lang: &str) -> Result<Self> {
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

			let date = if lang != "en_US" || lang != "en" {
				Self::localize_date(dt, lang)?
			} else {
				dt.format("%a, %e %b").to_string()
			};

			let weather_code = WeatherCode::resolve(&product.weather.daily.weathercode[i], None, lang).await?;
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

	fn localize_date(dt: DateTime<Utc>, lang: &str) -> Result<String> {
		let file = File::open("./locales/pure-rust-locales.txt")?;
		let reader = BufReader::new(file);

		let mut date = String::new();

		for line in reader.lines().skip(1).flatten() {
			let parts: Vec<&str> = line.split('_').collect();
			let short_lang_code = parts[0];

			if short_lang_code == lang {
				date = dt
					.format_localized("%a, %e %b", line.as_str().try_into().unwrap())
					.to_string();
				break;
			}
		}

		if date.is_empty() {
			date = dt.format("%a, %e %b").to_string()
		}

		Ok(date)
	}
}

use anyhow::Result;
use chrono::offset::TimeZone;
use chrono::prelude::*;
use colored::Color::BrightBlack;
use serde::{Deserialize, Serialize};

use crate::{args::Forecast as ForecastParams, config::Gui, params::units::Units};

use super::{
	border::*,
	current::Current,
	utils::{adjust_lang_width, ColorOption},
	weathercode::WeatherCode,
	Product, MIN_WIDTH,
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
		gui_cfg: &Gui,
		lang: &str,
	) -> Result<()> {
		let forecast = Self::prepare(product, lang).await?;
		let mut width = forecast.width + 10;
		let mut cell_width = MIN_WIDTH / 2;

		let (mut include_day, mut include_week) = (false, false);
		for val in forecast_params {
			if ForecastParams::disable == *val {
				Current::render(product, false, units, gui_cfg, lang).await?;
				return Ok(());
			}
			if ForecastParams::day == *val {
				include_day = true;
			}
			if ForecastParams::week == *val {
				include_week = true;
			}
		}

		if include_day {
			let dimensions_current = Current::render(product, true, units, gui_cfg, lang).await?;

			if dimensions_current.cell_width > cell_width {
				cell_width = dimensions_current.cell_width
			}
			if dimensions_current.width > width {
				width = dimensions_current.width
			}
		}

		if !include_week {
			return Ok(());
		}

		let border_cfg = gui_cfg.border.unwrap_or_default();
		let color_cfg = gui_cfg.color.unwrap_or_default();

		// Border Top
		println!(
			"{}",
			&Edge::Top.fmt(width, &border_cfg).color_option(BrightBlack, &color_cfg)
		);

		let mut chunks = forecast.days.chunks(1).peekable();

		let mut n = 0;
		while let Some(_) = chunks.next() {
			let forecast_day = format!(
				"{: <cell_width$}{}{: >width$}",
				forecast.days[n].date,
				forecast.days[n].weather,
				forecast.days[n].interpretation,
				width = width
					- forecast.days[n].date.len()
					- forecast.days[n].weather.len()
					- adjust_lang_width(&forecast.days[n].interpretation, lang)
					- if cell_width == MIN_WIDTH / 2 {
						4
					} else {
						4 + cell_width - MIN_WIDTH / 2
					}
			);
			println!(
				"{} {: <width$} {}",
				&Border::L.fmt(&border_cfg).color_option(BrightBlack, &color_cfg),
				forecast_day,
				&Border::R.fmt(&border_cfg).color_option(BrightBlack, &color_cfg),
				width = width - adjust_lang_width(&forecast.days[n].interpretation, lang) - 2,
			);
			if chunks.peek().is_some() {
				println!(
					"{}",
					&match &border_cfg {
						BorderVariant::double => Separator::Double.fmt(width, &border_cfg),
						BorderVariant::solid => Separator::Solid.fmt(width, &border_cfg),
						_ => Separator::Dashed.fmt(width, &border_cfg),
					}
					.color_option(BrightBlack, &color_cfg)
				)
			}

			n += 1;
		}

		// Border Bottom
		println!(
			"{}",
			Edge::Bottom
				.fmt(width, &border_cfg)
				.color_option(BrightBlack, &color_cfg)
		);

		Ok(())
	}

	async fn prepare(product: &Product, lang: &str) -> Result<Self> {
		let mut days = Vec::new();
		let mut width: usize = 0;

		for (i, _) in product.weather.daily.time.iter().enumerate() {
			let time = &product.weather.daily.time[i];
			let date = Utc
				.with_ymd_and_hms(
					time[0..4].parse::<i32>().unwrap_or_default(),
					time[5..7].parse::<u32>().unwrap_or_default(),
					time[8..10].parse::<u32>().unwrap_or_default(),
					0,
					0,
					0,
				)
				.unwrap();

			// let date = date.format("%a, %b %e").to_string();
			let date = &date.to_rfc2822()[..11];

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
}

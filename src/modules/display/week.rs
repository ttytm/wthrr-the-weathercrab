use anyhow::Result;
use chrono::offset::TimeZone;
use chrono::prelude::*;
use colored::Color::BrightBlack;
use serde::{Deserialize, Serialize};

use crate::modules::{localization::Locales, params::Params};

use super::{
	border::{Border, BorderStyle, Edge, Separator},
	current::Dimensions,
	gui_config::ColorOption,
	product::{Product, MIN_CELL_WIDTH},
	utils::lang_len_diff,
	weathercode::WeatherCode,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Week {
	pub days: Vec<ForecastDay>,
	pub width: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForecastDay {
	pub date: String,
	pub weather: String,
	pub interpretation: String,
}

impl Week {
	pub fn render(self, params: &Params, current_dimensions: Option<Dimensions>) {
		let forecast = self;
		let (mut width, mut cell_width) = (forecast.width + 10, MIN_CELL_WIDTH);
		let (gui, lang) = (&params.config.gui, &params.config.language);

		if let Some(dims) = current_dimensions {
			cell_width = std::cmp::max(cell_width, dims.cell_width);
			width = std::cmp::max(width, dims.width);
		}

		// Border Top
		println!("{}", &Edge::Top.fmt(width, &gui.border).color_option(BrightBlack, &gui.color));

		let mut chunks = forecast.days.chunks(1).peekable();

		let mut n = 0;
		let date_len = forecast.days[0].date.chars().count() - lang_len_diff(&forecast.days[0].date, lang);
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
					- if &lang[..2] == "zh" || &lang[..2] == "ja" || &lang[..2] == "ko" {
						2
					} else {
						0
					} - if cell_width == MIN_CELL_WIDTH {
					2
				} else {
					2 + cell_width - MIN_CELL_WIDTH
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
				);
			}

			n += 1;
		}

		// Border Bottom
		println!("{}", Edge::Bottom.fmt(width, &gui.border).color_option(BrightBlack, &gui.color));
	}

	pub fn prep(product: &Product, params: &Params) -> Result<Self> {
		let (lang, t) = (&params.config.language, &params.texts.weather);
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

			let date = if lang == "en_US" || lang == "en" {
				dt.format("%a, %e %b").to_string()
			} else {
				Locales::localize_date(dt, lang)?
			};

			let weather_code = WeatherCode::resolve(product.weather.daily.weathercode[i], false, &t.weather_code)?;
			let weather = format!(
				"{} {:.1}{}/{:.1}{}",
				weather_code.icon,
				product.weather.daily.temperature_2m_max[i],
				product.weather.daily_units.temperature_2m_max,
				product.weather.daily.temperature_2m_min[i],
				product.weather.daily_units.temperature_2m_min,
			);
			let day = format!("{}{}{}", date, weather, weather_code.interpretation);
			let day_width = day.chars().count() + lang_len_diff(&day, lang) + 2;
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

		Ok(Week { days, width })
	}
}

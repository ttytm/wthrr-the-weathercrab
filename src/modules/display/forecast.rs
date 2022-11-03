use anyhow::Result;
use chrono::offset::TimeZone;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use term_painter::{Color::*, ToStyle};

use crate::{args::Forecast as ForecastArgs, params::units::Units};

use super::{
	border::{Border, Separator},
	current::Current,
	utils::adjust_lang_width,
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
	pub async fn render(product: &Product, forecast_args: &ForecastArgs, units: &Units, lang: &str) -> Result<()> {
		let forecast = Self::prepare(product, lang).await?;
		let mut width = forecast.width + 10;
		let mut cell_width = MIN_WIDTH / 2;

		match forecast_args {
			// Only display daily forecast
			ForecastArgs { day: true, week: false } => {
				Current::render(product, true, units, lang).await?;
				return Ok(());
			}
			// Display both forecasts if no or both forecast subcommand flags are added
			ForecastArgs {
				day: false,
				week: false,
			}
			| ForecastArgs { day: true, week: true } => {
				// Align dimensions of daily and weekly forecast
				let dimensions_current = Current::render(product, true, units, lang).await?;

				if dimensions_current.cell_width > cell_width {
					cell_width = dimensions_current.cell_width
				}
				if dimensions_current.width > width {
					width = dimensions_current.width
				}
			}
			_ => {}
		}

		// Border Top
		BrightBlack.with(|| println!("{}{}{} ", Border::TL, Border::T.to_string().repeat(width), Border::TR));

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
				BrightBlack.paint(Border::L),
				forecast_day,
				BrightBlack.paint(Border::R),
				width = width - adjust_lang_width(&forecast.days[n].interpretation, lang) - 2,
			);
			if chunks.peek().is_some() {
				BrightBlack.with(|| println!("{}", Separator::Line.fmt(width)));
			}

			n += 1;
		}

		// Border Bottom
		BrightBlack.with(|| println!("{}{}{}", Border::BL, Border::B.to_string().repeat(width), Border::BR));
		Ok(())
	}

	async fn prepare(product: &Product, lang: &str) -> Result<Self> {
		let mut days = Vec::new();
		let mut width: usize = 0;

		for (i, _) in product.weather.daily.time.iter().enumerate() {
			let time = &product.weather.daily.time[i];
			let date = Utc
				.ymd(
					time[0..4].parse().unwrap_or_default(),
					time[5..7].parse().unwrap_or_default(),
					time[8..10].parse().unwrap_or_default(),
				)
				.and_hms(0, 0, 0);
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

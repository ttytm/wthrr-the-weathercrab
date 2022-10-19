use anyhow::Result;
use chrono::offset::TimeZone;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use term_painter::{Color::*, ToStyle};

use crate::{args::Forecast as ForecastArgs, params::units::Units};

use super::{
	border::{Border, Separator},
	current::Current,
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

		if forecast_args.day && !forecast_args.week {
			Current::render(product, true, units, lang).await?;
			return Ok(());
		}

		// If week flag is not added -> ADD forecast to current days weather instead of displaying it exclusively
		if !forecast_args.week {
			let dimensions_current = Current::render(product, true, units, lang).await?;

			if dimensions_current.cell_width > cell_width {
				cell_width = dimensions_current.cell_width
			}
			if dimensions_current.width > width {
				width = dimensions_current.width
			}
		}

		// Border Top
		BrightBlack.with(|| println!("{}{}{} ", Border::TL, Border::T.to_string().repeat(width), Border::TR));

		let mut chunks = forecast.days.chunks(1).peekable();

		let mut n = 0;
		while let Some(_) = chunks.next() {
			let merge = format!(
				"{: <cell_width$}{}{: >width$}",
				forecast.days[n].date,
				forecast.days[n].weather,
				forecast.days[n].interpretation,
				width = width
					- forecast.days[n].date.len()
					- forecast.days[n].weather.len()
					// TODO: add better calculation to determine width
					- if cell_width == 22 { 9 } else { 4 }
			);
			println!(
				"{} {: <width$} {}",
				BrightBlack.paint(Border::L),
				merge,
				BrightBlack.paint(Border::R),
				width = width - 2,
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
			let merge = format!("{}{}{}", date, weather, weather_code.interpretation);
			if merge.len() > width {
				width = merge.len();
			}

			let day: ForecastDay = {
				ForecastDay {
					date: date.to_string(),
					weather,
					interpretation: weather_code.interpretation,
					// icon: weather_code.icon.unwrap_or_default(),
				}
			};

			days.push(day);
		}

		Ok(Forecast { width, days })
	}
}

use anyhow::Result;
use chrono::{Timelike, Utc};
use colored::{
	Color::{Blue, BrightBlack, Yellow},
	Colorize,
};
use std::fmt::Write as _;

use crate::{
	config::ColorVariant,
	params::units::{Precipitation, Temperature, Time, Units},
	weather::Weather,
};

use super::{
	border::*,
	graph::{Graph, GraphConfig},
	utils::{style_number, ColorOption},
	weathercode::WeatherCode,
};

const DISPLAY_HOURS: [usize; 8] = [1, 3, 6, 9, 12, 15, 18, 21];

pub struct HourlyForecast {
	temperatures: String,
	graph: Graph,
	precipitation: String,
	time_indicator_col: usize,
}

impl HourlyForecast {
	pub fn render(self, width: usize, units: &Units, border_variant: &BorderVariant, color_variant: &ColorVariant) {
		println!(
			"{}",
			&Separator::Blank
				.fmt(width, border_variant)
				.color_option(BrightBlack, color_variant)
		);

		let temperature_unit = match units.temperature {
			Some(Temperature::fahrenheit) => "宅",
			_ => "糖",
		};
		let precipitation_unit = match units.precipitation {
			Some(Precipitation::inch) => "ᵢₙ",
			_ => "ₘₘ",
		};

		println!(
			"{} {: <width$} {}",
			Border::L.fmt(border_variant).color_option(BrightBlack, color_variant),
			"Hourly Forecast".bold(),
			Border::R.fmt(border_variant).color_option(BrightBlack, color_variant),
			width = width - 2
		);

		println!(
			"{}",
			self.prepare_separator(border_variant, width, '╤')
				.color_option(BrightBlack, color_variant),
		);

		println!(
			"{} {: <width$}{}{}",
			Border::L.fmt(border_variant).color_option(BrightBlack, color_variant),
			self.temperatures.color_option(Yellow, color_variant).bold(),
			temperature_unit.color_option(Yellow, color_variant),
			Border::R.fmt(border_variant).color_option(BrightBlack, color_variant),
			width = width - 3
		);
		println!(
			"{}",
			&Separator::Blank
				.fmt(width, border_variant)
				.color_option(BrightBlack, color_variant)
		);

		if self.graph.1.chars().count() > 0 {
			println!(
				"{}{}{}",
				Border::L.fmt(border_variant).color_option(BrightBlack, color_variant),
				self.graph.1.color_option(Yellow, color_variant),
				Border::R.fmt(border_variant).color_option(BrightBlack, color_variant)
			);
		}
		println!(
			"{}{}{}",
			Border::L.fmt(border_variant).color_option(BrightBlack, color_variant),
			self.graph.0.color_option(Yellow, color_variant),
			Border::R.fmt(border_variant).color_option(BrightBlack, color_variant)
		);

		println!(
			"{} {: <width$}{}{}",
			Border::L.fmt(border_variant).color_option(BrightBlack, color_variant),
			self.precipitation.color_option(Blue, color_variant).bold(),
			precipitation_unit.color_option(Blue, color_variant),
			Border::R.fmt(border_variant).color_option(BrightBlack, color_variant),
			width = width - 4
		);

		println!(
			"{}",
			self.prepare_separator(border_variant, width, '╧')
				.color_option(BrightBlack, color_variant),
		);

		let hours = match units.time {
			Some(Time::am_pm) => ["¹²·⁰⁰ₐₘ", "³·⁰⁰ₐₘ", "⁶˙⁰⁰ₐₘ", "⁹˙⁰⁰ₐₘ", "¹²˙⁰⁰ₚₘ", "³˙⁰⁰ₚₘ", "⁶˙⁰⁰ₚₘ", "⁹˙⁰⁰ₚₘ"],
			_ => ["⁰⁰˙⁰⁰", "⁰³˙⁰⁰", "⁰⁶˙⁰⁰", "⁰⁹˙⁰⁰", "¹²˙⁰⁰", "¹⁵˙⁰⁰", "¹⁸˙⁰⁰", "²¹˙⁰⁰"],
		};
		print!(
			"{}",
			Border::L.fmt(border_variant).color_option(BrightBlack, color_variant)
		);
		for hour in hours {
			print!("{: <9}", hour)
		}
		println!(
			"{}",
			Border::R.fmt(border_variant).color_option(BrightBlack, color_variant)
		);
	}

	pub async fn prepare(
		weather: &Weather,
		current_hour: usize,
		night: bool,
		graph_cfg: &GraphConfig,
		lang: &str,
	) -> Result<Self> {
		let temperatures = Self::prepare_temperature(weather, night, lang).await?;
		let precipitation = Self::prepare_precipitation(&weather.hourly.precipitation[..=24])?;
		let graph = Graph::prepare_graph(&weather.hourly.temperature_2m[..=24], graph_cfg)?;
		let time_indicator_col = current_hour * 3 + (Timelike::minute(&Utc::now()) / 20) as usize;

		Ok(HourlyForecast {
			temperatures,
			graph,
			time_indicator_col,
			precipitation,
		})
	}

	async fn prepare_temperature(weather: &Weather, night: bool, lang: &str) -> Result<String> {
		let mut result = String::new();

		for hour in DISPLAY_HOURS {
			let temp = weather.hourly.temperature_2m[hour].round() as i32;
			let temp_sub = style_number(temp, true)?;
			let wmo_code = WeatherCode::resolve(&weather.hourly.weathercode[hour], Some(night), lang).await?;
			let length = if hour == 1 { 1 } else { 8 };
			let _ = write!(result, "{: >length$}{}", temp_sub, wmo_code.icon);
		}

		Ok(result)
	}

	fn prepare_precipitation(precipitation: &[f64]) -> Result<String> {
		let mut result = String::new();

		for hour in DISPLAY_HOURS {
			let prec = precipitation[hour].ceil() as i32;
			let precipitation_sup = style_number(prec, true)?;
			let length = if hour == 1 { 2 } else { 8 };
			let _ = write!(result, "{: >length$} ", precipitation_sup);
		}

		Ok(result)
	}

	fn prepare_separator(&self, border_variant: &BorderVariant, width: usize, time_indicator: char) -> String {
		let mut current_hour = self.time_indicator_col + 3;

		if current_hour > width {
			current_hour -= width
		}

		match border_variant {
			BorderVariant::double => format!(
				"╟{:─>current_hour$}{:─>width$}╢",
				time_indicator,
				"",
				width = width - current_hour
			),
			BorderVariant::solid => format!(
				"┠{:─>current_hour$}{:─>width$}┨",
				time_indicator,
				"",
				width = width - current_hour
			),
			_ => format!(
				"├{:┈>current_hour$}{:┈>width$}┤",
				time_indicator,
				"",
				width = width - current_hour
			),
		}
	}
}

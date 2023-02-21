use anyhow::Result;
use chrono::{Timelike, Utc};
use colored::{
	Color::{Blue, BrightBlack, Yellow},
	Colorize,
};
use std::fmt::Write as _;

use crate::modules::{
	locales::{WeatherCodeLocales, WeatherLocales},
	params::{
		gui::{ColorOption, ColorVariant},
		units::{Precipitation, Temperature, Time, Units},
	},
	weather::{Hourly, Weather},
};

use super::{
	border::*,
	graph::{Graph, GraphOpts},
	utils::style_number,
	weathercode::WeatherCode,
};

const DISPLAY_HOURS: [usize; 8] = [0, 3, 6, 9, 12, 15, 18, 21];

pub struct HourlyForecast {
	temperatures: String,
	precipitation: String,
	graph: Graph,
	time_indicator_col: usize,
}

impl HourlyForecast {
	pub fn render(
		self,
		width: usize,
		units: &Units,
		border_variant: &BorderStyle,
		color_variant: &ColorVariant,
		t: &WeatherLocales,
	) {
		println!(
			"{}",
			&Separator::Blank
				.fmt(width, border_variant)
				.color_option(BrightBlack, color_variant)
		);

		let temperature_unit = match units.temperature {
			Temperature::fahrenheit => "宅",
			_ => "糖",
		};
		let precipitation_unit = match units.precipitation {
			Precipitation::inch => "ᵢₙ",
			_ => "ₘₘ",
		};

		println!(
			"{} {: <width$} {}",
			Border::L.fmt(border_variant).color_option(BrightBlack, color_variant),
			t.hourly_forecast.bold(),
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
			Time::am_pm => ["¹²·⁰⁰ₐₘ", "³·⁰⁰ₐₘ", "⁶˙⁰⁰ₐₘ", "⁹˙⁰⁰ₐₘ", "¹²˙⁰⁰ₚₘ", "³˙⁰⁰ₚₘ", "⁶˙⁰⁰ₚₘ", "⁹˙⁰⁰ₚₘ"],
			_ => ["⁰⁰˙⁰⁰", "⁰³˙⁰⁰", "⁰⁶˙⁰⁰", "⁰⁹˙⁰⁰", "¹²˙⁰⁰", "¹⁵˙⁰⁰", "¹⁸˙⁰⁰", "²¹˙⁰⁰"],
		};
		print!(
			"{}",
			Border::L.fmt(border_variant).color_option(BrightBlack, color_variant)
		);
		for hour in hours {
			print!("{hour: <9}")
		}
		println!(
			"{}",
			Border::R.fmt(border_variant).color_option(BrightBlack, color_variant)
		);
	}

	pub fn prepare(
		weather: &Weather,
		current_hour: usize,
		night: bool,
		graph_opts: &GraphOpts,
		t: &WeatherCodeLocales,
	) -> Result<Self> {
		let Hourly {
			temperature_2m,
			weathercode,
			precipitation,
			..
		} = &weather.hourly;

		// slice 25 items to use the 25th in the last "next_level" of a graph
		// show weather of the next day if it is the last hour of the current day
		let temperatures = if current_hour != 23 {
			&temperature_2m[..=24]
		} else {
			&temperature_2m[25..=49]
		};
		let weather_codes = if current_hour != 23 {
			&weathercode[..=24]
		} else {
			&weathercode[25..=49]
		};
		let precipitation = if current_hour != 23 {
			&precipitation[..=24]
		} else {
			&precipitation[25..=49]
		};

		// add 3 cols to adjust to the multiple chars used to display the current hour below the chart
		let time_indicator_col =
			if current_hour != 23 { (current_hour * 3) + 3 } else { 0 } + (Timelike::minute(&Utc::now()) / 20) as usize;

		Ok(HourlyForecast {
			temperatures: Self::prepare_temperatures(temperatures, weather_codes, night, t)?,
			precipitation: Self::prepare_precipitation(precipitation)?,
			graph: Graph::prepare_graph(temperatures, graph_opts)?,
			time_indicator_col,
		})
	}

	fn prepare_temperatures(
		temperatures: &[f64],
		weather_codes: &[f64],
		night: bool,
		t: &WeatherCodeLocales,
	) -> Result<String> {
		let mut result = String::new();

		for hour in DISPLAY_HOURS {
			let temp = temperatures[hour].round() as i32;
			let temp_sub = style_number(temp, true)?;
			let wmo_code = WeatherCode::resolve(&weather_codes[hour], Some(night), t)?;
			let colspan = if hour == 0 { 2 } else { 8 };
			let _ = write!(result, "{: >colspan$}{}", temp_sub, wmo_code.icon);
		}

		Ok(result)
	}

	fn prepare_precipitation(precipitation: &[f64]) -> Result<String> {
		let mut result = String::new();

		for hour in DISPLAY_HOURS {
			let prec = precipitation[hour].ceil() as i32;
			let precipitation_sup = style_number(prec, true)?;
			let colspan = if hour == 0 { 2 } else { 8 };
			let _ = write!(result, "{precipitation_sup: >colspan$} ");
		}

		Ok(result)
	}

	fn prepare_separator(&self, border_variant: &BorderStyle, width: usize, time_indicator_glyph: char) -> String {
		let time_indicator_col = self.time_indicator_col;

		match border_variant {
			BorderStyle::double => format!(
				"╟{:─>time_indicator_col$}{:─>width$}╢",
				time_indicator_glyph,
				"",
				width = width - time_indicator_col
			),
			BorderStyle::solid => format!(
				"┠{:─>time_indicator_col$}{:─>width$}┨",
				time_indicator_glyph,
				"",
				width = width - time_indicator_col
			),
			_ => format!(
				"├{:┈>time_indicator_col$}{:┈>width$}┤",
				time_indicator_glyph,
				"",
				width = width - time_indicator_col
			),
		}
	}
}

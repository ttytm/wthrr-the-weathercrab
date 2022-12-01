use anyhow::Result;
use chrono::{Timelike, Utc};
use colored::{
	Color::{Blue, BrightBlack, Yellow},
	Colorize,
};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt::Write as _};

use crate::{
	config::ColorVariant,
	params::units::{Precipitation, Temperature, Time, Units},
	weather::{Hourly, Weather},
};

use super::{
	border::*,
	utils::{style_number, ColorOption},
	weathercode::WeatherCode,
};

pub struct Graph {
	temperatures: String,
	graph: String,
	precipitation: String,
	time_indicator_col: usize,
}

#[derive(Default, Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum GraphVariant {
	#[default]
	lines,
	lines_shallow,
	dots,
	dots_double,
	// dots_fill,
}

const DISPLAY_HOURS: [usize; 8] = [0, 3, 6, 9, 12, 15, 18, 21];

impl Graph {
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
		println!(
			"{}{}{}",
			Border::L.fmt(border_variant).color_option(BrightBlack, color_variant),
			self.graph.color_option(Yellow, color_variant),
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
		graph_variant: &GraphVariant,
		lang: &str,
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

		Ok(Graph {
			temperatures: Self::prepare_temperatures(temperatures, weather_codes, night, lang).await?,
			graph: Self::prepare_graph(temperatures, graph_variant)?,
			precipitation: Self::prepare_precipitation(precipitation)?,
			time_indicator_col,
		})
	}

	async fn prepare_temperatures(
		temperatures: &[f64],
		weather_codes: &[f64],
		night: bool,
		lang: &str,
	) -> Result<String> {
		let mut result = String::new();

		for hour in DISPLAY_HOURS {
			let temp = temperatures[hour].round() as i32;
			let temp_sub = style_number(temp, true)?;
			let wmo_code = WeatherCode::resolve(&weather_codes[hour], Some(night), lang).await?;
			let length = if hour == 0 { 1 } else { 8 };
			let _ = write!(result, "{: >length$}{}", temp_sub, wmo_code.icon);
		}

		Ok(result)
	}

	fn prepare_precipitation(precipitation: &[f64]) -> Result<String> {
		let mut result = String::new();

		for hour in DISPLAY_HOURS {
			let prec = precipitation[hour].ceil() as i32;
			let precipitation_sup = style_number(prec, true)?;
			let length = if hour == 0 { 2 } else { 8 };
			let _ = write!(result, "{: >length$} ", precipitation_sup);
		}

		Ok(result)
	}

	fn prepare_graph(temperatures: &[f64], graph_variant: &GraphVariant) -> Result<String> {
		let min_temp = temperatures.iter().fold(f64::INFINITY, |a, &b| a.min(b));
		let max_temp = temperatures.iter().copied().fold(f64::NEG_INFINITY, f64::max);

		let graph_levels = match graph_variant {
			GraphVariant::lines => ['🭻', '🭺', '🭹', '🭸', '🭷', '🭶', '▔'].to_vec(),
			GraphVariant::lines_shallow => ['⎽', '⎼', '⎻', '⎺'].to_vec(),
			GraphVariant::dots => ['⡀', '⠄', '⠂', '⠁'].to_vec(),
			GraphVariant::dots_double => ['⣀', '⠤', '⠒', '⠉'].to_vec(),
			// somthing like this is better suited for a graph that spans more the one line
			// GraphVariant::dots_fill => ['⣀', '⣤', '⣶', '⣿'].to_vec(),
		};
		let level_margin = (max_temp - min_temp) / (graph_levels.len() - 1) as f64;
		let mut last_level = None;
		let mut graph = String::new();

		for (i, temp) in temperatures.iter().enumerate() {
			let current_level = ((temp - min_temp) / level_margin) as usize;

			if let Some(last_level) = last_level {
				match current_level.cmp(&last_level) {
					Ordering::Greater => graph.push(graph_levels[current_level + 1]),
					Ordering::Less => graph.push(graph_levels[current_level - 1]),
					Ordering::Equal => graph.push(graph_levels[current_level]),
				}
			} else {
				graph.push(graph_levels[current_level])
			}

			graph.push(graph_levels[current_level]);

			let next_level = ((temperatures[i + 1] - min_temp) / level_margin) as usize;

			match current_level.cmp(&next_level) {
				Ordering::Greater => graph.push(graph_levels[current_level - 1]),
				Ordering::Less => graph.push(graph_levels[current_level + 1]),
				Ordering::Equal => graph.push(graph_levels[current_level]),
			}

			if i == 23 {
				break;
			}

			last_level = Some(next_level);
		}

		Ok(graph)
	}

	fn prepare_separator(&self, border_variant: &BorderVariant, width: usize, time_indicator_glyph: char) -> String {
		let time_indicator_col = self.time_indicator_col;

		match border_variant {
			BorderVariant::double => format!(
				"╟{:─>time_indicator_col$}{:─>width$}╢",
				time_indicator_glyph,
				"",
				width = width - time_indicator_col
			),
			BorderVariant::solid => format!(
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

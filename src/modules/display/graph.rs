use anyhow::Result;
use colored::{
	Color::{Blue, BrightBlack, Yellow},
	Colorize,
};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt::Write as _};

use crate::{
	config::ColorVariant,
	params::units::{Precipitation, Temperature, Time, Units},
	weather::Weather,
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

const DISPLAY_HOURS: [usize; 8] = [1, 3, 6, 9, 12, 15, 18, 21];

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
			&match border_variant {
				BorderVariant::double => Separator::Double.fmt(width, border_variant),
				BorderVariant::solid => Separator::Solid.fmt(width, border_variant),
				_ => Separator::Dashed.fmt(width, border_variant),
			}
			.color_option(BrightBlack, color_variant)
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
			match border_variant {
				BorderVariant::double => Separator::Double.fmt(width, border_variant),
				BorderVariant::solid => Separator::Solid.fmt(width, border_variant),
				_ => Separator::Dashed.fmt(width, border_variant),
			}
			.color_option(BrightBlack, color_variant)
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

	pub async fn prepare(weather: &Weather, night: bool, graph_variant: &GraphVariant, lang: &str) -> Result<Self> {
		let temperatures = Self::prepare_temperature(weather, night, lang).await?;
		let precipitation = Self::prepare_precipitation(&weather.hourly.precipitation[..=24])?;
		let graph = Self::prepare_graph(&weather.hourly.temperature_2m[..=24], graph_variant)?;

		Ok(Graph { temperatures, graph, precipitation })
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
}

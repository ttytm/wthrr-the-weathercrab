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
	weather::Weather,
};

use super::{
	border::*,
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

struct Graph {
	one: String,
	two: String,
}

struct GraphLvls {
	glyphs: Vec<char>,
	margin: f64,
	last: Option<usize>,
	current: usize,
	next: usize,
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

		if self.graph.two.chars().count() > 0 {
			println!(
				"{}{}{}",
				Border::L.fmt(border_variant).color_option(BrightBlack, color_variant),
				self.graph.two.color_option(Yellow, color_variant),
				Border::R.fmt(border_variant).color_option(BrightBlack, color_variant)
			);
		}
		println!(
			"{}{}{}",
			Border::L.fmt(border_variant).color_option(BrightBlack, color_variant),
			self.graph.one.color_option(Yellow, color_variant),
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
		let temperatures = Self::prepare_temperature(weather, night, lang).await?;
		let precipitation = Self::prepare_precipitation(&weather.hourly.precipitation[..=24])?;
		let graph = Self::prepare_graph(&weather.hourly.temperature_2m[..=24], graph_variant)?;
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

	fn prepare_graph(temperatures: &[f64], graph_variant: &GraphVariant) -> Result<Graph> {
		let min_temp = temperatures.iter().fold(f64::INFINITY, |a, &b| a.min(b));
		let max_temp = temperatures.iter().copied().fold(f64::NEG_INFINITY, f64::max);

		// TODO: use config variable
		let double = false;

		let mut graph_glyphs = match graph_variant {
			GraphVariant::lines => vec!['▁', '🭻', '🭺', '🭹', '🭸', '🭷', '🭶', '▔'],
			GraphVariant::lines_shallow => vec!['⎽', '⎼', '⎻', '⎺'],
			GraphVariant::dots => vec!['⡀', '⠄', '⠂', '⠁'],
			GraphVariant::dots_double => vec!['⣀', '⠤', '⠒', '⠉'],
			// somthing like this is better suited for a graph that spans more the one line
			// GraphVariant::dots_fill => ['⣀', '⣤', '⣶', '⣿'].to_vec(),
		};

		if double {
			graph_glyphs.append(&mut graph_glyphs.to_vec());
		}
		println!("{:?}", graph_glyphs);

		let lvl_margin = (max_temp - min_temp) / (graph_glyphs.len() - 1) as f64;

		let mut graph_lvls = GraphLvls {
			glyphs: graph_glyphs,
			last: None,
			current: 0,
			next: 0,
			margin: lvl_margin,
		};

		let mut graph = Graph {
			one: String::new(),
			two: String::new(),
		};

		// create graph - calculate and push three characters per iteration to graph strings
		for (i, temp) in temperatures.iter().enumerate() {
			graph_lvls.current = ((temp - min_temp) / graph_lvls.margin) as usize;
			graph_lvls.next = ((temperatures[i + 1] - min_temp) / graph_lvls.margin) as usize;

			// char 1/3 - compare with last level
			if let Some(last_lvl) = graph_lvls.last {
				match Some(last_lvl.cmp(&graph_lvls.current)) {
					Some(o) if o == Ordering::Less => graph.one.push(graph_lvls.glyphs[graph_lvls.get_idx(o)]),
					Some(o) if o == Ordering::Equal => graph.one.push(graph_lvls.glyphs[graph_lvls.get_idx(o)]),
					Some(o) if o == Ordering::Greater => graph.one.push(graph_lvls.glyphs[graph_lvls.get_idx(o)]),
					_ => {}
				}
			} else {
				// first iteration - without a last_lvl
				graph.one.push(graph_lvls.glyphs[graph_lvls.get_idx(Ordering::Equal)])
			}

			// char 2/3
			graph.one.push(graph_lvls.glyphs[graph_lvls.get_idx(Ordering::Equal)]);

			// char 3/3 - compare with next level
			match Some(graph_lvls.next.cmp(&graph_lvls.current)) {
				Some(o) if o == Ordering::Less => graph.one.push(graph_lvls.glyphs[graph_lvls.get_idx(o)]),
				Some(o) if o == Ordering::Equal => graph.one.push(graph_lvls.glyphs[graph_lvls.get_idx(o)]),
				Some(o) if o == Ordering::Greater => graph.one.push(graph_lvls.glyphs[graph_lvls.get_idx(o)]),
				_ => {}
			}

			if i == 23 {
				break;
			}

			graph_lvls.last = Some(graph_lvls.next);
		}

		Ok(graph)
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

impl GraphLvls {
	fn get_idx(&self, pending_comparison: Ordering) -> usize {
		match pending_comparison {
			Ordering::Less => {
				if self.next < self.current - 1 && self.current > 1 {
					self.current - 2
				} else {
					self.current - 1
				}
			}
			Ordering::Equal => {
				if self.next > self.current + 1 && self.current < self.glyphs.len() {
					self.current + 1
				} else if self.next < self.current && self.current > 0 {
					self.current - 1
				} else {
					self.current
				}
			}
			Ordering::Greater => {
				if self.next > self.current + 1 && self.current + 1 < self.glyphs.len() {
					self.current + 2
				} else {
					self.current + 1
				}
			}
		}
	}
}

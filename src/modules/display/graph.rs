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

pub struct Graph {
	temperatures: String,
	graph: GraphS,
	precipitation: String,
	time_indicator_col: usize,
}

struct GraphS {
	top: String,
	bot: String,
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
			self.graph.top.color_option(Yellow, color_variant),
			Border::R.fmt(border_variant).color_option(BrightBlack, color_variant)
		);
		println!(
			"{}{}{}",
			Border::L.fmt(border_variant).color_option(BrightBlack, color_variant),
			self.graph.bot.color_option(Yellow, color_variant),
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

		Ok(Graph {
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

	fn prepare_graph(temperatures: &[f64], graph_variant: &GraphVariant) -> Result<GraphS> {
		let min_temp = temperatures.iter().fold(f64::INFINITY, |a, &b| a.min(b));
		let max_temp = temperatures.iter().copied().fold(f64::NEG_INFINITY, f64::max);

		let graph_lvls = match graph_variant {
			GraphVariant::lines => ['▁', '🭻', '🭺', '🭹', '🭸', '🭷', '🭶', '▔'].to_vec(),
			GraphVariant::lines_shallow => ['⎽', '⎼', '⎻', '⎺'].to_vec(),
			GraphVariant::dots => ['⡀', '⠄', '⠂', '⠁'].to_vec(),
			GraphVariant::dots_double => ['⣀', '⠤', '⠒', '⠉'].to_vec(),
			// somthing like this is better suited for a graph that spans more the one line
			// GraphVariant::dots_fill => ['⣀', '⣤', '⣶', '⣿'].to_vec(),
		};

		let graph_lvl_idx_sum = graph_lvls.len() - 1;
		let lvl_margin = (max_temp - min_temp) / graph_lvl_idx_sum as f64;
		let mut last_lvl: Option<usize> = None;
		let mut graph = String::new();
		let mut graph_two = String::new();

		// create graph - calculate and push three characters per iteration to graph strings
		for (i, temp) in temperatures.iter().enumerate() {
			let curr_lvl = ((temp - min_temp) / lvl_margin) as usize;
			let next_lvl = ((temperatures[i + 1] - min_temp) / lvl_margin) as usize;
			/* let correction = if next_lvl > curr_lvl + 1 || next_lvl < curr_lvl - 1 {
				Some(1)
			} else {
				None
			}; */

			// earlier compare with next level
			if let Some(last_lvl) = last_lvl {
				match last_lvl.cmp(&curr_lvl) {
					/* Ordering::Less => graph.push(graph_lvls[last_lvl - 1]),
					Ordering::Equal => graph.push(graph_lvls[last_lvl]),
					Ordering::Greater => graph.push(graph_lvls[last_lvl]), */
					// ---
					Ordering::Less => {
						let idx = if curr_lvl > 0 && next_lvl < curr_lvl - 1 {
							curr_lvl - 2
						} else {
							curr_lvl - 1
						};
						graph.push(graph_lvls[idx])
					}
					Ordering::Equal => {
						let idx = if curr_lvl < graph_lvl_idx_sum && next_lvl > curr_lvl + 1 {
							curr_lvl + 1
						} else if curr_lvl > 0 && next_lvl < curr_lvl - 1 {
							curr_lvl - 1
						} else {
							curr_lvl
						};
						graph.push(graph_lvls[idx])
					}
					Ordering::Greater => {
						let idx = if curr_lvl < graph_lvl_idx_sum && next_lvl > curr_lvl + 1 {
							curr_lvl + 2
						} else {
							curr_lvl + 1
						};
						graph.push(graph_lvls[idx])
					} // ---
					  /* Ordering::Less => {
						  graph.push(graph_lvls[adjust_index(last_lvl - 1, next_lvl, graph_lvl_idx_sum, Adjustment::Up)])
					  }
					  Ordering::Equal => graph.push(graph_lvls[adjust_index(last_lvl, next_lvl, graph_lvl_idx_sum)]),
					  Ordering::Greater => graph.push(graph_lvls[adjust_index(last_lvl, next_lvl, graph_lvl_idx_sum)]), */
				}
			} else {
				// first iteration without a last_lvl
				let idx = if curr_lvl < graph_lvl_idx_sum && next_lvl > curr_lvl + 1 {
					curr_lvl + 1
				} else if curr_lvl > 0 && next_lvl < curr_lvl - 1 {
					curr_lvl - 1
				} else {
					curr_lvl
				};
				graph.push(graph_lvls[idx])
			}

			/* fn adjust_index(curr_index: usize, next_level: usize, graph_index_sum: usize, adj: Adjustment) -> usize {
				/* if nex_lvl > !(curr_index + 1) ||if curr_index > 0 && next_level < curr_index - 1 {
				} */
				match adj {
					Adjustment::Up => {
						if curr_index < graph_index_sum && next_level > curr_index + 1 {
							return curr_index + 1;
						} else {
							return curr_index;
						}
					}
					Adjustment::Down => {
						if curr_index > 0 && next_level < curr_index - 1 {
							return curr_index - 1;
						} else {
							return curr_index;
						}
					}
				}
				/* if curr_index < graph_index_sum && next_level > curr_index + 1 {
					curr_index + 1
				} else if curr_index > 0 && next_level < curr_index - 1 {
					curr_index - 1
				} else {
					curr_index
				} */
			} */

			// TODO: compare with next level if difference is > 1 then current_level + 1
			// graph.push(graph_lvls[curr_lvl]);
			// ---
			let idx = if curr_lvl < graph_lvl_idx_sum && next_lvl > curr_lvl + 1 {
				curr_lvl + 1
			} else if curr_lvl > 0 && next_lvl < curr_lvl - 1 {
				curr_lvl - 1
			} else {
				curr_lvl
			};
			graph.push(graph_lvls[idx]);
			// ---
			// graph.push(graph_lvls[adjust_index(curr_lvl, next_lvl, graph_lvl_idx_sum)]);

			// TODO: if difference is > 1 then current_level + 2
			match next_lvl.cmp(&curr_lvl) {
				/* Ordering::Less => graph.push(graph_lvls[curr_lvl - 1]),
				Ordering::Equal => graph.push(graph_lvls[curr_lvl]),
				Ordering::Greater => graph.push(graph_lvls[curr_lvl + 1]),*/
				// ---
				Ordering::Less => {
					let idx = if curr_lvl > 0 && next_lvl < curr_lvl - 1 {
						curr_lvl - 2
					} else {
						curr_lvl - 1
					};
					graph.push(graph_lvls[idx])
				}
				Ordering::Equal => {
					let idx = if curr_lvl < graph_lvl_idx_sum && next_lvl > curr_lvl + 1 {
						curr_lvl + 1
					} else if curr_lvl > 0 && next_lvl < curr_lvl - 1 {
						curr_lvl - 1
					} else {
						curr_lvl
					};
					graph.push(graph_lvls[idx])
				}
				Ordering::Greater => {
					let idx = if curr_lvl < graph_lvl_idx_sum && next_lvl > curr_lvl + 1 {
						curr_lvl + 2
					} else {
						curr_lvl + 1
					};
					graph.push(graph_lvls[idx])
				} // ---
				  // ---
				  /* Ordering::Less => graph.push(graph_lvls[adjust_index(curr_lvl - 1, next_lvl, graph_lvl_idx_sum)]),
				  Ordering::Equal => graph.push(graph_lvls[adjust_index(curr_lvl, next_lvl, graph_lvl_idx_sum)]),
				  Ordering::Greater => graph.push(graph_lvls[adjust_index(curr_lvl + 1, next_lvl, graph_lvl_idx_sum)]), */
			}

			if i == 23 {
				break;
			}

			// print!("{}-{}-{} ", last_lvl.unwrap_or_default(), curr_lvl, next_lvl);

			last_lvl = Some(next_lvl);
		}

		/* for (i, temp) in temperatures.iter().enumerate() {
			let current_level = ((temp - min_temp) / level_margin) as usize;

			if let Some(last_level) = last_level {
				match last_level.cmp(&current_level) {
					Ordering::Greater => graph.push(graph_levels[last_level + 1]),
					Ordering::Less => graph.push(graph_levels[last_level - 1]),
					Ordering::Equal => graph.push(graph_levels[last_level]),
				}
			} else {
				graph.push(graph_levels[current_level])
			}

			graph.push(graph_levels[current_level]);

			let next_level = ((temperatures[i + 1] - min_temp) / level_margin) as usize;

			match next_level.cmp(&current_level) {
				Ordering::Greater => graph.push(graph_levels[current_level + 1]),
				Ordering::Less => graph.push(graph_levels[current_level - 1]),
				Ordering::Equal => graph.push(graph_levels[current_level]),
			}

			if i == 23 {
				break;
			}

			last_level = Some(next_level);
		} */

		Ok(GraphS { top: graph_two, bot: graph })
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

enum Adjustment {
	Up,
	Down,
	Both,
}

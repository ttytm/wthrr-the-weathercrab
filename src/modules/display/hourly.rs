use anyhow::Result;
use std::{cmp::Ordering, fmt::Write as _};
use term_painter::{
	Attr::Bold,
	Color::{Blue, BrightBlack, Yellow},
	ToStyle,
};

use crate::{
	params::units::{Precipitation, Temperature, Time, Units},
	weather::Weather,
};

use super::{
	border::{Border, Separator},
	weathercode::WeatherCode,
};

pub struct HourlyForecast {
	temperatures: String,
	graph: String,
	precipitation: String,
}

const DISPLAY_HOURS: [usize; 8] = [1, 3, 6, 9, 12, 15, 18, 21];

impl HourlyForecast {
	pub fn render(self, width: usize, units: &Units) {
		BrightBlack.with(|| println!("{}", Separator::Blank.fmt(width)));

		let temperature_unit = match units.temperature {
			Some(Temperature::fahrenheit) => "宅",
			_ => "糖",
		};
		let precipitation_unit = match units.precipitation {
			Some(Precipitation::inch) => "ⁱⁿ",
			_ => "ₘₘ",
		};

		println!(
			"{} {: <width$} {}",
			BrightBlack.paint(Border::L),
			Bold.paint("Hourly Forecast"),
			BrightBlack.paint(Border::R),
			width = width - 2
		);

		BrightBlack.with(|| println!("{}", Separator::Dotted.fmt(width)));

		Yellow.with(|| {
			println!(
				"{} {: <width$}{}{}",
				BrightBlack.paint(Border::L),
				Bold.paint(self.temperatures),
				temperature_unit,
				BrightBlack.paint(Border::R),
				width = width - 3
			);
			BrightBlack.with(|| println!("{}", Separator::Blank.fmt(width)));
			println!(
				"{}{}{}",
				BrightBlack.paint(Border::L),
				Bold.paint(self.graph),
				BrightBlack.paint(Border::R)
			);

			Blue.with(|| {
				println!(
					"{} {: <width$}{}{}",
					BrightBlack.paint(Border::L),
					Bold.paint(self.precipitation),
					precipitation_unit,
					BrightBlack.paint(Border::R),
					width = width - 4
				)
			});
			BrightBlack.with(|| println!("{}", Separator::Dotted.fmt(width)));
		});

		let hours = match units.time {
			Some(Time::am_pm) => [
				"¹²·⁰⁰ₐₘ",
				"³·⁰⁰ₐₘ",
				"⁶˙⁰⁰ₐₘ",
				"⁹˙⁰⁰ₐₘ",
				"¹²˙⁰⁰ₚₘ",
				"³˙⁰⁰ₚₘ",
				"⁶˙⁰⁰ₚₘ",
				"⁹˙⁰⁰ₚₘ",
			],
			_ => ["⁰⁰˙⁰⁰", "⁰³˙⁰⁰", "⁰⁶˙⁰⁰", "⁰⁹˙⁰⁰", "¹²˙⁰⁰", "¹⁵˙⁰⁰", "¹⁸˙⁰⁰", "²¹˙⁰⁰"],
		};
		print!("{}", BrightBlack.paint(Border::L),);
		for hour in hours {
			print!("{: <9}", hour)
		}
		println!("{}", BrightBlack.paint(Border::R));
	}

	pub async fn prepare(weather: &Weather, night: bool, lang: &str) -> Result<Self> {
		let temperatures = Self::prepare_temperature(weather, night, lang).await?;
		let precipitation = Self::prepare_precipitation(&weather.hourly.precipitation[..=24])?;
		let graph = Self::prepare_graph(&weather.hourly.temperature_2m[..=24])?;

		Ok(HourlyForecast {
			temperatures,
			graph,
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

	fn prepare_graph(temperatures: &[f64]) -> Result<String> {
		let min_temp = temperatures.iter().fold(f64::INFINITY, |a, &b| a.min(b));
		let max_temp = temperatures.iter().copied().fold(f64::NEG_INFINITY, f64::max);

		const GRAPH_LEVELS: [char; 7] = ['🭻', '🭺', '🭹', '🭸', '🭷', '🭶', '▔'];
		let level_margin = (max_temp - min_temp) / 6.0;
		let mut last_level = None;
		let mut graph = String::new();

		for (i, temp) in temperatures.iter().enumerate() {
			let current_level = ((temp - min_temp) / level_margin) as usize;

			if let Some(last_level) = last_level {
				match current_level.cmp(&last_level) {
					Ordering::Greater => graph.push(GRAPH_LEVELS[current_level + 1]),
					Ordering::Less => graph.push(GRAPH_LEVELS[current_level - 1]),
					Ordering::Equal => graph.push(GRAPH_LEVELS[current_level]),
				}
			} else {
				graph.push(GRAPH_LEVELS[current_level])
			}

			graph.push(GRAPH_LEVELS[current_level]);

			let next_level = ((temperatures[i + 1] - min_temp) / level_margin) as usize;

			match current_level.cmp(&next_level) {
				Ordering::Greater => graph.push(GRAPH_LEVELS[current_level - 1]),
				Ordering::Less => graph.push(GRAPH_LEVELS[current_level + 1]),
				Ordering::Equal => graph.push(GRAPH_LEVELS[current_level]),
			}

			if i == 23 {
				break;
			}

			last_level = Some(next_level);
		}

		Ok(graph)
	}
}

fn style_number(mut num: i32, sub: bool) -> Result<String> {
	const SUPERSCRIPT_DIGITS: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
	const SUBSCRIPT_DIGITS: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];

	let mut result = String::new();

	if num == 0 {
		result.push(match sub {
			true => SUBSCRIPT_DIGITS[0],
			_ => SUPERSCRIPT_DIGITS[0],
		});
		return Ok(result);
	}

	if num < 0 {
		num = -num;
		result.push(if sub { '₋' } else { '⁻' });
	}

	let mut started = false;
	let mut power_of_ten = 1_000_000_000;
	for _ in 0..10 {
		let digit = num / power_of_ten;
		num -= digit * power_of_ten;
		power_of_ten /= 10;
		if digit != 0 || started {
			started = true;
			result.push(match sub {
				true => SUBSCRIPT_DIGITS[digit as usize],
				_ => SUPERSCRIPT_DIGITS[digit as usize],
			})
		}
	}

	Ok(result)
}

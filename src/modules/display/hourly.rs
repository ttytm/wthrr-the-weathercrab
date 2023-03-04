use anyhow::Result;
use chrono::{Timelike, Utc};
use colored::{
	Color::{Blue, BrightBlack, Yellow},
	Colorize,
};
use std::fmt::Write as _;

use crate::modules::{
	localization::{WeatherCodeLocales, WeatherLocales},
	units::{Precipitation, Temperature, Time, Units},
	weather::Weather,
};

use super::{
	border::*,
	graph::{Graph, GraphOpts},
	gui_config::{ColorOption, ColorVariant},
	utils::style_number,
	weathercode::WeatherCode,
};

const DISPLAY_HOURS: [usize; 8] = [0, 3, 6, 9, 12, 15, 18, 21];

pub struct HourlyForecast {
	temperatures: String,
	precipitation: String,
	temp_max_min: String,
	precipitation_probability_max: u8,
	graph: Graph,
	time_indicator_col: Option<usize>,
}

impl HourlyForecast {
	pub fn render(
		self,
		width: usize,
		units: &Units,
		border_style: &BorderStyle,
		color_variant: &ColorVariant,
		t: &WeatherLocales,
	) {
		println!(
			"{}",
			&Separator::Blank
				.fmt(width, border_style)
				.color_option(BrightBlack, color_variant)
		);

		let temperature_unit = match units.temperature {
			Temperature::fahrenheit => "",
			_ => "",
		};
		let precipitation_unit = match units.precipitation {
			Precipitation::mm => "ₘₘ",
			Precipitation::inch => "ᵢₙ",
			_ => "󰖎",
		};

		println!(
			"{} {: <width$} {}",
			Border::L.fmt(border_style).color_option(BrightBlack, color_variant),
			t.hourly_forecast.bold(),
			Border::R.fmt(border_style).color_option(BrightBlack, color_variant),
			width = width - 2
		);
		println!(
			"{} {} ❲{}{}❳{: <width$} {}",
			Border::L.fmt(border_style).color_option(BrightBlack, color_variant),
			self.temp_max_min,
			self.precipitation_probability_max,
			"󰖎".bold(),
			"",
			Border::R.fmt(border_style).color_option(BrightBlack, color_variant),
			width = width - 5 - self.temp_max_min.len() - self.precipitation_probability_max.to_string().len()
		);

		match self.time_indicator_col {
			Some(_) => {
				println!(
					"{}",
					self.prepare_separator(border_style, width, '╤')
						.color_option(BrightBlack, color_variant),
				);
			}
			_ => {
				println!(
					"{}",
					Separator::Dashed
						.fmt(width, border_style)
						.color_option(BrightBlack, color_variant)
				);
			}
		}

		println!(
			"{} {: <width$}{} {}",
			Border::L.fmt(border_style).color_option(BrightBlack, color_variant),
			self.temperatures.color_option(Yellow, color_variant).bold(),
			temperature_unit.color_option(Yellow, color_variant).bold(),
			Border::R.fmt(border_style).color_option(BrightBlack, color_variant),
			width = width - 3
		);
		println!(
			"{}",
			&Separator::Blank
				.fmt(width, border_style)
				.color_option(BrightBlack, color_variant)
		);

		if self.graph.1.chars().count() > 0 {
			println!(
				"{}{}{}",
				Border::L.fmt(border_style).color_option(BrightBlack, color_variant),
				self.graph.1.color_option(Yellow, color_variant),
				Border::R.fmt(border_style).color_option(BrightBlack, color_variant)
			);
		}
		println!(
			"{}{}{}",
			Border::L.fmt(border_style).color_option(BrightBlack, color_variant),
			self.graph.0.color_option(Yellow, color_variant),
			Border::R.fmt(border_style).color_option(BrightBlack, color_variant)
		);

		println!(
			"{} {: <width$}{} {}",
			Border::L.fmt(border_style).color_option(BrightBlack, color_variant),
			self.precipitation.color_option(Blue, color_variant).bold(),
			if units.precipitation == Precipitation::probability {
				// to enlarge the water percent icon we use bold as a hack
				precipitation_unit.color_option(Blue, color_variant).bold()
			} else {
				precipitation_unit.color_option(Blue, color_variant)
			},
			Border::R.fmt(border_style).color_option(BrightBlack, color_variant),
			width = width - 3
		);

		match self.time_indicator_col {
			Some(_) => {
				println!(
					"{}",
					self.prepare_separator(border_style, width, '╧')
						.color_option(BrightBlack, color_variant),
				);
			}
			_ => {
				println!(
					"{}",
					Separator::Dashed
						.fmt(width, border_style)
						.color_option(BrightBlack, color_variant)
				);
			}
		}

		let hours = match units.time {
			Time::am_pm => ["¹²·⁰⁰ₐₘ", "³·⁰⁰ₐₘ", "⁶˙⁰⁰ₐₘ", "⁹˙⁰⁰ₐₘ", "¹²˙⁰⁰ₚₘ", "³˙⁰⁰ₚₘ", "⁶˙⁰⁰ₚₘ", "⁹˙⁰⁰ₚₘ"],
			_ => ["⁰⁰˙⁰⁰", "⁰³˙⁰⁰", "⁰⁶˙⁰⁰", "⁰⁹˙⁰⁰", "¹²˙⁰⁰", "¹⁵˙⁰⁰", "¹⁸˙⁰⁰", "²¹˙⁰⁰"],
		};
		print!(
			"{}",
			Border::L.fmt(border_style).color_option(BrightBlack, color_variant)
		);
		for hour in hours {
			print!("{hour: <9}")
		}
		println!(
			"{}",
			Border::R.fmt(border_style).color_option(BrightBlack, color_variant)
		);
	}

	pub fn prepare(
		weather: &Weather,
		current_hour: usize,
		night: bool,
		graph_opts: &GraphOpts,
		units: &Units,
		t: &WeatherCodeLocales,
	) -> Result<Self> {
		let (temperatures, weather_codes, precipitation);

		// The graph splits one hour into three "levels": last, current and next.
		// We slice 25 items to use the 25th in the last "next"-level of a graph.
		// If it's the end of one day we show the weather of the next day
		if current_hour != 23 {
			temperatures = &weather.hourly.temperature_2m[..=24];
			weather_codes = &weather.hourly.weathercode[..=24];
			precipitation = match units.precipitation {
				Precipitation::probability => {
					Self::prepare_precipitation_probability(&weather.hourly.precipitation_probability[..=24])?
				}
				_ => Self::prepare_precipitation(&weather.hourly.precipitation[..=24])?,
			};
		} else {
			temperatures = &weather.hourly.temperature_2m[25..=49];
			weather_codes = &weather.hourly.weathercode[25..=49];
			precipitation = match units.precipitation {
				Precipitation::probability => {
					Self::prepare_precipitation_probability(&weather.hourly.precipitation_probability[..=24])?
				}
				_ => Self::prepare_precipitation(&weather.hourly.precipitation[25..=49])?,
			};
		};

		let time_indicator_col = match graph_opts.time_indicator {
			true => Some(
				// add 3 cols to adjust to the multiple chars used to display the current hour below the chart
				if current_hour != 23 { (current_hour * 3) + 3 } else { 1 }
					+ (Timelike::minute(&Utc::now()) / 20) as usize,
			),
			_ => None,
		};

		let temp_max_min = format!(
			"{}/{}{}",
			weather.daily.temperature_2m_max[0],
			weather.daily.temperature_2m_min[0],
			weather.hourly_units.temperature_2m,
		);

		let precipitation_probability_max = weather.daily.precipitation_probability_max[0];

		Ok(HourlyForecast {
			temperatures: Self::prepare_temperatures(temperatures, weather_codes, night, t)?,
			precipitation,
			temp_max_min,
			precipitation_probability_max,
			graph: Graph::prepare_graph(temperatures, graph_opts)?,
			time_indicator_col,
		})
	}

	fn prepare_temperatures(
		temperatures: &[f32],
		weather_codes: &[u8],
		night: bool,
		t: &WeatherCodeLocales,
	) -> Result<String> {
		let mut result = String::new();

		for hour in DISPLAY_HOURS {
			let temp = temperatures[hour].round() as i32;
			let temp_sub = style_number(temp, true)?;
			let wmo_code = WeatherCode::resolve(weather_codes[hour], night, t)?;
			let colspan = if hour == 0 { 2 } else { 8 };
			let _ = write!(result, "{: >colspan$}{}", temp_sub, wmo_code.icon);
		}

		Ok(result)
	}

	// TODO: make precipitation fns generic by chance
	fn prepare_precipitation(precipitation: &[f32]) -> Result<String> {
		let mut result = String::new();

		for hour in DISPLAY_HOURS {
			let precipitation_sup = style_number(precipitation[hour].ceil() as i32, true)?;
			let colspan = if hour == 0 { 2 } else { 8 };
			let _ = write!(result, "{precipitation_sup: >colspan$} ");
		}

		Ok(result)
	}

	fn prepare_precipitation_probability(precipitation: &[u8]) -> Result<String> {
		let mut result = String::new();

		for hour in DISPLAY_HOURS {
			let precipitation_sup = style_number(precipitation[hour].into(), true)?;
			let colspan = if hour == 0 { 2 } else { 8 };
			let _ = write!(result, "{precipitation_sup: >colspan$} ");
		}

		Ok(result)
	}

	fn prepare_separator(&self, border_variant: &BorderStyle, width: usize, time_indicator_glyph: char) -> String {
		let time_indicator_col = self.time_indicator_col.unwrap();

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

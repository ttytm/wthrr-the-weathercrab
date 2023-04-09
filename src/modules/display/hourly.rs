use anyhow::Result;
use chrono::{Timelike, Utc};
use colored::{
	Color::{Blue, BrightBlack, Yellow},
	Colorize,
};
use std::fmt::Write as _;

use crate::modules::{
	localization::WeatherCodeLocales,
	params::Params,
	units::{Precipitation, Temperature, Time},
	weather::OptionalWeather,
};

use super::{
	border::{Border, BorderStyle, Separator},
	graph::Graph,
	gui_config::ColorOption,
	product::Product,
	utils::{lang_len_diff, style_number},
	weathercode::WeatherCode,
};

const DISPLAY_HOURS: [usize; 8] = [0, 3, 6, 9, 12, 15, 18, 21];
pub const WIDTH: usize = 72;

pub struct HourlyForecast {
	heading: String,
	temperatures: String,
	precipitation: String,
	graph: Graph,
	summary: Option<WeatherSummery>,
	time_indicator_col: Option<usize>,
}

struct WeatherSummery {
	temp_max_min: String,
	precipitation_probability_max: u8,
}

impl HourlyForecast {
	pub fn render(self, params: &Params) {
		let HourlyForecast {
			heading,
			temperatures,
			precipitation,
			summary,
			graph,
			time_indicator_col,
		} = self;

		let (units, gui) = (&params.config.units, &params.config.gui);

		// Blank Line
		println!(
			"{}",
			&Separator::Blank.fmt(WIDTH, &gui.border).color_option(BrightBlack, &gui.color)
		);

		// Set Measurment Unit Symbols
		let temperature_unit = match units.temperature {
			Temperature::fahrenheit => "",
			_ => "",
		};
		let precipitation_unit = match units.precipitation {
			Precipitation::mm => "ₘₘ",
			Precipitation::inch => "ᵢₙ",
			_ => "󰖎 ",
		};

		// Hourly Forecast Heading
		println!(
			"{} {: <WIDTH$} {}",
			Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			heading.bold(),
			Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			WIDTH = WIDTH - 2 - lang_len_diff(&heading, &params.config.language)
		);

		// Day Max/Mix Temperatur + Max Precipitation
		if let Some(summary) = summary {
			println!(
				"{} {} ❲{}{}❳{: <WIDTH$} {}",
				Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
				summary.temp_max_min,
				summary.precipitation_probability_max,
				"󰖎".bold(),
				"",
				Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color),
				WIDTH =
					WIDTH - 5 - summary.temp_max_min.len() - summary.precipitation_probability_max.to_string().len()
			);
		}

		// Graph Border Top with Potential Time Indicator
		match time_indicator_col {
			Some(col) => {
				println!(
					"{}",
					Self::prepare_separator(col, &gui.border, WIDTH, '╤').color_option(BrightBlack, &gui.color),
				);
			}
			_ => {
				println!(
					"{}",
					Separator::Dashed.fmt(WIDTH, &gui.border).color_option(BrightBlack, &gui.color)
				);
			}
		}

		// Temperatures
		println!(
			"{} {: <WIDTH$}{} {}",
			Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			temperatures.color_option(Yellow, &gui.color).bold(),
			temperature_unit.color_option(Yellow, &gui.color).bold(),
			Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			WIDTH = WIDTH - 3
		);

		// Blank Line
		println!(
			"{}",
			&Separator::Blank.fmt(WIDTH, &gui.border).color_option(BrightBlack, &gui.color)
		);

		// Graph Row 1
		if graph.1.chars().count() > 0 {
			println!(
				"{}{}{}",
				Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
				graph.1.color_option(Yellow, &gui.color),
				Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color)
			);
		}
		// Graph Row 2
		println!(
			"{}{}{}",
			Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			graph.0.color_option(Yellow, &gui.color),
			Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color)
		);

		// Precipitation
		println!(
			"{} {: <WIDTH$}{}{}",
			Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			precipitation.color_option(Blue, &gui.color).bold(),
			if units.precipitation == Precipitation::probability {
				// to enlarge the water percent icon we use bold as a hack
				precipitation_unit.color_option(Blue, &gui.color).bold()
			} else {
				precipitation_unit.color_option(Blue, &gui.color)
			},
			Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			WIDTH = WIDTH - 1 - precipitation_unit.chars().count()
		);

		// Graph Border Bottom with Potential Time Indicator
		match time_indicator_col {
			Some(col) => {
				println!(
					"{}",
					Self::prepare_separator(col, &gui.border, WIDTH, '╧').color_option(BrightBlack, &gui.color),
				);
			}
			_ => {
				println!(
					"{}",
					Separator::Dashed.fmt(WIDTH, &gui.border).color_option(BrightBlack, &gui.color)
				);
			}
		}

		// Graph Hours Row
		print!("{}", Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color));
		let hours = match units.time {
			Time::am_pm => ["¹²·⁰⁰ₐₘ", "³·⁰⁰ₐₘ", "⁶˙⁰⁰ₐₘ", "⁹˙⁰⁰ₐₘ", "¹²˙⁰⁰ₚₘ", "³˙⁰⁰ₚₘ", "⁶˙⁰⁰ₚₘ", "⁹˙⁰⁰ₚₘ"],
			_ => ["⁰⁰˙⁰⁰", "⁰³˙⁰⁰", "⁰⁶˙⁰⁰", "⁰⁹˙⁰⁰", "¹²˙⁰⁰", "¹⁵˙⁰⁰", "¹⁸˙⁰⁰", "²¹˙⁰⁰"],
		};
		for hour in hours {
			print!("{hour: <9}");
		}
		println!("{}", Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color));
	}

	pub fn prepare(product: &Product, params: &Params, day_index: usize) -> Result<Self> {
		let weather = &product.weather;
		let current_hour = weather.current_weather.time[11..13].parse::<usize>().unwrap_or_default();

		// The graph splits one hour into three "levels": last, current and next.
		// We slice 25 items to use the 25th in the last "next"-level of a graph.
		let day_start_idx = day_index * 24;
		let day_end_idx = day_start_idx + 24;
		let next_day_start_idx = day_end_idx + 1;
		let next_day_end_idx = next_day_start_idx + 24;

		let (temperatures, weather_codes, precipitation): (&[f32], &[u8], Vec<u8>);
		let (mut temps, mut codes, mut prec);
		// If it's the last possible requested day, the last index(start_index of the 7th day) is not available.
		// Therefore we'll extend the values by 1. For this we simply use the last value of the array twice.
		if day_index == 6 {
			temperatures = {
				temps = weather.hourly.temperature_2m[day_start_idx..].to_vec();
				temps.push(weather.hourly.temperature_2m[weather.hourly.temperature_2m.len() - 1]);
				&temps
			};
			weather_codes = {
				codes = weather.hourly.weathercode[day_start_idx..].to_vec();
				codes.push(weather.hourly.weathercode[weather.hourly.weathercode.len() - 1]);
				&codes
			};
			precipitation = match params.config.units.precipitation {
				Precipitation::probability => {
					prec = weather.hourly.precipitation_probability[day_start_idx..].to_vec();
					prec.push(
						weather.hourly.precipitation_probability[weather.hourly.precipitation_probability.len() - 1],
					);
					prec
				}
				_ => weather.hourly.precipitation[day_start_idx..]
					.iter()
					.map(|x| x.ceil() as u8)
					.collect::<Vec<u8>>(),
			};
		// If it's the end of one day we show the weather of the next day
		} else if current_hour == day_end_idx - 1 {
			temperatures = &weather.hourly.temperature_2m[next_day_start_idx..=next_day_end_idx];
			weather_codes = &weather.hourly.weathercode[next_day_start_idx..=next_day_end_idx];
			precipitation = match params.config.units.precipitation {
				Precipitation::probability => {
					weather.hourly.precipitation_probability[next_day_start_idx..=next_day_end_idx].to_vec()
				}
				_ => weather.hourly.precipitation[next_day_start_idx..=next_day_end_idx]
					.iter()
					.map(|x| x.ceil() as u8)
					.collect::<Vec<u8>>(),
			};
		} else {
			temperatures = &weather.hourly.temperature_2m[day_start_idx..=day_end_idx];
			weather_codes = &weather.hourly.weathercode[day_start_idx..=day_end_idx];
			precipitation = match params.config.units.precipitation {
				Precipitation::probability => {
					weather.hourly.precipitation_probability[day_start_idx..=day_end_idx].to_vec()
				}
				_ => weather.hourly.precipitation[day_start_idx..=day_end_idx]
					.iter()
					.map(|x| x.ceil() as u8)
					.collect::<Vec<u8>>(),
			};
		};

		let time_indicator_col = if day_index == 0 && params.config.gui.graph.time_indicator {
			let col_adjustment = if current_hour == day_end_idx - 1 {
				// if it's the last hour of the day, the time idicator will be placed at the beginning of the graph
				1
			} else {
				// add 3 cols to adjust to the multiple chars used to display the current hour below the chart
				(current_hour * 3) + 3
			};
			Some(col_adjustment + (Timelike::minute(&Utc::now()) / 20) as usize)
		} else {
			None
		};

		let sunrise_sunset = (
			weather.daily.sunrise[day_index][11..13].parse::<usize>().unwrap_or_default(),
			weather.daily.sunset[day_index][11..13].parse::<usize>().unwrap_or_default(),
		);

		// Future or historical forecast already include a weather Max/Min summary in the top part of the display.
		let summary = match day_index {
			0 => Some(WeatherSummery {
				temp_max_min: format!(
					"{:.1}/{:.1}{}",
					weather.daily.temperature_2m_max[day_index],
					weather.daily.temperature_2m_min[day_index],
					weather.daily_units.temperature_2m_max,
				),
				precipitation_probability_max: weather.daily.precipitation_probability_max[day_index],
			}),
			_ => None,
		};

		Ok(HourlyForecast {
			heading: params.texts.weather.hourly_forecast.to_string(),
			temperatures: Self::prepare_temperatures(
				temperatures,
				weather_codes,
				sunrise_sunset,
				&params.texts.weather.weather_code,
			)?,
			precipitation: Self::prepare_precipitation(&precipitation),
			summary,
			graph: Graph::prepare_graph(temperatures, &params.config.gui.graph),
			time_indicator_col,
		})
	}

	pub fn prepare_historical(weather: &OptionalWeather, params: &Params) -> Result<Self> {
		// If it's the last possible requested day, the last index(start_index of the 7th day) is not available.
		// Therefore we'll extend the values by 1. For this we simply use the last value of the array twice.
		let (mut temps, mut codes, mut prec);
		let temperatures = {
			temps = weather.hourly.as_ref().unwrap().temperature_2m.as_ref().unwrap()[0..].to_vec();
			temps.push(temps[temps.len() - 1]);
			&temps
		};
		let weather_codes = {
			codes = weather.hourly.as_ref().unwrap().weathercode.as_ref().unwrap()[0..].to_vec();
			codes.push(codes[codes.len() - 1]);
			&codes
		};
		let sunrise_sunset = (
			weather.daily.as_ref().unwrap().sunrise.as_ref().unwrap()[0][11..13]
				.parse::<usize>()
				.unwrap_or_default(),
			weather.daily.as_ref().unwrap().sunset.as_ref().unwrap()[0][11..13]
				.parse::<usize>()
				.unwrap_or_default(),
		);
		let precipitation = {
			prec = weather.hourly.as_ref().unwrap().precipitation.as_ref().unwrap()[0..].to_vec();
			prec.push(prec[prec.len() - 1]);
			prec.iter().map(|x| x.ceil() as u8).collect::<Vec<u8>>()
		};

		Ok(Self {
			heading: params.texts.weather.daily_overview.to_string(),
			temperatures: Self::prepare_temperatures(
				temperatures,
				weather_codes,
				sunrise_sunset,
				&params.texts.weather.weather_code,
			)?,
			precipitation: Self::prepare_precipitation(&precipitation),
			summary: None,
			graph: Graph::prepare_graph(temperatures, &params.config.gui.graph),
			time_indicator_col: None,
		})
	}

	fn prepare_temperatures(
		temperatures: &[f32],
		weather_codes: &[u8],
		sunrise_sunset: (usize, usize),
		t: &WeatherCodeLocales,
	) -> Result<String> {
		let mut result = String::new();

		for hour in DISPLAY_HOURS {
			let temp = temperatures[hour].round() as i32;
			let temp_sub = style_number(temp, true);
			let night = hour < sunrise_sunset.0 || hour > sunrise_sunset.1;
			let wmo_code = WeatherCode::resolve(weather_codes[hour], night, t)?;
			let colspan = if hour == 0 { 2 } else { 8 };
			let _ = write!(result, "{: >colspan$}{}", temp_sub, wmo_code.icon);
		}

		Ok(result)
	}

	fn prepare_precipitation(precipitation: &[u8]) -> String {
		let mut result = String::new();

		for hour in DISPLAY_HOURS {
			let precipitation_sup = style_number(precipitation[hour].into(), true);
			let colspan = if hour == 0 { 2 } else { 8 };
			let _ = write!(result, "{precipitation_sup: >colspan$} ");
		}

		result
	}

	fn prepare_separator(
		time_indicator_col: usize,
		border_variant: &BorderStyle,
		width: usize,
		time_indicator_glyph: char,
	) -> String {
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

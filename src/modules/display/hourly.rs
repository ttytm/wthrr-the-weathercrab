use anyhow::Result;
use chrono::{NaiveDateTime, Timelike};
use dialoguer::console::style;
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
	gui_config::ConfigurableColor,
	product::{Product, TOTAL_BORDER_PADDING},
	utils::{pad_string_to_width, style_number},
	weathercode::WeatherCode,
};

const DISPLAY_HOURS: [usize; 8] = [0, 3, 6, 9, 12, 15, 18, 21];
pub const WIDTH: usize = 72;

struct WeatherSummary {
	temp_max_min: String,
	precipitation_probability_max: u8,
}

pub fn prep(product: &Product, params: &Params, day_index: usize) -> Result<Vec<String>> {
	let weather = &product.weather;
	let current_dt = NaiveDateTime::parse_from_str(&product.weather.current_weather.time, "%Y-%m-%dT%H:%M")?;
	let current_hour = current_dt.hour() as usize;

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
				prec.push(weather.hourly.precipitation_probability[weather.hourly.precipitation_probability.len() - 1]);
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
		Some(col_adjustment + (current_dt.minute() / 20) as usize)
	} else {
		None
	};

	let sunrise_sunset = (
		weather.daily.sunrise[day_index][11..13].parse::<usize>().unwrap_or_default(),
		weather.daily.sunset[day_index][11..13].parse::<usize>().unwrap_or_default(),
	);

	// Future or historical forecast already include a weather Max/Min summary in the top part of the display.
	let summary = match day_index {
		0 => Some(WeatherSummary {
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

	let (units, gui) = (&params.config.units, &params.config.gui);
	let width_no_border_pad = WIDTH - TOTAL_BORDER_PADDING;

	let mut result = Vec::<String>::new();

	// Blank Line
	result.push(format!(
		"{}",
		&Separator::Blank.fmt(WIDTH, &gui.border).plain_or_bright_black(&gui.color),
	));

	// Set Measurement Unit Symbols
	let temperature_unit = match units.temperature {
		Temperature::fahrenheit => "",
		Temperature::celsius => "",
	};
	let precipitation_unit = match units.precipitation {
		Precipitation::mm => "ₘₘ",
		Precipitation::inch => "ᵢₙ",
		Precipitation::probability => "󰖎 ",
	};

	// Hourly Forecast Heading
	result.push(format!(
		"{} {} {}",
		Border::L.fmt(&gui.border).plain_or_bright_black(&gui.color),
		style(pad_string_to_width(&params.texts.weather.hourly_forecast, width_no_border_pad)).bold(),
		Border::R.fmt(&gui.border).plain_or_bright_black(&gui.color),
	));

	// Day Max/Mix Temperature + Max Precipitation
	if let Some(summary) = summary {
		result.push(format!(
			"{} {} ❲{}{}❳{: <WIDTH$} {}",
			Border::L.fmt(&gui.border).plain_or_bright_black(&gui.color),
			summary.temp_max_min,
			summary.precipitation_probability_max,
			style("󰖎").bold(),
			"",
			Border::R.fmt(&gui.border).plain_or_bright_black(&gui.color),
			WIDTH = WIDTH - 5 - summary.temp_max_min.len() - summary.precipitation_probability_max.to_string().len()
		));
	}

	// Graph Border Top with Potential Time Indicator
	match time_indicator_col {
		Some(col) => result.push(format!(
			"{}",
			prepare_separator(col, &gui.border, WIDTH, '╤').plain_or_bright_black(&gui.color)
		)),
		_ => result.push(format!(
			"{}",
			Separator::Dashed.fmt(WIDTH, &gui.border).plain_or_bright_black(&gui.color)
		)),
	};

	// Temperatures
	let temps = prepare_temperatures(temperatures, weather_codes, sunrise_sunset, &params.texts.weather.weather_code)?;
	result.push(format!(
		"{} {: <WIDTH$}{} {}",
		Border::L.fmt(&gui.border).plain_or_bright_black(&gui.color),
		temps.plain_or_yellow(&gui.color).bold(),
		temperature_unit.plain_or_yellow(&gui.color).bold(),
		Border::R.fmt(&gui.border).plain_or_bright_black(&gui.color),
		WIDTH = WIDTH - 3
	));

	// Blank Line
	result.push(format!(
		"{}",
		&Separator::Blank.fmt(WIDTH, &gui.border).plain_or_bright_black(&gui.color)
	));

	let graph = Graph::prepare_graph(temperatures, &params.config.gui.graph);
	// Graph Row 1
	if graph.1.chars().count() > 0 {
		result.push(format!(
			"{}{}{}",
			Border::L.fmt(&gui.border).plain_or_bright_black(&gui.color),
			graph.1.plain_or_yellow(&gui.color),
			Border::R.fmt(&gui.border).plain_or_bright_black(&gui.color),
		));
	}
	// Graph Row 2
	result.push(format!(
		"{}{}{}",
		Border::L.fmt(&gui.border).plain_or_bright_black(&gui.color),
		graph.0.plain_or_yellow(&gui.color),
		Border::R.fmt(&gui.border).plain_or_bright_black(&gui.color),
	));

	// Precipitation
	let precipitation = prepare_precipitation(&precipitation);
	result.push(format!(
		"{} {: <WIDTH$}{}{}",
		Border::L.fmt(&gui.border).plain_or_bright_black(&gui.color),
		precipitation.plain_or_blue(&gui.color).bold(),
		if units.precipitation == Precipitation::probability {
			// to enlarge the water percent icon we use bold as a hack
			precipitation_unit.plain_or_blue(&gui.color).bold()
		} else {
			precipitation_unit.plain_or_blue(&gui.color)
		},
		Border::R.fmt(&gui.border).plain_or_bright_black(&gui.color),
		WIDTH = WIDTH - 1 - precipitation_unit.chars().count()
	));

	// Graph Border Bottom with Potential Time Indicator
	match time_indicator_col {
		Some(col) => result.push(format!(
			"{}",
			prepare_separator(col, &gui.border, WIDTH, '╧').plain_or_bright_black(&gui.color)
		)),
		_ => result.push(format!(
			"{}",
			Separator::Dashed.fmt(WIDTH, &gui.border).plain_or_bright_black(&gui.color)
		)),
	};

	// Graph Hours Row
	let mut hours_row = format!("{}", Border::L.fmt(&gui.border).plain_or_bright_black(&gui.color));
	let hours = match units.time {
		Time::am_pm => ["¹²˙⁰⁰ₐₘ", "³˙⁰⁰ₐₘ", "⁶˙⁰⁰ₐₘ", "⁹˙⁰⁰ₐₘ", "¹²˙⁰⁰ₚₘ", "³˙⁰⁰ₚₘ", "⁶˙⁰⁰ₚₘ", "⁹˙⁰⁰ₚₘ"],
		Time::military => ["⁰⁰˙⁰⁰", "⁰³˙⁰⁰", "⁰⁶˙⁰⁰", "⁰⁹˙⁰⁰", "¹²˙⁰⁰", "¹⁵˙⁰⁰", "¹⁸˙⁰⁰", "²¹˙⁰⁰"],
	};
	for hour in hours {
		hours_row.push_str(&format!("{hour: <9}"));
	}
	hours_row.push_str(&format!("{}", Border::R.fmt(&gui.border).plain_or_bright_black(&gui.color)));
	result.push(hours_row);

	Ok(result)
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

pub fn prep_historical(weather: &OptionalWeather, params: &Params) -> Result<Vec<String>> {
	// If it's the last possible requested day, the last index(start_index of the 7th day) is not available.
	// Therefore we'll extend the values by 1. For this we simply use the last value of the array twice.
	let (mut temps, mut codes, mut prec);
	let temperatures = {
		temps = weather.hourly.temperature_2m.as_ref().unwrap()[0..].to_vec();
		temps.push(temps[temps.len() - 1]);
		&temps
	};
	let weather_codes = {
		codes = weather.hourly.weathercode.as_ref().unwrap()[0..].to_vec();
		codes.push(codes[codes.len() - 1]);
		&codes
	};
	let sunrise_sunset = (
		weather.daily.sunrise.as_ref().unwrap()[0][11..13]
			.parse::<usize>()
			.unwrap_or_default(),
		weather.daily.sunset.as_ref().unwrap()[0][11..13]
			.parse::<usize>()
			.unwrap_or_default(),
	);
	let precipitation = {
		prec = weather.hourly.precipitation.as_ref().unwrap()[0..].to_vec();
		prec.push(prec[prec.len() - 1]);
		prec.iter().map(|x| x.ceil() as u8).collect::<Vec<u8>>()
	};

	let (units, gui) = (&params.config.units, &params.config.gui);
	let width_no_border_pad = WIDTH - 2;

	let mut result = Vec::<String>::new();

	// Blank Line
	result.push(format!(
		"{}",
		&Separator::Blank.fmt(WIDTH, &gui.border).plain_or_bright_black(&gui.color),
	));

	// Set Measurement Unit Symbols
	let temperature_unit = match units.temperature {
		Temperature::fahrenheit => "",
		Temperature::celsius => "",
	};
	let precipitation_unit = match units.precipitation {
		Precipitation::mm => "ₘₘ",
		Precipitation::inch => "ᵢₙ",
		Precipitation::probability => "󰖎 ",
	};

	// Hourly Forecast Heading
	result.push(format!(
		"{} {} {}",
		Border::L.fmt(&gui.border).plain_or_bright_black(&gui.color),
		style(pad_string_to_width(&params.texts.weather.daily_overview, width_no_border_pad)).bold(),
		Border::R.fmt(&gui.border).plain_or_bright_black(&gui.color),
	));

	// Graph Border Top
	result.push(format!(
		"{}",
		Separator::Dashed.fmt(WIDTH, &gui.border).plain_or_bright_black(&gui.color)
	));

	// Temperatures
	let temps = prepare_temperatures(temperatures, weather_codes, sunrise_sunset, &params.texts.weather.weather_code)?;
	result.push(format!(
		"{} {: <WIDTH$}{} {}",
		Border::L.fmt(&gui.border).plain_or_bright_black(&gui.color),
		temps.plain_or_yellow(&gui.color).bold(),
		temperature_unit.plain_or_yellow(&gui.color).bold(),
		Border::R.fmt(&gui.border).plain_or_bright_black(&gui.color),
		WIDTH = WIDTH - 3
	));

	// Blank Line
	result.push(format!(
		"{}",
		&Separator::Blank.fmt(WIDTH, &gui.border).plain_or_bright_black(&gui.color)
	));

	let graph = Graph::prepare_graph(temperatures, &params.config.gui.graph);
	// Graph Row 1
	if graph.1.chars().count() > 0 {
		result.push(format!(
			"{}{}{}",
			Border::L.fmt(&gui.border).plain_or_bright_black(&gui.color),
			graph.1.plain_or_yellow(&gui.color),
			Border::R.fmt(&gui.border).plain_or_bright_black(&gui.color),
		));
	}
	// Graph Row 2
	result.push(format!(
		"{}{}{}",
		Border::L.fmt(&gui.border).plain_or_bright_black(&gui.color),
		graph.0.plain_or_yellow(&gui.color),
		Border::R.fmt(&gui.border).plain_or_bright_black(&gui.color),
	));

	// Precipitation
	let precipitation = prepare_precipitation(&precipitation);
	result.push(format!(
		"{} {: <WIDTH$}{}{}",
		Border::L.fmt(&gui.border).plain_or_bright_black(&gui.color),
		precipitation.plain_or_blue(&gui.color).bold(),
		if units.precipitation == Precipitation::probability {
			// to enlarge the water percent icon we use bold as a hack
			precipitation_unit.plain_or_blue(&gui.color).bold()
		} else {
			precipitation_unit.plain_or_blue(&gui.color)
		},
		Border::R.fmt(&gui.border).plain_or_bright_black(&gui.color),
		WIDTH = WIDTH - 1 - precipitation_unit.chars().count()
	));

	// Graph Border Bottom
	result.push(format!(
		"{}",
		Separator::Dashed.fmt(WIDTH, &gui.border).plain_or_bright_black(&gui.color)
	));

	// Graph Hours Row
	let mut hours_row = format!("{}", Border::L.fmt(&gui.border).plain_or_bright_black(&gui.color));
	let hours = match units.time {
		Time::am_pm => ["¹²˙⁰⁰ₐₘ", "³˙⁰⁰ₐₘ", "⁶˙⁰⁰ₐₘ", "⁹˙⁰⁰ₐₘ", "¹²˙⁰⁰ₚₘ", "³˙⁰⁰ₚₘ", "⁶˙⁰⁰ₚₘ", "⁹˙⁰⁰ₚₘ"],
		Time::military => ["⁰⁰˙⁰⁰", "⁰³˙⁰⁰", "⁰⁶˙⁰⁰", "⁰⁹˙⁰⁰", "¹²˙⁰⁰", "¹⁵˙⁰⁰", "¹⁸˙⁰⁰", "²¹˙⁰⁰"],
	};
	for hour in hours {
		hours_row.push_str(&format!("{hour: <9}"));
	}
	hours_row.push_str(&format!("{}", Border::R.fmt(&gui.border).plain_or_bright_black(&gui.color)));
	result.push(hours_row);

	Ok(result)
}

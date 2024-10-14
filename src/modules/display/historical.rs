use anyhow::Result;
use chrono::NaiveDate;
use dialoguer::console::style;
use unicode_width::UnicodeWidthStr;

use crate::modules::{
	display::hourly::WIDTH,
	localization::Locales,
	params::Params,
	units::{Precipitation, Time},
};

use super::{
	border::{Border, BorderStyle, Edge, Separator},
	gui_config::ConfigurableColor,
	hourly,
	product::{Product, TOTAL_BORDER_PADDING},
	utils::pad_string_to_width,
	weathercode::WeatherCode,
};

#[allow(clippy::too_many_lines)]
pub fn prep(product: &Product, params: &Params, date: NaiveDate) -> Result<Vec<String>> {
	let address = Product::trunc_address(product.address.clone(), 60);

	// Helpers
	let weather = &product.historical_weather.as_ref().unwrap()[&date];
	let weather_daily_units = weather.daily_units.as_ref().unwrap();
	let lang = &params.config.language;
	// Times
	let sunrise = weather.daily.sunrise.as_ref().unwrap();
	let sunset = weather.daily.sunset.as_ref().unwrap();
	let (sunrise_hour, sunset_hour) = (
		sunrise[0][11..13].parse::<usize>().unwrap_or_default(),
		sunset[0][11..13].parse::<usize>().unwrap_or_default(),
	);

	// Display Items
	let sunrise = match params.config.units.time {
		Time::am_pm => format!("{}:{}am", sunrise_hour, &sunrise[0][14..16]),
		Time::military => sunrise[0][11..16].to_string(),
	};
	let sunset = match params.config.units.time {
		Time::am_pm => format!("{}:{}pm", sunset_hour - 12, &sunset[0][14..16]),
		Time::military => sunset[0][11..16].to_string(),
	};
	let temp_max_min = format!(
		"{:.1}/{:.1}{}",
		weather.daily.temperature_2m_max.as_ref().unwrap()[0],
		weather.daily.temperature_2m_min.as_ref().unwrap()[0],
		weather_daily_units.temperature_2m_max,
	);
	let apparent_temp_max_min = format!(
		"{} {:.1}/{:.1}{}",
		params.texts.weather.felt_like,
		weather.daily.apparent_temperature_max.as_ref().unwrap()[0],
		weather.daily.apparent_temperature_min.as_ref().unwrap()[0],
		weather_daily_units.temperature_2m_max,
	);
	let precipitation_sum = format!(
		"❲{}{}❳",
		weather.daily.precipitation_sum.as_ref().unwrap()[0],
		if params.config.units.precipitation == Precipitation::inch {
			"ᵢₙ"
		} else {
			"ₘₘ"
		}
	);
	let date = format!(
		" {}",
		if lang == "en_US" || lang == "en" {
			date.format("%a, %-d %b %Y").to_string()
		} else {
			Locales::localize_date(date, lang)?
		}
	);
	let sunrise = format!(" {sunrise}");
	let sunset = format!(" {sunset}");
	let wmo_code = WeatherCode::resolve(
		weather.daily.weathercode.as_ref().unwrap()[0],
		false,
		&params.texts.weather.weather_code,
	)?;

	let gui = &params.config.gui;
	let width_no_border_pad = WIDTH - TOTAL_BORDER_PADDING;

	let mut result = Vec::<String>::new();

	// Border Top
	result.push(format!(
		"{}",
		&Edge::Top.fmt(WIDTH, gui.border).plain_or_bright_black(gui.color)
	));

	// Address / Title
	result.push(format!(
		"{} {} {}",
		Border::L.fmt(gui.border).plain_or_bright_black(gui.color),
		style(pad_string_to_width(&address, width_no_border_pad)).bold(),
		Border::R.fmt(gui.border).plain_or_bright_black(gui.color),
	));

	// Separator
	result.push(format!(
		"{}",
		&match gui.border {
			BorderStyle::double => Separator::Double.fmt(WIDTH, gui.border),
			BorderStyle::solid => Separator::Solid.fmt(WIDTH, gui.border),
			_ => Separator::Single.fmt(WIDTH, gui.border),
		}
		.plain_or_bright_black(gui.color)
	));

	// Temperature & Weathercode
	let temperature_and_weathercode = format!(
		"{} {}, {} {}",
		wmo_code.icon, wmo_code.interpretation, temp_max_min, precipitation_sum
	);
	result.push(format!(
		"{} {}{} {}",
		Border::L.fmt(gui.border).plain_or_bright_black(gui.color),
		style(pad_string_to_width(
			&temperature_and_weathercode,
			width_no_border_pad - date.width()
		))
		.bold(),
		date,
		Border::R.fmt(gui.border).plain_or_bright_black(gui.color),
	));

	// Apparent Temperature & Sun Rise & Sun Set
	let sunrise_and_sunset = format!("{sunrise}  {sunset}");
	result.push(format!(
		"{} {}{} {}",
		Border::L.fmt(gui.border).plain_or_bright_black(gui.color),
		pad_string_to_width(&apparent_temp_max_min, width_no_border_pad - sunrise_and_sunset.width()),
		sunrise_and_sunset,
		Border::R.fmt(gui.border).plain_or_bright_black(gui.color),
	));

	// Hourly Overview
	// Adjust forecast config to use values that are compatible with historical weather.
	let mut params = params.clone();
	params.config.gui.graph.time_indicator = false;
	if params.config.units.precipitation == Precipitation::probability {
		params.config.units.precipitation = Precipitation::mm;
	};
	for line in hourly::prep_historical(weather, &params)? {
		result.push(line);
	}

	// Border Bottom
	result.push(format!(
		"{}",
		Edge::Bottom.fmt(WIDTH, gui.border).plain_or_bright_black(gui.color)
	));

	Ok(result)
}

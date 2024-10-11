use anyhow::Result;
use chrono::{Duration, Local};
use dialoguer::console::style;
use unicode_width::UnicodeWidthStr;

use crate::modules::{localization::Locales, params::Params, units::Time};

use super::{
	border::{Border, BorderStyle, Edge, Separator},
	gui_config::ConfigurableColor,
	hourly,
	hourly::WIDTH,
	product::{Product, TOTAL_BORDER_PADDING},
	utils::pad_string_to_width,
	weathercode::WeatherCode,
};

pub fn prep(product: &Product, params: &Params, day_index: usize) -> Result<Vec<String>> {
	let weather = &product.weather;
	let address = Product::trunc_address(product.address.clone(), 60);

	// Times
	let (current_hour, sunrise_hour, sunset_hour) = (
		weather.current_weather.time[11..13].parse::<usize>().unwrap_or_default(),
		weather.daily.sunrise[day_index][11..13].parse::<usize>().unwrap_or_default(),
		weather.daily.sunset[day_index][11..13].parse::<usize>().unwrap_or_default(),
	);
	let sunrise = match params.config.units.time {
		Time::am_pm => format!("{}:{}am", sunrise_hour, &weather.daily.sunrise[day_index][14..16]),
		Time::military => weather.daily.sunrise[day_index][11..16].to_string(),
	};
	let sunset = match params.config.units.time {
		Time::am_pm => format!("{}:{}pm", sunset_hour - 12, &weather.daily.sunset[day_index][14..16]),
		Time::military => weather.daily.sunset[day_index][11..16].to_string(),
	};
	let night = current_hour < sunrise_hour || current_hour > sunset_hour;

	let temp_max_min = format!(
		"{:.1}/{:.1}{}",
		weather.daily.temperature_2m_max[day_index],
		weather.daily.temperature_2m_min[day_index],
		weather.daily_units.temperature_2m_max,
	);
	let apparent_temp_max_min = format!(
		"{} {:.1}/{:.1}{}",
		params.texts.weather.feels_like,
		weather.daily.apparent_temperature_max[day_index],
		weather.daily.apparent_temperature_min[day_index],
		weather.daily_units.temperature_2m_max,
	);
	let precipitation_probability_max = format!("❲{}󰖎❳", weather.daily.precipitation_probability_max[day_index]);

	let dt = Local::now() + Duration::days(day_index as i64);
	let lang = &params.config.language;
	let date = format!(
		" {}",
		if lang == "en_US" || lang == "en" {
			dt.format("%a, %e %b").to_string()
		} else {
			Locales::localize_date(dt.date_naive(), lang)?
		}
	);
	let sunrise = format!(" {sunrise}");
	let sunset = format!(" {sunset}");
	let wmo_code =
		WeatherCode::resolve(weather.daily.weathercode[day_index], night, &params.texts.weather.weather_code)?;

	let gui = &params.config.gui;
	let width_no_border_pad = WIDTH - TOTAL_BORDER_PADDING;

	let mut result = Vec::<String>::new();

	// Border Top
	result.push(format!(
		"{}",
		&Edge::Top.fmt(WIDTH, &gui.border).plain_or_bright_black(&gui.color)
	));

	// Address / Title
	result.push(format!(
		"{} {} {}",
		Border::L.fmt(&gui.border).plain_or_bright_black(&gui.color),
		style(pad_string_to_width(&address, width_no_border_pad)).bold(),
		Border::R.fmt(&gui.border).plain_or_bright_black(&gui.color),
	));

	// Separator
	result.push(format!(
		"{}",
		&match &gui.border {
			BorderStyle::double => Separator::Double.fmt(WIDTH, &gui.border),
			BorderStyle::solid => Separator::Solid.fmt(WIDTH, &gui.border),
			_ => Separator::Single.fmt(WIDTH, &gui.border),
		}
		.plain_or_bright_black(&gui.color),
	));

	// Temperature & Weathercode
	let temperature_and_weathercode = format!(
		"{} {}, {} {}",
		wmo_code.icon, wmo_code.interpretation, temp_max_min, precipitation_probability_max
	);
	result.push(format!(
		"{} {}{} {}",
		Border::L.fmt(&gui.border).plain_or_bright_black(&gui.color),
		style(pad_string_to_width(
			&temperature_and_weathercode,
			width_no_border_pad - date.width()
		))
		.bold(),
		date,
		Border::R.fmt(&gui.border).plain_or_bright_black(&gui.color),
	));

	// Apparent Temperature & Sun Rise & Sun Set
	let sunrise_and_sunset = format!("{sunrise}  {sunset}");
	result.push(format!(
		"{} {}{} {}",
		Border::L.fmt(&gui.border).plain_or_bright_black(&gui.color),
		pad_string_to_width(&apparent_temp_max_min, width_no_border_pad - sunrise_and_sunset.width()),
		sunrise_and_sunset,
		Border::R.fmt(&gui.border).plain_or_bright_black(&gui.color),
	));

	// Hourly Forecast
	for line in hourly::prep(product, params, day_index)? {
		result.push(line);
	}

	// Border Bottom
	result.push(format!(
		"{}",
		Edge::Bottom.fmt(WIDTH, &gui.border).plain_or_bright_black(&gui.color)
	));

	Ok(result)
}

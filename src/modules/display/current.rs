use anyhow::Result;
use dialoguer::console::style;

use crate::modules::{params::Params, units::Time};

use super::{
	border::{Border, BorderStyle, Edge, Separator},
	gui_config::ConfigurableColor,
	hourly,
	product::{Product, MIN_CELL_WIDTH, MIN_WIDTH, TOTAL_BORDER_PADDING},
	utils::pad_string_to_width,
	weathercode::WeatherCode,
	wind::WindDirection,
};

pub struct Dimensions {
	pub width: usize,
	pub cell_width: usize,
}

#[allow(clippy::too_many_lines)]
pub fn prep(product: &Product, params: &Params, add_hourly: bool) -> Result<(Vec<String>, Dimensions)> {
	let weather = &product.weather;
	let address = Product::trunc_address(product.address.clone(), 60);
	let t = &params.texts.weather;

	// Times
	let (current_hour, sunrise_hour, sunset_hour) = (
		weather.current_weather.time[11..13].parse::<usize>().unwrap_or_default(),
		weather.daily.sunrise[0][11..13].parse::<usize>().unwrap_or_default(),
		weather.daily.sunset[0][11..13].parse::<usize>().unwrap_or_default(),
	);
	let sunrise = match params.config.units.time {
		Time::am_pm => format!("{}:{}am", sunrise_hour, &weather.daily.sunrise[0][14..16]),
		Time::military => weather.daily.sunrise[0][11..16].to_string(),
	};
	let sunset = match params.config.units.time {
		Time::am_pm => format!("{}:{}pm", sunset_hour - 12, &weather.daily.sunset[0][14..16]),
		Time::military => weather.daily.sunset[0][11..16].to_string(),
	};
	let night = current_hour < sunrise_hour || current_hour > sunset_hour;

	// Display Items
	let temperature = format!(
		"{:.1}{}",
		weather.current_weather.temperature, weather.hourly_units.temperature_2m
	);
	let apparent_temperature = format!(
		"{} {:.1}{}",
		t.feels_like, weather.hourly.apparent_temperature[current_hour], weather.hourly_units.temperature_2m
	);
	let humidity = format!(
		"{}: {}{}",
		t.humidity, weather.hourly.relativehumidity_2m[current_hour], weather.hourly_units.relativehumidity_2m,
	);
	let dewpoint = format!(
		"{}: {:.1}{}",
		t.dew_point, weather.hourly.dewpoint_2m[current_hour], weather.hourly_units.dewpoint_2m
	);
	let wind_direction = WindDirection::get_direction(weather.current_weather.winddirection)?;
	let wind = format!(
		"{} {}{} {}",
		wind_direction.get_icon(),
		weather.current_weather.windspeed,
		weather.hourly_units.windspeed_10m,
		wind_direction
	);
	let pressure = format!(
		" {}{}",
		weather.hourly.surface_pressure[current_hour], weather.hourly_units.surface_pressure
	);
	let sunrise = format!(" {sunrise}");
	let sunset = format!(" {sunset}");
	let wmo_code = WeatherCode::resolve(weather.current_weather.weathercode, night, &t.weather_code)?;

	// Dimensions
	// Overall width
	let width = if add_hourly {
		super::hourly::WIDTH
	} else {
		let title_width = address.chars().count();
		let title_padding = 2 * TOTAL_BORDER_PADDING; // 2 spaces on each side
		if title_width > MIN_WIDTH {
			title_width + title_padding
		} else {
			MIN_WIDTH + title_padding
		}
	};
	let width_no_border_pad = width - TOTAL_BORDER_PADDING;
	// Cell width
	let cell_width = if add_hourly {
		22
	} else {
		let longest_cell_width = humidity.chars().count();
		// increase cell_width for languages with longer texts
		if longest_cell_width > MIN_CELL_WIDTH {
			longest_cell_width
		} else {
			MIN_CELL_WIDTH + TOTAL_BORDER_PADDING
		}
	};

	let gui = &params.config.gui;

	let mut result = Vec::<String>::new();

	// Border Top
	result.push(format!(
		"{}",
		&Edge::Top.fmt(width, gui.border).plain_or_bright_black(gui.color)
	));

	// Address / Title
	// TODO: restore centered title
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
			BorderStyle::double => Separator::Double.fmt(width, gui.border),
			BorderStyle::solid => Separator::Solid.fmt(width, gui.border),
			_ => Separator::Single.fmt(width, gui.border),
		}
		.plain_or_bright_black(gui.color),
	));

	result.push(format!(
		"{} {} {}",
		Border::L.fmt(gui.border).plain_or_bright_black(gui.color),
		style(pad_string_to_width(
			&(wmo_code.icon.to_string() + " " + &wmo_code.interpretation + ", " + &temperature),
			width_no_border_pad
		))
		.bold(),
		Border::R.fmt(gui.border).plain_or_bright_black(gui.color),
	));

	// Apparent Temperature
	result.push(format!(
		"{} {} {}",
		Border::L.fmt(gui.border).plain_or_bright_black(gui.color),
		pad_string_to_width(&apparent_temperature, width_no_border_pad),
		Border::R.fmt(gui.border).plain_or_bright_black(gui.color),
	));

	// Blank Line
	result.push(format!(
		"{}",
		Separator::Blank.fmt(width, gui.border).plain_or_bright_black(gui.color)
	));

	// Humidity & Dewpoint
	result.push(format!(
		"{} {}{} {}",
		Border::L.fmt(gui.border).plain_or_bright_black(gui.color),
		pad_string_to_width(&humidity, cell_width),
		// NOTE: When using the Thai language, an apparent combining character issue was observed
		// with the dew point, resulting in the border being displaced by one space or the border
		// color being removed in some terminal/font configurations.
		pad_string_to_width(&dewpoint, width_no_border_pad - cell_width),
		Border::R.fmt(gui.border).plain_or_bright_black(gui.color),
	));

	// Wind & Pressure
	result.push(format!(
		"{} {: <cell_width$}{: <width$} {}",
		Border::L.fmt(gui.border).plain_or_bright_black(gui.color),
		wind,
		pressure,
		Border::R.fmt(gui.border).plain_or_bright_black(gui.color),
		width = width_no_border_pad - cell_width
	));

	// Sunrise & Sunset
	result.push(format!(
		"{} {: <cell_width$}{: <width$} {}",
		Border::L.fmt(gui.border).plain_or_bright_black(gui.color),
		sunrise,
		sunset,
		Border::R.fmt(gui.border).plain_or_bright_black(gui.color),
		width = width_no_border_pad - cell_width
	));

	// Hourly Forecast
	if add_hourly {
		for line in hourly::prep(product, params, 0)? {
			result.push(line);
		}
	};

	// Border Bottom
	result.push(format!(
		"{}",
		Edge::Bottom.fmt(width, gui.border).plain_or_bright_black(gui.color)
	));

	Ok((result, Dimensions { width, cell_width }))
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::modules::display::utils::common_tests::{
		init_test_params, init_test_product, TEST_PARAMS, TEST_PRODUCT,
	};
	use strip_ansi_escapes;

	#[test]
	fn output_without_forecast() {
		let test_product = TEST_PRODUCT.get_or_init(init_test_product);
		let test_params = TEST_PARAMS.get_or_init(init_test_params);

		let want = "\
╭──────────────────────────────────────╮
│ Berlin, Germany                      │
├──────────────────────────────────────┤
│  Overcast, 14.0°C                   │
│ Feels like 11.8°C                    │
│                                      │
│ Humidity: 72%    Dew Point: 8.7°C    │
│ ↑ 10.7km/h S      1001.3hPa         │
│  07:18           18:29             │
╰──────────────────────────────────────╯";

		let (lines, _) = prep(test_product, test_params, false).unwrap();
		let have = strip_ansi_escapes::strip_str(lines.join("\n"));
		assert_eq!(want, have);
	}

	#[test]
	fn output_with_hourly_forecast() {
		let test_product = TEST_PRODUCT.get_or_init(init_test_product);
		let test_params = TEST_PARAMS.get_or_init(init_test_params);

		let want = "\
╭────────────────────────────────────────────────────────────────────────╮
│ Berlin, Germany                                                        │
├────────────────────────────────────────────────────────────────────────┤
│  Overcast, 14.0°C                                                     │
│ Feels like 11.8°C                                                      │
│                                                                        │
│ Humidity: 72%         Dew Point: 8.7°C                                 │
│ ↑ 10.7km/h S           1001.3hPa                                      │
│  07:18                18:29                                          │
│                                                                        │
│ Hourly Forecast                                                        │
│ 15.1/6.8°C ❲25󰖎❳                                                       │
├┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈╤┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┤
│  ₉       ₈       ₇       ₇      ₁₂      ₁₅      ₁₄      ₁₄    │
│                                                                        │
│                                    🭺🭹🭸🭷🭷🭶🭶🭶▔▔▔▔🭶🭶🭶🭶🭶🭶🭷🭷🭷🭷🭷🭷🭸🭸🭸🭸🭸🭸🭹🭹🭹🭹🭹🭹│
│🭹🭹🭹🭺🭺🭺🭺🭺🭺🭻🭻🭻🭻🭻🭻▁▁▁▁▁▁▁▁▁▁▁▁🭻🭻🭺🭹🭸🭷🭶▔▔                                    │
│  ₀        ₀        ₀        ₀        ₀       ₂₅        ₀        ₀    󰖎 │
├┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈╧┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┤
│⁰⁰˙⁰⁰    ⁰³˙⁰⁰    ⁰⁶˙⁰⁰    ⁰⁹˙⁰⁰    ¹²˙⁰⁰    ¹⁵˙⁰⁰    ¹⁸˙⁰⁰    ²¹˙⁰⁰    │
╰────────────────────────────────────────────────────────────────────────╯";

		let (lines, _) = prep(&test_product, &test_params, true).unwrap();
		let have = strip_ansi_escapes::strip_str(lines.join("\n"));
		assert_eq!(want, have);
	}
}

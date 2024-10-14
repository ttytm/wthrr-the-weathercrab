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

	let dt = Local::now() + Duration::days(day_index.try_into()?);
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
		&match &gui.border {
			BorderStyle::double => Separator::Double.fmt(WIDTH, gui.border),
			BorderStyle::solid => Separator::Solid.fmt(WIDTH, gui.border),
			_ => Separator::Single.fmt(WIDTH, gui.border),
		}
		.plain_or_bright_black(gui.color),
	));

	// Temperature & Weathercode
	let temperature_and_weathercode = format!(
		"{} {}, {} {}",
		wmo_code.icon, wmo_code.interpretation, temp_max_min, precipitation_probability_max
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

	// Hourly Forecast
	for line in hourly::prep(product, params, day_index)? {
		result.push(line);
	}

	// Border Bottom
	result.push(format!(
		"{}",
		Edge::Bottom.fmt(WIDTH, gui.border).plain_or_bright_black(gui.color)
	));

	Ok(result)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::modules::display::utils::common_tests::{
		init_test_params, init_test_product, TEST_PARAMS, TEST_PRODUCT,
	};
	use strip_ansi_escapes;

	#[test]
	fn output() {
		let test_product = TEST_PRODUCT.get_or_init(init_test_product);
		let test_params = TEST_PARAMS.get_or_init(init_test_params);

		let want = "\
╭────────────────────────────────────────────────────────────────────────╮
│ Berlin, Germany                                                        │
├────────────────────────────────────────────────────────────────────────┤
│  Slight Rain, 20.3/12.0°C ❲88󰖎❳                          Mon, 14 Oct │
│ Feels like 21.1/11.4°C                                 07:20   18:26 │
│                                                                        │
│ Hourly Forecast                                                        │
├┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┤
│ ₁₃      ₁₂      ₁₂      ₁₃      ₁₈      ₂₀      ₁₉      ₁₇    │
│                                                                        │
│                                    🭺🭹🭸🭷🭷🭶🭶🭶▔▔▔▔▔▔▔🭶🭶🭶🭷🭷🭹🭹🭹🭹🭺🭺🭺🭻🭻🭻▁▁▁   │
│🭻🭻🭻▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁🭻🭻🭻🭺🭹🭹🭸🭷🭷🭶▔▔▔                                 ▔▔▔│
│  ₀       ₂₀       ₈₈       ₁₃        ₃        ₀       ₁₃       ₇₀    󰖎 │
├┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┤
│⁰⁰˙⁰⁰    ⁰³˙⁰⁰    ⁰⁶˙⁰⁰    ⁰⁹˙⁰⁰    ¹²˙⁰⁰    ¹⁵˙⁰⁰    ¹⁸˙⁰⁰    ²¹˙⁰⁰    │
╰────────────────────────────────────────────────────────────────────────╯";

		// FIXME: index
		let lines = prep(test_product, test_params, 1).unwrap();
		let have = strip_ansi_escapes::strip_str(lines.join("\n"));
		assert_eq!(want, have);

		let want = "\
╭────────────────────────────────────────────────────────────────────────╮
│ Berlin, Germany                                                        │
├────────────────────────────────────────────────────────────────────────┤
│  Moderate Rain, 17.9/13.7°C ❲98󰖎❳                        Tue, 15 Oct │
│ Feels like 17.4/13.7°C                                 07:22   18:24 │
│                                                                        │
│ Hourly Forecast                                                        │
├┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┤
│ ₁₆      ₁₅      ₁₄      ₁₄      ₁₇      ₁₇      ₁₆      ₁₅    │
│                                                                        │
│                                    🭹🭸🭷🭸🭶🭸🭸🭸🭸🭸🭸🭸🭸🭸🭺🭺🭻🭻🭻🭻▁               │
│▔▔▔▔▔▔▔▔🭷🭸🭹🭹🭺🭺🭺🭺🭺🭺🭻🭻🭻▁▁▁▁▁▁🭻🭻🭺🭹🭹🭸🭶▔▔                     ▔▔▔🭶🭶🭸🭸🭸🭺🭺🭺🭺🭺🭺🭺│
│ ₈₃       ₉₈       ₆₈       ₃₀       ₁₇       ₁₉       ₂₇       ₂₂    󰖎 │
├┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┤
│⁰⁰˙⁰⁰    ⁰³˙⁰⁰    ⁰⁶˙⁰⁰    ⁰⁹˙⁰⁰    ¹²˙⁰⁰    ¹⁵˙⁰⁰    ¹⁸˙⁰⁰    ²¹˙⁰⁰    │
╰────────────────────────────────────────────────────────────────────────╯";

		let lines = prep(test_product, test_params, 2).unwrap();
		let have = strip_ansi_escapes::strip_str(lines.join("\n"));
		assert_eq!(want, have);
	}
}

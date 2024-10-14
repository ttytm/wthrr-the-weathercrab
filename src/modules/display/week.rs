use anyhow::Result;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use unicode_width::UnicodeWidthStr;

use crate::modules::{localization::Locales, params::Params};

use super::{
	border::{Border, BorderStyle, Edge, Separator},
	current::Dimensions,
	gui_config::ConfigurableColor,
	product::{Product, MIN_CELL_WIDTH, TOTAL_BORDER_PADDING},
	utils::pad_string_to_width,
	weathercode::WeatherCode,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct ForecastDay {
	pub date: String,
	pub weather: String,
	pub interpretation: String,
}

pub fn prep(product: &Product, params: &Params, current_dimensions: Option<Dimensions>) -> Result<Vec<String>> {
	let (lang, t) = (&params.config.language, &params.texts.weather);
	let mut days = Vec::new();
	let mut width: usize = 0;

	for (i, _) in product.weather.daily.time.iter().enumerate() {
		let time = &product.weather.daily.time[i];

		let dt = NaiveDate::parse_from_str(time, "%Y-%m-%d")?;
		let date = if lang == "en_US" || lang == "en" {
			dt.format("%a, %e %b").to_string()
		} else {
			Locales::localize_date(dt, lang)?
		};

		let weather_code = WeatherCode::resolve(product.weather.daily.weathercode[i], false, &t.weather_code)?;
		let weather = format!(
			"{} {:.1}{}/{:.1}{}",
			weather_code.icon,
			product.weather.daily.temperature_2m_max[i],
			product.weather.daily_units.temperature_2m_max,
			product.weather.daily.temperature_2m_min[i],
			product.weather.daily_units.temperature_2m_min,
		);
		let day_width = format!("{}{}{}", date, weather, weather_code.interpretation).width();
		if day_width > width {
			width = day_width;
		}

		let day: ForecastDay = {
			ForecastDay {
				date: date.to_string(),
				weather,
				interpretation: weather_code.interpretation,
			}
		};

		days.push(day);
	}

	// let forecast = self;
	let gui = &params.config.gui;

	let (mut width, mut cell_width) = (width + 10, MIN_CELL_WIDTH);
	if let Some(dims) = current_dimensions {
		cell_width = std::cmp::max(cell_width, dims.cell_width);
		width = std::cmp::max(width, dims.width);
	}
	let width_no_border_pad = width - TOTAL_BORDER_PADDING;

	let mut result = Vec::<String>::new();

	// Border Top
	result.push(format!(
		"{}",
		&Edge::Top.fmt(width, gui.border).plain_or_bright_black(gui.color)
	));

	let mut chunks = days.chunks(1).peekable();

	let mut n = 0;
	while let Some(_) = chunks.next() {
		let forecast_day = format!(
			"{}{}{}",
			pad_string_to_width(&days[n].date, cell_width),
			pad_string_to_width(
				&days[n].weather,
				width_no_border_pad - days[n].interpretation.width() - cell_width
			),
			days[n].interpretation,
		);
		result.push(format!(
			"{} {} {}",
			&Border::L.fmt(gui.border).plain_or_bright_black(gui.color),
			pad_string_to_width(&forecast_day, width_no_border_pad),
			&Border::R.fmt(gui.border).plain_or_bright_black(gui.color),
		));
		if chunks.peek().is_some() {
			result.push(format!(
				"{}",
				&match gui.border {
					BorderStyle::double => Separator::Double.fmt(width, gui.border),
					BorderStyle::solid => Separator::Solid.fmt(width, gui.border),
					_ => Separator::Dashed.fmt(width, gui.border),
				}
				.plain_or_bright_black(gui.color)
			));
		}

		n += 1;
	}

	// Border Bottom
	result.push(format!(
		"{}",
		Edge::Bottom.fmt(width, gui.border).plain_or_bright_black(gui.color)
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
╭──────────────────────────────────────────────────────╮
│ Mon,  7 Oct     15.1°C/6.8°C    Slight Rain Showers │
├┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┤
│ Tue,  8 Oct     20.3°C/12.0°C           Slight Rain │
├┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┤
│ Wed,  9 Oct     17.9°C/13.7°C         Moderate Rain │
├┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┤
│ Thu, 10 Oct     20.2°C/13.7°C           Slight Rain │
├┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┤
│ Fri, 11 Oct     13.2°C/7.9°C               Overcast │
├┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┤
│ Sat, 12 Oct     12.2°C/4.8°C               Overcast │
├┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┤
│ Sun, 13 Oct     10.9°C/5.6°C    Slight Rain Showers │
╰──────────────────────────────────────────────────────╯";

		let lines = prep(test_product, test_params, None).unwrap();
		let have = strip_ansi_escapes::strip_str(lines.join("\n"));
		assert_eq!(want, have);
	}
}

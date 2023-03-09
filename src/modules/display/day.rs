use anyhow::Result;
use chrono::{Duration, Local};
use colored::{Color::BrightBlack, Colorize};

use crate::modules::{display::hourly::WIDTH, localization::Locales, params::Params, units::Time};

use super::{
	border::*, gui_config::ColorOption, hourly::HourlyForecast, product::Product, utils::lang_len_diff,
	weathercode::WeatherCode,
};

pub struct Day {
	address: String,
	temp_max_min: String,
	apparent_temp_max_min: String,
	precipitation_probability_max: String,
	date: String,
	sunrise: String,
	sunset: String,
	wmo_code: WeatherCode,
}

impl Day {
	pub fn render(product: &Product, params: &Params, day_index: usize) -> Result<()> {
		let Day {
			address,
			temp_max_min,
			apparent_temp_max_min,
			precipitation_probability_max,
			date,
			sunrise,
			sunset,
			wmo_code,
		} = Self::prepare(product, params, day_index)?;

		let (gui, lang) = (&params.config.gui, &params.config.language);

		// Border Top
		println!("{}", &Edge::Top.fmt(WIDTH, &gui.border).color_option(BrightBlack, &gui.color));

		// Address / Title
		println!(
			"{} {: ^WIDTH$} {}",
			Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			address.bold(),
			Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			WIDTH = WIDTH - 2 - lang_len_diff(&address, lang)
		);

		// Separator
		println!(
			"{}",
			&match &gui.border {
				BorderStyle::double => Separator::Double.fmt(WIDTH, &gui.border),
				BorderStyle::solid => Separator::Solid.fmt(WIDTH, &gui.border),
				_ => Separator::Single.fmt(WIDTH, &gui.border),
			}
			.color_option(BrightBlack, &gui.color)
		);

		// Temperature & Weathercode
		let temperatur_and_weathercode = format!(
			"{} {}, {} {}",
			wmo_code.icon, wmo_code.interpretation, temp_max_min, precipitation_probability_max
		);
		println!(
			"{} {} {: >WIDTH$} {}",
			Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			temperatur_and_weathercode.bold(),
			date,
			Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			WIDTH = WIDTH
				- 3 - lang_len_diff(&wmo_code.interpretation, lang)
				- temperatur_and_weathercode.chars().count()
				- lang_len_diff(&date, lang)
		);

		// Apparent Temperature & Sun Rise & Sun Set
		let sunrise_and_sunset = format!("{}  {}", sunrise, sunset);
		println!(
			"{} {} {: >WIDTH$} {}",
			Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			apparent_temp_max_min,
			sunrise_and_sunset,
			Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			WIDTH = WIDTH
				- 3 - lang_len_diff(&params.texts.weather.feels_like, lang)
				- apparent_temp_max_min.chars().count()
		);

		// Hourly Forecast
		HourlyForecast::render(&product.weather, params, day_index)?;

		// Border Bottom
		println!("{}", Edge::Bottom.fmt(WIDTH, &gui.border).color_option(BrightBlack, &gui.color));

		Ok(())
	}

	fn prepare(product: &Product, params: &Params, day_index: usize) -> Result<Self> {
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
			_ => weather.daily.sunrise[day_index][11..16].to_string(),
		};
		let sunset = match params.config.units.time {
			Time::am_pm => format!("{}:{}pm", sunset_hour - 12, &weather.daily.sunset[day_index][14..16]),
			_ => weather.daily.sunset[day_index][11..16].to_string(),
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
			if !(lang == "en_US" || lang == "en") {
				Locales::localize_date(dt.into(), lang)?
			} else {
				dt.format("%a, %e %b").to_string()
			}
		);
		let sunrise = format!(" {sunrise}");
		let sunset = format!(" {sunset}");
		let wmo_code =
			WeatherCode::resolve(weather.daily.weathercode[day_index], night, &params.texts.weather.weather_code)?;

		Ok(Day {
			address,
			temp_max_min,
			apparent_temp_max_min,
			precipitation_probability_max,
			date,
			sunrise,
			sunset,
			wmo_code,
		})
	}
}

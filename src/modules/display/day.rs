use anyhow::Result;
use chrono::{Duration, Local};
use dialoguer::console::style;
use unicode_width::UnicodeWidthStr;

use crate::modules::{display::hourly::WIDTH, localization::Locales, params::Params, units::Time};

use super::{
	border::{Border, BorderStyle, Edge, Separator},
	gui_config::ConfigurableColor,
	hourly::HourlyForecast,
	product::Product,
	utils::pad_string_to_width,
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
	hourly_forecast: HourlyForecast,
}

impl Day {
	pub fn render(self, params: &Params) {
		let Self {
			address,
			temp_max_min,
			apparent_temp_max_min,
			precipitation_probability_max,
			date,
			sunrise,
			sunset,
			wmo_code,
			hourly_forecast,
		} = self;

		let gui = &params.config.gui;
		let width_no_border_pad = WIDTH - 2;

		// Border Top
		println!("{}", &Edge::Top.fmt(WIDTH, &gui.border).plain_or_bright_black(&gui.color),);

		// Address / Title
		println!(
			"{} {} {}",
			Border::L.fmt(&gui.border).plain_or_bright_black(&gui.color),
			style(pad_string_to_width(&address, width_no_border_pad)).bold(),
			Border::R.fmt(&gui.border).plain_or_bright_black(&gui.color),
		);

		// Separator
		println!(
			"{}",
			&match &gui.border {
				BorderStyle::double => Separator::Double.fmt(WIDTH, &gui.border),
				BorderStyle::solid => Separator::Solid.fmt(WIDTH, &gui.border),
				_ => Separator::Single.fmt(WIDTH, &gui.border),
			}
			.plain_or_bright_black(&gui.color),
		);

		// Temperature & Weathercode
		let temperature_and_weathercode = format!(
			"{} {}, {} {}",
			wmo_code.icon, wmo_code.interpretation, temp_max_min, precipitation_probability_max
		);
		println!(
			"{} {}{} {}",
			Border::L.fmt(&gui.border).plain_or_bright_black(&gui.color),
			style(pad_string_to_width(
				&temperature_and_weathercode,
				width_no_border_pad - date.width()
			))
			.bold(),
			date,
			Border::R.fmt(&gui.border).plain_or_bright_black(&gui.color),
		);

		// Apparent Temperature & Sun Rise & Sun Set
		let sunrise_and_sunset = format!("{sunrise}  {sunset}");
		println!(
			"{} {}{} {}",
			Border::L.fmt(&gui.border).plain_or_bright_black(&gui.color),
			pad_string_to_width(&apparent_temp_max_min, width_no_border_pad - sunrise_and_sunset.width()),
			sunrise_and_sunset,
			Border::R.fmt(&gui.border).plain_or_bright_black(&gui.color),
		);

		// Hourly Forecast
		hourly_forecast.render(params);

		// Border Bottom
		println!("{}", Edge::Bottom.fmt(WIDTH, &gui.border).plain_or_bright_black(&gui.color),);
	}

	pub fn prep(product: &Product, params: &Params, day_index: usize) -> Result<Self> {
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
		let hourly_forecast = HourlyForecast::prepare(product, params, day_index)?;

		Ok(Self {
			address,
			temp_max_min,
			apparent_temp_max_min,
			precipitation_probability_max,
			date,
			sunrise,
			sunset,
			wmo_code,
			hourly_forecast,
		})
	}
}

use anyhow::Result;
use colored::{Color::BrightBlack, Colorize};

use crate::modules::params::Params;

use super::{
	border::*,
	gui_config::ColorOption,
	hourly::HourlyForecast,
	product::{Product, MIN_WIDTH},
	utils::{lang_len_diff, Times},
	weathercode::WeatherCode,
	wind::WindDirection,
};

pub struct Current {
	address: String,
	temperature: String,
	apparent_temperature: String,
	humidity: String,
	dewpoint: String,
	wind: String,
	pressure: String,
	sunrise: String,
	sunset: String,
	wmo_code: WeatherCode,
	dimensions: Dimensions,
}

pub struct Dimensions {
	pub width: usize,
	pub cell_width: usize,
}

impl Current {
	pub fn render(product: &Product, params: &Params, add_hourly: bool) -> Result<Dimensions> {
		let Current {
			address,
			temperature,
			apparent_temperature,
			humidity,
			dewpoint,
			wind,
			pressure,
			sunrise,
			sunset,
			wmo_code,
			dimensions,
		} = Self::prepare(product, params, add_hourly)?;

		let Dimensions { width, cell_width } = dimensions;
		let (gui, lang) = (&params.config.gui, &params.config.language);

		// Border Top
		println!(
			"{}",
			&Edge::Top.fmt(width, &gui.border).color_option(BrightBlack, &gui.color)
		);

		// Address / Title
		println!(
			"{} {: ^width$} {}",
			Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			address.bold(),
			Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			width = width - 2 - lang_len_diff(&address, lang)
		);

		// Separator
		println!(
			"{}",
			&match &gui.border {
				BorderStyle::double => Separator::Double.fmt(width, &gui.border),
				BorderStyle::solid => Separator::Solid.fmt(width, &gui.border),
				_ => Separator::Single.fmt(width, &gui.border),
			}
			.color_option(BrightBlack, &gui.color)
		);

		// Temperature & Weathercode
		println!(
			"{} {: <width$} {}",
			Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			(wmo_code.icon.to_string() + " " + &wmo_code.interpretation + ", " + &temperature).bold(),
			Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			width = width - 2 - lang_len_diff(&wmo_code.interpretation, lang)
		);

		// Apparent Temperature
		println!(
			"{} {: <width$} {}",
			Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			apparent_temperature,
			Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			width = width - 2 - lang_len_diff(&apparent_temperature, lang)
            // manually account for displacepment of this row until improving the lang_len_diff regex
            + if &lang[..2] == "ja" || &lang[..2] == "ko" { 2 } else { 0 }
		);

		// Blank Line
		println!(
			"{}",
			Separator::Blank
				.fmt(width, &gui.border)
				.color_option(BrightBlack, &gui.color)
		);

		// Humidity & Dewpoint
		println!(
			"{} {: <width$} {}",
			Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			format!(
				"{: <cell_width$} {}",
				humidity,
				dewpoint,
				cell_width = cell_width - lang_len_diff(&humidity, lang)
			),
			Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			width = width - 2 - lang_len_diff(&humidity, lang) - lang_len_diff(&dewpoint, lang)
				+ if &lang[..2] == "ja" || &lang[..2] == "ko" { 3 } else { 0 }
		);

		// Wind & Pressure
		println!(
			"{} {: <cell_width$}{: <width$} {}",
			Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			wind,
			pressure,
			Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			width = width - 2 - cell_width
		);

		// Sunrise & Sunset
		println!(
			"{} {: <cell_width$}{: <width$} {}",
			Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			sunrise,
			sunset,
			Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			width = width - 2 - cell_width
		);

		// Hourly Forecast
		if add_hourly {
			HourlyForecast::render(&product.weather, params, 0)?;
		}

		// Border Bottom
		println!(
			"{}",
			Edge::Bottom
				.fmt(width, &gui.border)
				.color_option(BrightBlack, &gui.color)
		);

		Ok(dimensions)
	}

	fn prepare(product: &Product, params: &Params, add_hourly: bool) -> Result<Self> {
		let weather = &product.weather;
		let address = Product::trunc_address(product.address.clone(), 60);
		let t = &params.texts.weather;

		let Times { current_hour, sunrise, sunset, night } = product.weather.get_times(params.config.units.time, 0);

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
		let title_width = address.chars().count();
		let title_padding = 2 * 2; // 2 spaces on each side
		let longest_cell_width = humidity.chars().count();
		let dimensions = Dimensions {
			width: if add_hourly {
				super::hourly::WIDTH
			} else if title_width > MIN_WIDTH {
				title_width + title_padding
			} else {
				MIN_WIDTH + title_padding
			},
			cell_width: if add_hourly {
				22
			} else if longest_cell_width > MIN_WIDTH / 2 {
				// increase cell_width for languages with longer texts
				longest_cell_width + 2
			} else {
				MIN_WIDTH / 2
			},
		};

		Ok(Current {
			address,
			temperature,
			apparent_temperature,
			humidity,
			dewpoint,
			wind,
			pressure,
			sunrise,
			sunset,
			wmo_code,
			dimensions,
		})
	}
}

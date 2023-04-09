use anyhow::Result;
use colored::{Color::BrightBlack, Colorize};

use crate::modules::{params::Params, units::Time};

use super::{
	border::{Border, BorderStyle, Edge, Separator},
	gui_config::ColorOption,
	hourly::HourlyForecast,
	product::{Product, MIN_CELL_WIDTH, MIN_WIDTH},
	utils::lang_len_diff,
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
	hourly_forecast: Option<HourlyForecast>,
}

pub struct Dimensions {
	pub width: usize,
	pub cell_width: usize,
}

impl Current {
	pub fn render(self, params: &Params) -> Dimensions {
		let Self {
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
			hourly_forecast,
		} = self;

		let Dimensions { width, cell_width } = dimensions;
		let (gui, lang) = (&params.config.gui, &params.config.language);

		// Border Top
		println!("{}", &Edge::Top.fmt(width, &gui.border).color_option(BrightBlack, &gui.color));

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
			Separator::Blank.fmt(width, &gui.border).color_option(BrightBlack, &gui.color)
		);

		// Humidity & Dewpoint
		println!(
			"{} {: <width$} {}",
			Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			format!(
				"{: <cell_width$} {}",
				humidity,
				dewpoint,
				cell_width = cell_width
					- lang_len_diff(&humidity, lang)
					- if &lang[..2] == "ja" || &lang[..2] == "ko" { 0 } else { 1 }
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
		if let Some(forecast) = hourly_forecast {
			forecast.render(params);
		}

		// Border Bottom
		println!("{}", Edge::Bottom.fmt(width, &gui.border).color_option(BrightBlack, &gui.color));

		dimensions
	}

	pub fn prep(product: &Product, params: &Params, add_hourly: bool) -> Result<Self> {
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
			} else if longest_cell_width > MIN_CELL_WIDTH {
				// increase cell_width for languages with longer texts
				longest_cell_width
			} else {
				MIN_CELL_WIDTH + 2
			},
		};
		let hourly_forecast = if add_hourly {
			Some(HourlyForecast::prepare(product, params, 0)?)
		} else {
			None
		};

		Ok(Self {
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
			hourly_forecast,
		})
	}
}

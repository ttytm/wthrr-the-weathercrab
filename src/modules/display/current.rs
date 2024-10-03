use anyhow::Result;
use dialoguer::console::style;

use crate::modules::{params::Params, units::Time};

use super::{
	border::{Border, BorderStyle, Edge, Separator},
	gui_config::ConfigurableColor,
	hourly::HourlyForecast,
	product::{Product, MIN_CELL_WIDTH, MIN_WIDTH},
	utils::pad_string_to_width,
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

		let gui = &params.config.gui;
		let Dimensions { width, cell_width } = dimensions;
		let width_no_border_pad = width - 2;

		// Border Top
		println!("{}", &Edge::Top.fmt(width, &gui.border).plain_or_bright_black(&gui.color));

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
				BorderStyle::double => Separator::Double.fmt(width, &gui.border),
				BorderStyle::solid => Separator::Solid.fmt(width, &gui.border),
				_ => Separator::Single.fmt(width, &gui.border),
			}
			.plain_or_bright_black(&gui.color),
		);

		println!(
			"{} {} {}",
			Border::L.fmt(&gui.border).plain_or_bright_black(&gui.color),
			style(pad_string_to_width(
				&(wmo_code.icon.to_string() + " " + &wmo_code.interpretation + ", " + &temperature),
				width_no_border_pad
			))
			.bold(),
			Border::R.fmt(&gui.border).plain_or_bright_black(&gui.color),
		);

		// Apparent Temperature
		println!(
			"{} {} {}",
			Border::L.fmt(&gui.border).plain_or_bright_black(&gui.color),
			pad_string_to_width(&apparent_temperature, width_no_border_pad),
			Border::R.fmt(&gui.border).plain_or_bright_black(&gui.color),
		);

		// Blank Line
		println!("{}", Separator::Blank.fmt(width, &gui.border).plain_or_bright_black(&gui.color));

		// Humidity & Dewpoint
		println!(
			"{} {}{} {}",
			Border::L.fmt(&gui.border).plain_or_bright_black(&gui.color),
			pad_string_to_width(&humidity, cell_width),
			// NOTE: When using the Thai language, an apparent combining character issue was observed
			// with the dew point, resulting in the border being displaced by one space or the border
			// color being removed in some terminal/font configurations.
			pad_string_to_width(&dewpoint, width_no_border_pad - cell_width),
			Border::R.fmt(&gui.border).plain_or_bright_black(&gui.color),
		);

		// Wind & Pressure
		println!(
			"{} {: <cell_width$}{: <width$} {}",
			Border::L.fmt(&gui.border).plain_or_bright_black(&gui.color),
			wind,
			pressure,
			Border::R.fmt(&gui.border).plain_or_bright_black(&gui.color),
			width = width_no_border_pad - cell_width
		);

		// Sunrise & Sunset
		println!(
			"{} {: <cell_width$}{: <width$} {}",
			Border::L.fmt(&gui.border).plain_or_bright_black(&gui.color),
			sunrise,
			sunset,
			Border::R.fmt(&gui.border).plain_or_bright_black(&gui.color),
			width = width_no_border_pad - cell_width
		);

		// Hourly Forecast
		if let Some(forecast) = hourly_forecast {
			forecast.render(params);
		}

		// Border Bottom
		println!("{}", Edge::Bottom.fmt(width, &gui.border).plain_or_bright_black(&gui.color));

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

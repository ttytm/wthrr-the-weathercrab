use anyhow::Result;
use colored::{Color::BrightBlack, Colorize};

use crate::modules::{
	localization::WeatherLocales,
	units::{Time, Units},
};

use super::{
	border::*,
	graph::GraphOpts,
	gui_config::{ColorOption, Gui},
	hourly::HourlyForecast,
	product::{Product, MIN_WIDTH},
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
	sun_rise: String,
	sun_set: String,
	wmo_code: WeatherCode,
	hourly_forecast: Option<HourlyForecast>,
	dimensions: Dimensions,
}

pub struct Dimensions {
	pub width: usize,
	pub cell_width: usize,
}

impl Current {
	pub fn render(
		product: &Product,
		add_hourly: bool,
		units: &Units,
		gui: &Gui,
		lang: &str,
		t: &WeatherLocales,
	) -> Result<Dimensions> {
		let Current {
			address,
			temperature,
			apparent_temperature,
			humidity,
			dewpoint,
			wind,
			pressure,
			sun_rise,
			sun_set,
			wmo_code,
			hourly_forecast,
			dimensions,
		} = Self::prepare(product, add_hourly, units, &gui.graph, t)?;

		let Dimensions { width, cell_width } = dimensions;

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

		// Temperature
		println!(
			"{} {: <width$} {}",
			Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			(temperature + " " + &wmo_code.interpretation).bold(),
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
		);

		// Blank Line
		println!(
			"{}",
			Separator::Blank
				.fmt(width, &gui.border)
				.color_option(BrightBlack, &gui.color)
		);

		// Humidity & Dewpoint
		let humidity_dewpoint_split = format!(
			"{: <cell_width$}{}",
			humidity,
			dewpoint,
			cell_width = cell_width - lang_len_diff(&humidity, lang)
		);
		println!(
			"{} {: <width$} {}",
			Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			humidity_dewpoint_split,
			Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			width = width - 2 - lang_len_diff(&humidity, lang) - lang_len_diff(&dewpoint, lang)
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
			sun_rise,
			sun_set,
			Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			width = width - 2 - cell_width
		);

		// Hourly Forecast
		if let Some(hourly_forecast) = hourly_forecast {
			hourly_forecast.render(width, units, &gui.border, &gui.color, t)
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

	fn prepare(
		product: &Product,
		add_hourly: bool,
		units: &Units,
		graph_opts: &GraphOpts,
		t: &WeatherLocales,
	) -> Result<Self> {
		let weather = &product.weather;
		let address = Product::trunc_address(product.address.clone(), 60);

		let (current_hour, sunrise_hour, sunset_hour) = (
			weather.current_weather.time[11..13]
				.parse::<usize>()
				.unwrap_or_default(),
			weather.daily.sunrise[0][11..13].parse::<usize>().unwrap_or_default(),
			weather.daily.sunset[0][11..13].parse::<usize>().unwrap_or_default(),
		);
		let sunrise_time = match units.time {
			Time::am_pm => format!("{}:{}am", sunrise_hour, &weather.daily.sunrise[0][14..16]),
			_ => weather.daily.sunrise[0][11..16].to_string(),
		};
		let sunset_time = match units.time {
			Time::am_pm => format!("{}:{}pm", sunset_hour - 12, &weather.daily.sunset[0][14..16]),
			_ => weather.daily.sunset[0][11..16].to_string(),
		};
		let night = current_hour < sunrise_hour || current_hour > sunset_hour;

		let wmo_code = WeatherCode::resolve(&weather.current_weather.weathercode, night, &t.weather_code)?;

		let temperature = format!(
			"{} {}{}",
			wmo_code.icon, weather.current_weather.temperature, weather.hourly_units.temperature_2m
		);
		let apparent_temperature = format!(
			"{} {}{}",
			t.feels_like, weather.hourly.apparent_temperature[current_hour], weather.hourly_units.temperature_2m
		);
		let humidity = format!(
			"{}: {}{}",
			t.humidity, weather.hourly.relativehumidity_2m[current_hour], weather.hourly_units.relativehumidity_2m,
		);
		let dewpoint = format!(
			"{}: {}{}",
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
		let sun_rise = format!(" {sunrise_time}");
		let sun_set = format!(" {sunset_time}");

		// Dimensions
		let title_width = address.chars().count();
		let title_padding = 2 * 2; // 2 spaces on each side
		let longest_cell_width = humidity.chars().count();

		let dimensions = Dimensions {
			width: if add_hourly {
				72
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

		let hourly_forecast = match add_hourly {
			true => Some(HourlyForecast::prepare(
				weather,
				current_hour,
				night,
				graph_opts,
				units,
				&t.weather_code,
			)?),
			_ => None,
		};

		Ok(Current {
			address,
			temperature,
			apparent_temperature,
			humidity,
			dewpoint,
			wind,
			pressure,
			sun_rise,
			sun_set,
			wmo_code,
			hourly_forecast,
			dimensions,
		})
	}
}

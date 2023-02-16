use anyhow::Result;
use colored::{Color::BrightBlack, Colorize};

use crate::modules::{
	params::{
		gui::{ColorOption, ColorVariant},
		units::{Time, Units},
	},
	translation::translate,
};

use super::{
	border::*,
	graph::{Graph, GraphVariant},
	utils::adjust_lang_width,
	weathercode::WeatherCode,
	wind::WindDirection,
	Product, MIN_WIDTH,
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
	hourly_forecast: Option<Graph>,
	dimensions: Dimensions,
}

pub struct Dimensions {
	pub width: usize,
	pub cell_width: usize,
}

impl Current {
	pub async fn render(
		product: &Product,
		add_hourly: bool,
		units: &Units,
		border_variant: &BorderVariant,
		graph_variant: &GraphVariant,
		color_variant: &ColorVariant,
		lang: &str,
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
		} = Self::prepare(product, add_hourly, lang, units, graph_variant).await?;

		let Dimensions { width, cell_width } = dimensions;

		// Border Top
		println!(
			"{}",
			&Edge::Top
				.fmt(width, border_variant)
				.color_option(BrightBlack, color_variant)
		);

		// Address / Title
		println!(
			"{} {: ^width$} {}",
			Border::L.fmt(border_variant).color_option(BrightBlack, color_variant),
			address.bold(),
			Border::R.fmt(border_variant).color_option(BrightBlack, color_variant),
			width = width - 2 - adjust_lang_width(&address, lang)
		);

		// Separator
		println!(
			"{}",
			&match border_variant {
				BorderVariant::double => Separator::Double.fmt(width, border_variant),
				BorderVariant::solid => Separator::Solid.fmt(width, border_variant),
				_ => Separator::Single.fmt(width, border_variant),
			}
			.color_option(BrightBlack, color_variant)
		);

		// Temperature
		println!(
			"{} {: <width$} {}",
			Border::L.fmt(border_variant).color_option(BrightBlack, color_variant),
			(temperature + " " + &wmo_code.interpretation).bold(),
			Border::R.fmt(border_variant).color_option(BrightBlack, color_variant),
			width = width - 2 - adjust_lang_width(&wmo_code.interpretation, lang)
		);

		// Apparent Temperature
		println!(
			"{} {: <width$} {}",
			Border::L.fmt(border_variant).color_option(BrightBlack, color_variant),
			apparent_temperature,
			Border::R.fmt(border_variant).color_option(BrightBlack, color_variant),
			width = width - 2 - adjust_lang_width(&apparent_temperature, lang)
		);

		// Blank Line
		println!(
			"{}",
			Separator::Blank
				.fmt(width, border_variant)
				.color_option(BrightBlack, color_variant)
		);

		// Humidity & Dewpoint
		let humidity_dewpoint_split = format!(
			"{: <cell_width$}{}",
			humidity,
			dewpoint,
			cell_width = cell_width - adjust_lang_width(&humidity, lang)
		);
		println!(
			"{} {: <width$} {}",
			Border::L.fmt(border_variant).color_option(BrightBlack, color_variant),
			humidity_dewpoint_split,
			Border::R.fmt(border_variant).color_option(BrightBlack, color_variant),
			width = width - 2 - adjust_lang_width(&humidity, lang) - adjust_lang_width(&dewpoint, lang)
		);

		// Wind & Pressure
		println!(
			"{} {: <cell_width$}{: <width$} {}",
			Border::L.fmt(border_variant).color_option(BrightBlack, color_variant),
			wind,
			pressure,
			Border::R.fmt(border_variant).color_option(BrightBlack, color_variant),
			width = width - 2 - cell_width
		);

		// Sunrise & Sunset
		println!(
			"{} {: <cell_width$}{: <width$} {}",
			Border::L.fmt(border_variant).color_option(BrightBlack, color_variant),
			sun_rise,
			sun_set,
			Border::R.fmt(border_variant).color_option(BrightBlack, color_variant),
			width = width - 2 - cell_width
		);

		// Hourly Forecast
		if hourly_forecast.is_some() {
			hourly_forecast
				.unwrap()
				.render(width, units, border_variant, color_variant)
		}

		// Border Bottom
		println!(
			"{}",
			Edge::Bottom
				.fmt(width, border_variant)
				.color_option(BrightBlack, color_variant)
		);

		Ok(dimensions)
	}

	async fn prepare(
		product: &Product,
		add_hourly: bool,
		lang: &str,
		units: &Units,
		graph_variant: &GraphVariant,
	) -> Result<Self> {
		let weather = &product.weather;
		let address = Product::trunc_address(product.address.clone(), 60)?;

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

		let wmo_code = WeatherCode::resolve(&weather.current_weather.weathercode, Some(night), lang).await?;

		let temperature = format!(
			"{} {}{}",
			wmo_code.icon, weather.current_weather.temperature, weather.hourly_units.temperature_2m
		);
		let apparent_temperature = format!(
			"{} {}{}",
			translate(lang, "Feels like").await?,
			weather.hourly.apparent_temperature[current_hour],
			weather.hourly_units.temperature_2m
		);
		let humidity = format!(
			"{}: {}{}",
			translate(lang, "Humidity").await?,
			weather.hourly.relativehumidity_2m[current_hour],
			weather.hourly_units.relativehumidity_2m,
		);
		let dewpoint = format!(
			"{}: {}{}",
			translate(lang, "Dew Point").await?,
			weather.hourly.dewpoint_2m[current_hour],
			weather.hourly_units.dewpoint_2m
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
			true => Some(Graph::prepare(weather, current_hour, night, graph_variant, lang).await?),
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

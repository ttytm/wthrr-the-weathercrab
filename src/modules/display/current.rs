use anyhow::Result;
use term_painter::{Attr::Bold, Color::BrightBlack, ToStyle};

use crate::translation::translate;

use super::{
	border::{Border, Separator},
	weathercode::WeatherCode,
	wind::WindDirection,
	Product,
};

pub struct Current {
	title: String,
	temperature: String,
	apparent_temperature: String,
	humidity: String,
	dewpoint: String,
	wind: String,
	pressure: String,
	sun_time: String,
	wmo_code: WeatherCode,
	width: usize,
}

struct Dimensions {
	max_width: usize,
	min_width: usize,
	cell_width: usize,
}

impl Current {
	pub async fn render(product: &Product, lang: &str) -> Result<()> {
		let dims = Dimensions {
			max_width: 60,
			min_width: 34,
			cell_width: 17,
		};

		let Current {
			title,
			temperature,
			apparent_temperature,
			humidity,
			dewpoint,
			wind,
			pressure,
			sun_time,
			wmo_code,
			width,
		} = Self::prepare(product, &dims, lang).await?;

		// Border Top
		BrightBlack.with(|| println!("{}{}{} ", Border::TL, Border::T.to_string().repeat(width), Border::TR));

		// Title
		println!(
			"{} {: ^3$} {}",
			BrightBlack.paint(Border::L),
			// Bold.paint(title),
			title,
			BrightBlack.paint(Border::R),
			width - 2
		);

		BrightBlack.with(|| Separator::Line.fmt(width))?;

		// Temperature
		println!(
			"{} {} {}{}{}",
			BrightBlack.paint(Border::L),
			Bold.paint(&temperature),
			wmo_code.interpretation,
			" ".repeat(width - temperature.chars().count() - wmo_code.interpretation.chars().count() - 2),
			BrightBlack.paint(Border::R),
		);
		println!(
			"{} {: <3$}{}",
			BrightBlack.paint(Border::L),
			apparent_temperature,
			BrightBlack.paint(Border::R),
			width - 1
		);

		BrightBlack.with(|| Separator::Blank.fmt(width))?;

		let humidity_dewpoint_row = format!("{: <2$}{}", humidity, dewpoint, dims.cell_width);
		println!(
			"{} {: <3$}{}",
			BrightBlack.paint(Border::L),
			humidity_dewpoint_row,
			BrightBlack.paint(Border::R),
			width - 1
		);

		let wind_pressure_row = format!("{: <2$}{}", wind, pressure, dims.cell_width);
		println!(
			"{} {: <3$}{}",
			BrightBlack.paint(Border::L),
			wind_pressure_row,
			BrightBlack.paint(Border::R),
			width - 1
		);

		// Sun times
		println!(
			"{} {: <3$}{}",
			BrightBlack.paint(Border::L),
			sun_time,
			BrightBlack.paint(Border::R),
			width - 1
		);

		// Border Bottom
		BrightBlack.with(|| println!("{}{}{}", Border::BL, Border::B.to_string().repeat(width), Border::BR));

		Ok(())
	}

	async fn prepare(product: &Product, dims: &Dimensions, lang: &str) -> Result<Self> {
		let weather = &product.weather;
		let title = Product::check_address_len(product.address.clone(), dims.max_width)?;
		let title_len = title.chars().count();
		let width = (if title_len > dims.min_width {
			title_len
		} else {
			dims.min_width
		}) + 3 * 2;

		let (sunrise_time, sunset_time) = (&weather.daily.sunrise[0][11..16], &weather.daily.sunset[0][11..16]);
		let (current_hour, sunrise_hour, sunset_hour) = (
			weather.current_weather.time[11..13]
				.parse::<usize>()
				.unwrap_or_default(),
			sunrise_time[1..3].parse().unwrap_or_default(),
			sunset_time[1..3].parse().unwrap_or_default(),
		);
		let night = current_hour < sunrise_hour || current_hour > sunset_hour;
		let wmo_code = WeatherCode::resolve(&weather.current_weather.weathercode, Some(night), lang).await?;
		let wind_direction = WindDirection::get_direction(weather.current_weather.winddirection)?;

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

		let sun_time = format!(" {: <2$} {}", sunrise_time, sunset_time, dims.cell_width - 2);

		Ok(Current {
			title,
			temperature,
			apparent_temperature,
			humidity,
			dewpoint,
			wind,
			pressure,
			sun_time,
			wmo_code,
			width,
		})
	}
}

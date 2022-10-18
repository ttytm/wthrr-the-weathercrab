use anyhow::Result;
use regex::Regex;
use term_painter::{Attr::Bold, Color::BrightBlack, ToStyle};

use crate::translation::translate;

use super::{
	border::{Border, Separator},
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
	dimensions: Dimensions,
}

struct Dimensions {
	width: usize,
	cell_width: usize,
}

impl Current {
	pub async fn render(product: &Product, lang: &str) -> Result<usize> {
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
			dimensions,
		} = Self::prepare(product, lang).await?;

		let Dimensions { width, cell_width } = dimensions;

		// Border Top
		BrightBlack.with(|| println!("{}{}{}", Border::TL, Border::T.to_string().repeat(width), Border::TR));

		// Address / Title
		println!(
			"{} {: ^width$} {}",
			BrightBlack.paint(Border::L),
			Bold.paint(&address),
			BrightBlack.paint(Border::R),
			width = width - 2 - adjust_lang_width(&address, lang)
		);

		BrightBlack.with(|| println!("{}", Separator::Line.fmt(width)));

		// Temperature
		println!(
			"{} {: <width$} {}",
			BrightBlack.paint(Border::L),
			Bold.paint(temperature + " " + &wmo_code.interpretation),
			BrightBlack.paint(Border::R),
			width = width - 2 - adjust_lang_width(&wmo_code.interpretation, lang)
		);

		// Apparent Temperature
		println!(
			"{} {: <width$} {}",
			BrightBlack.paint(Border::L),
			apparent_temperature,
			BrightBlack.paint(Border::R),
			width = width - 2 - adjust_lang_width(&apparent_temperature, lang)
		);

		// Blank Line
		BrightBlack.with(|| println!("{}", Separator::Blank.fmt(width)));

		// Humidity & Dewpoint
		let humidity_dewpoint_split = format!(
			"{: <cell_width$}{}",
			humidity,
			dewpoint,
			cell_width = cell_width - adjust_lang_width(&humidity, lang)
		);
		println!(
			"{} {: <width$} {}",
			BrightBlack.paint(Border::L),
			humidity_dewpoint_split,
			BrightBlack.paint(Border::R),
			width = width - 2 - adjust_lang_width(&humidity, lang) - adjust_lang_width(&dewpoint, lang)
		);

		// Wind & Pressure
		println!(
			"{} {: <cell_width$}{: <width$} {}",
			BrightBlack.paint(Border::L),
			wind,
			pressure,
			BrightBlack.paint(Border::R),
			width = width - 2 - cell_width
		);

		// Sunrise & Sunset
		println!(
			"{} {: <cell_width$}{: <width$} {}",
			BrightBlack.paint(Border::L),
			sun_rise,
			sun_set,
			BrightBlack.paint(Border::R),
			width = width - 2 - cell_width
		);

		// Border Bottom
		BrightBlack.with(|| println!("{}{}{}", Border::BL, Border::B.to_string().repeat(width), Border::BR));

		Ok(cell_width)
	}

	async fn prepare(product: &Product, lang: &str) -> Result<Self> {
		let weather = &product.weather;
		let address = Product::trunc_address(product.address.clone(), 60)?;

		let (sunrise_time, sunset_time) = (&weather.daily.sunrise[0][11..16], &weather.daily.sunset[0][11..16]);
		let (current_hour, sunrise_hour, sunset_hour) = (
			weather.current_weather.time[11..13]
				.parse::<usize>()
				.unwrap_or_default(),
			sunrise_time[..2].parse().unwrap_or_default(),
			sunset_time[..2].parse().unwrap_or_default(),
		);
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
		let sun_rise = format!(" {}", sunrise_time);
		let sun_set = format!(" {}", sunset_time);

		// Dimensions
		let full_width = address.chars().count();
		let mut dimensions = Dimensions {
			// add 2 spaces on each side of the address
			width: (if full_width > MIN_WIDTH { full_width } else { MIN_WIDTH }) + (2 * 2),
			cell_width: MIN_WIDTH / 2,
		};
		// adjust cell_width for languages with longer texts
		let humidity_len = humidity.chars().count();
		if humidity_len > dimensions.cell_width - 2 {
			dimensions.cell_width = humidity_len + 2
		}

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
			dimensions,
		})
	}
}

fn adjust_lang_width(string: &str, lang: &str) -> usize {
	let correction = match lang {
		"zh" => {
			let re = Regex::new(r"\p{han}").unwrap();
			re.find_iter(string).count()
		}
		"ko" => {
			let re = Regex::new(r"[\u3131-\uD79D\w]").unwrap();
			let nu = Regex::new(r"[0-9\.]").unwrap();
			re.find_iter(string).count() - nu.find_iter(string).count()
		}
		"ja" => {
			let re = Regex::new(r"[ぁ-んァ-ン\w]").unwrap();
			let nu = Regex::new(r"[0-9\.]").unwrap();
			re.find_iter(string).count() - nu.find_iter(string).count()
		}
		_ => 0,
	};

	correction
}

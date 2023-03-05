use anyhow::Result;
use chrono::{Timelike, Utc};
use colored::{
	Color::{Blue, BrightBlack, Yellow},
	Colorize,
};
use std::fmt::Write as _;

use crate::modules::{
	localization::WeatherCodeLocales,
	params::Params,
	units::{Precipitation, Temperature, Time},
	weather::Weather,
};

use super::{
	border::*,
	graph::Graph,
	gui_config::ColorOption,
	utils::{style_number, Times},
	weathercode::WeatherCode,
};

const DISPLAY_HOURS: [usize; 8] = [0, 3, 6, 9, 12, 15, 18, 21];
pub const WIDTH: usize = 72;

pub struct HourlyForecast {
	temperatures: String,
	precipitation: String,
	temp_max_min: String,
	precipitation_probability_max: u8,
	graph: Graph,
	time_indicator_col: Option<usize>,
}

impl HourlyForecast {
	pub fn render(weather: &Weather, params: &Params) -> Result<()> {
		let HourlyForecast {
			temperatures,
			precipitation,
			temp_max_min,
			precipitation_probability_max,
			graph,
			time_indicator_col,
		} = Self::prepare(weather, params)?;

		let (units, gui, t) = (&params.config.units, &params.config.gui, &params.texts.weather);

		println!(
			"{}",
			&Separator::Blank
				.fmt(WIDTH, &gui.border)
				.color_option(BrightBlack, &gui.color)
		);

		let temperature_unit = match units.temperature {
			Temperature::fahrenheit => "",
			_ => "",
		};
		let precipitation_unit = match units.precipitation {
			Precipitation::mm => "ₘₘ",
			Precipitation::inch => "ᵢₙ",
			_ => "󰖎",
		};

		println!(
			"{} {: <WIDTH$} {}",
			Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			t.hourly_forecast.bold(),
			Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			WIDTH = WIDTH - 2
		);
		println!(
			"{} {} ❲{}{}❳{: <WIDTH$} {}",
			Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			temp_max_min,
			precipitation_probability_max,
			"󰖎".bold(),
			"",
			Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			WIDTH = WIDTH - 5 - temp_max_min.len() - precipitation_probability_max.to_string().len()
		);

		match time_indicator_col {
			Some(col) => {
				println!(
					"{}",
					Self::prepare_separator(col, &gui.border, WIDTH, '╤').color_option(BrightBlack, &gui.color),
				);
			}
			_ => {
				println!(
					"{}",
					Separator::Dashed
						.fmt(WIDTH, &gui.border)
						.color_option(BrightBlack, &gui.color)
				);
			}
		}

		println!(
			"{} {: <WIDTH$}{} {}",
			Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			temperatures.color_option(Yellow, &gui.color).bold(),
			temperature_unit.color_option(Yellow, &gui.color).bold(),
			Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			WIDTH = WIDTH - 3
		);
		println!(
			"{}",
			&Separator::Blank
				.fmt(WIDTH, &gui.border)
				.color_option(BrightBlack, &gui.color)
		);

		if graph.1.chars().count() > 0 {
			println!(
				"{}{}{}",
				Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
				graph.1.color_option(Yellow, &gui.color),
				Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color)
			);
		}
		println!(
			"{}{}{}",
			Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			graph.0.color_option(Yellow, &gui.color),
			Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color)
		);

		println!(
			"{} {: <WIDTH$}{} {}",
			Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			precipitation.color_option(Blue, &gui.color).bold(),
			if units.precipitation == Precipitation::probability {
				// to enlarge the water percent icon we use bold as a hack
				precipitation_unit.color_option(Blue, &gui.color).bold()
			} else {
				precipitation_unit.color_option(Blue, &gui.color)
			},
			Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			WIDTH = WIDTH - 3
		);

		match time_indicator_col {
			Some(col) => {
				println!(
					"{}",
					Self::prepare_separator(col, &gui.border, WIDTH, '╧').color_option(BrightBlack, &gui.color),
				);
			}
			_ => {
				println!(
					"{}",
					Separator::Dashed
						.fmt(WIDTH, &gui.border)
						.color_option(BrightBlack, &gui.color)
				);
			}
		}

		let hours = match units.time {
			Time::am_pm => ["¹²·⁰⁰ₐₘ", "³·⁰⁰ₐₘ", "⁶˙⁰⁰ₐₘ", "⁹˙⁰⁰ₐₘ", "¹²˙⁰⁰ₚₘ", "³˙⁰⁰ₚₘ", "⁶˙⁰⁰ₚₘ", "⁹˙⁰⁰ₚₘ"],
			_ => ["⁰⁰˙⁰⁰", "⁰³˙⁰⁰", "⁰⁶˙⁰⁰", "⁰⁹˙⁰⁰", "¹²˙⁰⁰", "¹⁵˙⁰⁰", "¹⁸˙⁰⁰", "²¹˙⁰⁰"],
		};
		print!("{}", Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color));
		for hour in hours {
			print!("{hour: <9}")
		}
		println!("{}", Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color));

		Ok(())
	}

	pub fn prepare(weather: &Weather, params: &Params) -> Result<Self> {
		let (temperatures, weather_codes, precipitation);
		let Times { current_hour, night, .. } = weather.get_times(params.config.units.time);

		// The graph splits one hour into three "levels": last, current and next.
		// We slice 25 items to use the 25th in the last "next"-level of a graph.
		// If it's the end of one day we show the weather of the next day
		if current_hour != 23 {
			temperatures = &weather.hourly.temperature_2m[..=24];
			weather_codes = &weather.hourly.weathercode[..=24];
			precipitation = match params.config.units.precipitation {
				Precipitation::probability => {
					Self::prepare_precipitation_probability(&weather.hourly.precipitation_probability[..=24])
				}
				_ => Self::prepare_precipitation(&weather.hourly.precipitation[..=24]),
			};
		} else {
			temperatures = &weather.hourly.temperature_2m[25..=49];
			weather_codes = &weather.hourly.weathercode[25..=49];
			precipitation = match params.config.units.precipitation {
				Precipitation::probability => {
					Self::prepare_precipitation_probability(&weather.hourly.precipitation_probability[..=24])
				}
				_ => Self::prepare_precipitation(&weather.hourly.precipitation[25..=49]),
			};
		};

		let time_indicator_col = match params.config.gui.graph.time_indicator {
			true => Some(
				// add 3 cols to adjust to the multiple chars used to display the current hour below the chart
				if current_hour != 23 { (current_hour * 3) + 3 } else { 1 }
					+ (Timelike::minute(&Utc::now()) / 20) as usize,
			),
			_ => None,
		};

		let temp_max_min = format!(
			"{:.1}/{:.1}{}",
			weather.daily.temperature_2m_max[0],
			weather.daily.temperature_2m_min[0],
			weather.hourly_units.temperature_2m,
		);

		let precipitation_probability_max = weather.daily.precipitation_probability_max[0];

		Ok(HourlyForecast {
			temperatures: Self::prepare_temperatures(
				temperatures,
				weather_codes,
				night,
				&params.texts.weather.weather_code,
			)?,
			precipitation,
			temp_max_min,
			precipitation_probability_max,
			graph: Graph::prepare_graph(temperatures, &params.config.gui.graph),
			time_indicator_col,
		})
	}

	fn prepare_temperatures(
		temperatures: &[f32],
		weather_codes: &[u8],
		night: bool,
		t: &WeatherCodeLocales,
	) -> Result<String> {
		let mut result = String::new();

		for hour in DISPLAY_HOURS {
			let temp = temperatures[hour].round() as i32;
			let temp_sub = style_number(temp, true);
			let wmo_code = WeatherCode::resolve(weather_codes[hour], night, t)?;
			let colspan = if hour == 0 { 2 } else { 8 };
			let _ = write!(result, "{: >colspan$}{}", temp_sub, wmo_code.icon);
		}

		Ok(result)
	}

	// TODO: make precipitation fns generic by chance
	fn prepare_precipitation(precipitation: &[f32]) -> String {
		let mut result = String::new();

		for hour in DISPLAY_HOURS {
			let precipitation_sup = style_number(precipitation[hour].ceil() as i32, true);
			let colspan = if hour == 0 { 2 } else { 8 };
			let _ = write!(result, "{precipitation_sup: >colspan$} ");
		}

		result
	}

	fn prepare_precipitation_probability(precipitation: &[u8]) -> String {
		let mut result = String::new();

		for hour in DISPLAY_HOURS {
			let precipitation_sup = style_number(precipitation[hour].into(), true);
			let colspan = if hour == 0 { 2 } else { 8 };
			let _ = write!(result, "{precipitation_sup: >colspan$} ");
		}

		result
	}

	fn prepare_separator(
		time_indicator_col: usize,
		border_variant: &BorderStyle,
		width: usize,
		time_indicator_glyph: char,
	) -> String {
		match border_variant {
			BorderStyle::double => format!(
				"╟{:─>time_indicator_col$}{:─>width$}╢",
				time_indicator_glyph,
				"",
				width = width - time_indicator_col
			),
			BorderStyle::solid => format!(
				"┠{:─>time_indicator_col$}{:─>width$}┨",
				time_indicator_glyph,
				"",
				width = width - time_indicator_col
			),
			_ => format!(
				"├{:┈>time_indicator_col$}{:┈>width$}┤",
				time_indicator_glyph,
				"",
				width = width - time_indicator_col
			),
		}
	}
}

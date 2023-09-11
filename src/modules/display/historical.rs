use anyhow::Result;
use chrono::NaiveDate;
use colored::{Color::BrightBlack, Colorize};

use crate::modules::{
	config::Config,
	display::{graph::GraphOpts, gui_config::Gui, hourly::WIDTH},
	localization::Locales,
	params::Params,
	units::{Precipitation, Time, Units},
};

use super::{
	border::{Border, BorderStyle, Edge, Separator},
	gui_config::ColorOption,
	hourly::HourlyForecast,
	product::Product,
	utils::lang_len_diff,
	weathercode::WeatherCode,
};

pub struct HistoricalWeather {
	address: String,
	temp_max_min: String,
	apparent_temp_max_min: String,
	precipitation_sum: String,
	date: String,
	sunrise: String,
	sunset: String,
	wmo_code: WeatherCode,
	hourly_forecast: HourlyForecast,
}

impl HistoricalWeather {
	pub fn render(self, params: &Params) {
		let Self {
			address,
			temp_max_min,
			apparent_temp_max_min,
			precipitation_sum,
			date,
			sunrise,
			sunset,
			wmo_code,
			hourly_forecast,
		} = self;

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
		let temperature_and_weathercode = format!(
			"{} {}, {} {}",
			wmo_code.icon, wmo_code.interpretation, temp_max_min, precipitation_sum
		);
		println!(
			"{} {} {: >WIDTH$} {}",
			Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			temperature_and_weathercode.bold(),
			date,
			Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			WIDTH = WIDTH
				- 3 - lang_len_diff(&wmo_code.interpretation, lang)
				- temperature_and_weathercode.chars().count()
				- lang_len_diff(&date, lang)
		);

		// Apparent Temperature & Sun Rise & Sun Set
		let sunrise_and_sunset = format!("{sunrise}  {sunset}");
		println!(
			"{} {} {: >WIDTH$} {}",
			Border::L.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			apparent_temp_max_min,
			sunrise_and_sunset,
			Border::R.fmt(&gui.border).color_option(BrightBlack, &gui.color),
			WIDTH = WIDTH
				- 3 - lang_len_diff(&params.texts.weather.felt_like, lang)
				- apparent_temp_max_min.chars().count()
		);

		// Hourly Overview
		// For now, we use this more expensive approach of cloning parameters for historical forecasts
		let params = Params {
			config: Config {
				gui: Gui {
					graph: GraphOpts {
						time_indicator: false,
						..params.config.gui.graph
					},
					..params.config.gui
				},
				units: Units {
					precipitation: Precipitation::mm,
					..params.config.units
				},
				..params.config.clone()
			},
			..params.clone()
		};
		hourly_forecast.render(&params);

		// Border Bottom
		println!("{}", Edge::Bottom.fmt(WIDTH, &gui.border).color_option(BrightBlack, &gui.color));
	}

	pub fn prep(product: &Product, params: &Params, date: &NaiveDate) -> Result<Self> {
		let address = Product::trunc_address(product.address.clone(), 60);

		// Helpers
		let weather = &product.historical_weather.as_ref().unwrap()[date];
		let weather_daily = weather.daily.as_ref().unwrap();
		let weather_daily_units = weather.daily_units.as_ref().unwrap();
		let lang = &params.config.language;
		// Times
		let sunrise = weather_daily.sunrise.as_ref().unwrap();
		let sunset = weather_daily.sunset.as_ref().unwrap();
		let (sunrise_hour, sunset_hour) = (
			sunrise[0][11..13].parse::<usize>().unwrap_or_default(),
			sunset[0][11..13].parse::<usize>().unwrap_or_default(),
		);

		// Display Items
		let sunrise = match params.config.units.time {
			Time::am_pm => format!("{}:{}am", sunrise_hour, &sunrise[0][14..16]),
			Time::military => sunrise[0][11..16].to_string(),
		};
		let sunset = match params.config.units.time {
			Time::am_pm => format!("{}:{}pm", sunset_hour - 12, &sunset[0][14..16]),
			Time::military => sunset[0][11..16].to_string(),
		};
		let temp_max_min = format!(
			"{:.1}/{:.1}{}",
			weather_daily.temperature_2m_max.as_ref().unwrap()[0],
			weather_daily.temperature_2m_min.as_ref().unwrap()[0],
			weather_daily_units.temperature_2m_max,
		);
		let apparent_temp_max_min = format!(
			"{} {:.1}/{:.1}{}",
			params.texts.weather.felt_like,
			weather_daily.apparent_temperature_max.as_ref().unwrap()[0],
			weather_daily.apparent_temperature_min.as_ref().unwrap()[0],
			weather_daily_units.temperature_2m_max,
		);
		let precipitation_sum = format!(
			"❲{}{}❳",
			weather_daily.precipitation_sum.as_ref().unwrap()[0],
			if params.config.units.precipitation == Precipitation::inch {
				"ᵢₙ"
			} else {
				"ₘₘ"
			}
		);
		let date = format!(
			" {}",
			if lang == "en_US" || lang == "en" {
				date.format("%a, %-d %b %Y").to_string()
			} else {
				Locales::localize_date(*date, lang)?
			}
		);
		let sunrise = format!(" {sunrise}");
		let sunset = format!(" {sunset}");
		let wmo_code = WeatherCode::resolve(
			weather.daily.as_ref().unwrap().weathercode.as_ref().unwrap()[0],
			false,
			&params.texts.weather.weather_code,
		)?;
		let hourly_forecast = HourlyForecast::prepare_historical(weather, params)?;

		Ok(Self {
			address,
			temp_max_min,
			apparent_temp_max_min,
			precipitation_sum,
			date,
			sunrise,
			sunset,
			wmo_code,
			hourly_forecast,
		})
	}
}

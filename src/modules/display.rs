use anyhow::Result;
use colored::Color::BrightBlack;
use regex::Regex;

use crate::{args::Forecast as ForecastParams, config::Gui, params::units::Units, weather::Weather};

use self::{current::Current, forecast::Forecast, utils::ColorOption};

pub mod border;
pub mod graph;
pub mod hourly;

mod current;
mod forecast;
mod greeting;
mod utils;
mod weathercode;
mod wind;

pub struct Product {
	pub address: String,
	pub weather: Weather,
}

pub const MIN_WIDTH: usize = 34;

impl Product {
	pub async fn render(&self, forecast: &[ForecastParams], units: &Units, gui: &Gui, lang: &str) -> Result<()> {
		greeting::render(gui.greeting.unwrap_or_else(|| Gui::default().greeting.unwrap()), lang).await?;

		if !forecast.is_empty() {
			Forecast::render(
				self,
				forecast,
				units,
				&gui.border.unwrap_or_default(),
				&gui.graph.unwrap_or_default(),
				&gui.color.unwrap_or_default(),
				lang,
			)
			.await?;
		} else {
			Current::render(
				self,
				false,
				units,
				&gui.border.unwrap_or_default(),
				&gui.graph.unwrap_or_default(),
				&gui.color.unwrap_or_default(),
				lang,
			)
			.await?;
		}

		// Disclaimer
		println!(
			" {}",
			"Weather data by Open-Meteo.com\n".color_option(BrightBlack, &gui.color.unwrap_or_default())
		);

		Ok(())
	}

	pub fn trunc_address(mut address: String, max_width: usize) -> Result<String> {
		let address_len = address.chars().count();

		address = if address_len > max_width {
			// For many places with overlong names the results seem better when partially removing text
			// between first and second comma instead of removing it between penultimate and last comma
			// let last_comma = title.matches(',').count();
			let prep_re = format!("^((?:[^,]*,){{{}}})[^,]*,(.*)", 1);
			let re = Regex::new(&prep_re).unwrap();

			re.replace(&address, "$1$2").to_string()
		} else {
			address
		};

		if address_len > max_width {
			address = Self::trunc_address(address, max_width)?;
		}

		Ok(address)
	}
}

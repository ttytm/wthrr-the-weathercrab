use anyhow::Result;
use regex::Regex;
use term_painter::{
	Color::{BrightBlack, NotSet},
	ToStyle,
};

use crate::{args::Forecast as ForecastArgs, weather::Weather};

use self::{current::Current, forecast::Forecast};

mod border;
mod current;
mod forecast;
mod greeting;
mod weathercode;
mod wind;

pub struct Product {
	pub address: String,
	pub weather: Weather,
}

const MAX_WIDTH: usize = 60;
pub const MIN_WIDTH: usize = 34;

impl Product {
	pub async fn render(&self, forecast: &Option<ForecastArgs>, include_greeting: bool, lang: &str) -> Result<()> {
		greeting::render(include_greeting, lang).await?;

		if forecast.is_some() {
			if forecast.as_ref().unwrap().week {
				Forecast::render(self, lang, None).await?;
			} else {
				let cell_width = Current::render(self, lang).await?;
				Forecast::render(self, lang, Some(cell_width)).await?;
			}
		} else {
			Current::render(self, lang).await?;
		}

		// Disclaimer
		BrightBlack.with(|| println!(" Weather data by Open-Meteo.com"));

		// Reset colors
		NotSet.with(|| println!());

		Ok(())
	}

	pub fn check_address_len(mut address: String) -> Result<String> {
		let address_len = address.chars().count();
		address = if address_len > MAX_WIDTH {
			Self::trunc_address(address)?
		} else {
			address
		};
		if address_len > MAX_WIDTH {
			address = Self::check_address_len(address)?;
		}
		Ok(address)
	}

	fn trunc_address(address: String) -> Result<String> {
		// let address_commas = title.matches(',').count();
		// For many places with overlong names the results seem better when partially removing text
		// between first and second comma instead of removing it between penultimate and last comma

		let prep_re = format!("^((?:[^,]*,){{{}}})[^,]*,(.*)", 1);
		let re = Regex::new(&prep_re).unwrap();
		let truncated_address = re.replace(&address, "$1$2").to_string();

		Ok(truncated_address)
	}
}

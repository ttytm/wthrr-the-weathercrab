use anyhow::Result;
use term_painter::{Color::*, ToStyle};
use regex::Regex;

use crate::weather::Weather;

use self::{current::Current, forecast::Forecast};

mod border;
mod current;
mod forecast;
mod weathercode;
mod wind;

pub struct Product {
	pub address: String,
	pub weather: Weather,
}

impl Product {
	pub async fn render(&self, include_forecast: bool, lang: &str) -> Result<()> {
		Current::render(self, lang).await?;

		if include_forecast {
			Forecast::render_forecast(self, lang).await?;
		}

		// Disclaimer
		BrightBlack.with(|| println!(" Weather data by Open-Meteo.com"));

		// Reset colors
		NotSet.with(|| println!());

		Ok(())
	}

	pub fn check_address_len(title: String, max_width: usize) -> Result<String> {
		let title_len = title.chars().count();
		let mut new_title = if title_len > max_width {
			Self::trunc_address(title)?
		} else {
			title
		};
		if title_len > max_width {
			new_title = Self::check_address_len(new_title, max_width)?;
		}
		Ok(new_title)
	}

	pub fn trunc_address(title: String) -> Result<String> {
		// let title_commas = title.matches(',').count();
		// For many places with overlong names the results seem better when partially removing text
		// between first and second comma instead of removing it between penultimate and last comma

		let prep_re = format!("^((?:[^,]*,){{{}}})[^,]*,(.*)", 1);
		let re = Regex::new(&prep_re).unwrap();
		let truncated_title = re.replace(&title, "$1$2").to_string();

		Ok(truncated_title)
	}
}

use anyhow::Result;
use term_painter::{Color::*, ToStyle};

use crate::Weather;
use {current::Current, forecast::Forecast};

mod border;
mod current;
mod forecast;
mod weathercode;
mod wind;

pub struct Product {
	pub weather: Weather,
	pub address: String,
}

impl Product {
	pub async fn render(&self, add_forecast: bool, lang: &str) -> Result<()> {
		Current::render(self, lang).await?;

		if add_forecast {
			Forecast::render_forecast(self, lang).await?;
		}

		// Disclaimer
		BrightBlack.with(|| println!(" Weather data by Open-Meteo.com"));

		// Reset colors
		NotSet.with(|| println!());

		Ok(())
	}
}

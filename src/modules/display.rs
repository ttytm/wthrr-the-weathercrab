use anyhow::Result;
use term_painter::{Color::*, ToStyle};

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
}

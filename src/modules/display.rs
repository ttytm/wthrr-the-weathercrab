use anyhow::Result;
use term_painter::{Color::*, ToStyle};

use crate::Product;
use {current::Current, forecast::Forecast};

mod border;
mod current;
mod forecast;
mod weathercode;
mod wind;

pub fn render(product: &Product, add_forecast: bool) -> Result<()> {
	Current::render(product)?;

	if add_forecast {
		Forecast::render_forecast(product)?;
	}

	// Disclaimer
	BrightBlack.with(|| println!(" Weather data by Open-Meteo.com"));

	// Reset colors
	NotSet.with(|| println!());

	Ok(())
}

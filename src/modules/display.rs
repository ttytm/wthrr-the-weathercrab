use anyhow::Result;
use term_painter::{Color::*, ToStyle};

use crate::display::current::Current;
use crate::display::forecast::Forecast;
use crate::weather::Weather;

mod border;
mod current;
mod forecast;
mod weathercode;
mod wind;

pub fn render(data: &Weather, city: String, add_forecast: bool) -> Result<()> {
	Current::render(data, city)?;

	if add_forecast {
		Forecast::render_forecast(data)?;
	}

	// Disclaimer
	BrightBlack.with(|| println!(" Weather data by Open-Meteo.com"));

	// Reset colors
	NotSet.with(|| println!());

	Ok(())
}

use anyhow::{Context, Result};
use serde::Deserialize;

use super::units::{Precipitation, Units};

// Open meteo json
// E.g., London:
// https://api.open-meteo.com/v1/forecast?latitude=51.5002&longitude=-0.1262&hourly=temperature_2m,relativehumidity_2m,apparent_temperature,surface_pressure,windspeed_10m,precipitation,weathercode&daily=weathercode,sunrise,sunset,winddirection_10m_dominant,temperature_2m_max,temperature_2m_min&current_weather=true&timezone=auto
#[derive(Deserialize, Debug)]
pub struct Weather {
	pub latitude: f64,
	pub longitude: f64,
	pub generationtime_ms: f64,
	pub utc_offset_seconds: i32,
	pub elevation: f32,
	pub current_weather: CurrentWeather,
	pub hourly_units: HourlyUnits,
	pub hourly: Hourly,
	pub daily_units: DailyUnits,
	pub daily: Daily,
}

#[derive(Deserialize, Debug)]
pub struct CurrentWeather {
	pub temperature: f32,
	pub windspeed: f32,
	pub winddirection: f32,
	pub weathercode: u8,
	pub time: String,
}

#[derive(Deserialize, Debug)]
pub struct HourlyUnits {
	pub temperature_2m: String,
	pub relativehumidity_2m: String,
	pub apparent_temperature: String,
	pub surface_pressure: String,
	pub dewpoint_2m: String,
	pub windspeed_10m: String,
	pub precipitation: String,
}

#[derive(Deserialize, Debug)]
pub struct Hourly {
	pub time: Vec<String>,
	pub temperature_2m: Vec<f32>,
	pub relativehumidity_2m: Vec<f32>,
	pub apparent_temperature: Vec<f32>,
	pub surface_pressure: Vec<f32>,
	pub dewpoint_2m: Vec<f32>,
	pub windspeed_10m: Vec<f32>,
	pub precipitation: Vec<f32>,
	pub precipitation_probability: Vec<u8>,
	pub weathercode: Vec<u8>,
}

#[derive(Deserialize, Debug)]
pub struct DailyUnits {
	pub time: String,
	pub weathercode: String,
	pub sunrise: String,
	pub sunset: String,
	pub winddirection_10m_dominant: String,
	pub temperature_2m_max: String,
	pub temperature_2m_min: String,
}

#[derive(Deserialize, Debug)]
pub struct Daily {
	pub time: Vec<String>,
	pub weathercode: Vec<u8>,
	pub sunrise: Vec<String>,
	pub sunset: Vec<String>,
	pub winddirection_10m_dominant: Vec<u16>,
	pub temperature_2m_max: Vec<f32>,
	pub temperature_2m_min: Vec<f32>,
	pub apparent_temperature_max: Vec<f32>,
	pub apparent_temperature_min: Vec<f32>,
	pub precipitation_probability_max: Vec<u8>,
}

impl Weather {
	pub async fn get(lat: f64, lon: f64, units: &Units) -> Result<Weather> {
		// TODO: conditionally expand api call
		let url = format!(
			"https://api.open-meteo.com/v1/forecast?
latitude={}
&longitude={}
&hourly=temperature_2m,relativehumidity_2m,apparent_temperature,surface_pressure,dewpoint_2m,windspeed_10m,weathercode,precipitation,precipitation_probability
&daily=weathercode,sunrise,sunset,winddirection_10m_dominant,temperature_2m_max,temperature_2m_min,precipitation_probability_max,apparent_temperature_max,apparent_temperature_min
&current_weather=true
&temperature_unit={}
&windspeed_unit={}
&precipitation_unit={}
&timezone=auto",
			lat,
			lon,
			units.temperature.as_ref(),
			units.speed.as_ref(),
			if units.precipitation == Precipitation::probability { "mm" } else {units.precipitation.as_ref()},
		);

		let res = reqwest::get(url)
			.await?
			.json::<Weather>()
			.await
			.with_context(|| "Weather data request failed.")?;

		Ok(res)
	}
}

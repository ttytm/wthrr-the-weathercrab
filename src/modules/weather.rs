use anyhow::{anyhow, Context, Result};
use chrono::NaiveDate;
use optional_struct::{optional_struct, Applyable};
use serde::Deserialize;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

use super::units::{Precipitation, Units};

// Open meteo json
// E.g., London:
// https://api.open-meteo.com/v1/forecast?latitude=51.5002&longitude=-0.1262&hourly=temperature_2m,relativehumidity_2m,apparent_temperature,surface_pressure,windspeed_10m,precipitation,weathercode&daily=weathercode,sunrise,sunset,winddirection_10m_dominant,temperature_2m_max,temperature_2m_min&current_weather=true&timezone=auto
#[optional_struct]
#[derive(Deserialize, Debug)]
pub struct Weather {
	#[optional_rename(OptionalCurrentWeather)]
	pub current_weather: CurrentWeather,
	#[optional_rename(OptionalHourlyUnits)]
	pub hourly_units: HourlyUnits,
	#[optional_rename(OptionalHourly)]
	pub hourly: Hourly,
	pub daily_units: DailyUnits,
	#[optional_rename(OptionalDaily)]
	pub daily: Daily,
}

#[optional_struct]
#[derive(Deserialize, Debug)]
pub struct CurrentWeather {
	pub temperature: f32,
	pub windspeed: f32,
	pub winddirection: f32,
	pub weathercode: u8,
	pub time: String,
}

#[optional_struct]
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

#[optional_struct]
#[derive(Deserialize, Debug)]
pub struct Hourly {
	pub temperature_2m: Vec<f32>,
	pub relativehumidity_2m: Vec<f32>,
	pub apparent_temperature: Vec<f32>,
	pub surface_pressure: Vec<f32>,
	pub dewpoint_2m: Vec<f32>,
	pub precipitation: Vec<f32>,
	pub precipitation_probability: Vec<u8>,
	pub weathercode: Vec<u8>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct DailyUnits {
	pub temperature_2m_max: String,
	pub temperature_2m_min: String,
}

#[optional_struct]
#[derive(Deserialize, Debug)]
pub struct Daily {
	pub time: Vec<String>,
	pub weathercode: Vec<u8>,
	pub sunrise: Vec<String>,
	pub sunset: Vec<String>,
	pub temperature_2m_max: Vec<f32>,
	pub temperature_2m_min: Vec<f32>,
	pub apparent_temperature_max: Vec<f32>,
	pub apparent_temperature_min: Vec<f32>,
	pub precipitation_probability_max: Vec<u8>,
	pub precipitation_sum: Option<Vec<f32>>,
}

impl Weather {
	pub async fn get(lat: f64, lon: f64, units: &Units) -> Result<Self> {
		// TODO: conditionally expand api call
		let url = format!(
			"https://api.open-meteo.com/v1/forecast?
latitude={lat}
&longitude={lon}
&current_weather=true
&temperature_unit={}
&windspeed_unit={}
&precipitation_unit={}
&hourly=temperature_2m,relativehumidity_2m,apparent_temperature,surface_pressure,dewpoint_2m,windspeed_10m,weathercode,precipitation,precipitation_probability
&daily=weathercode,sunrise,sunset,temperature_2m_max,temperature_2m_min,precipitation_probability_max,apparent_temperature_max,apparent_temperature_min
&timezone=auto",
			units.temperature.as_ref(),
			units.speed.as_ref(),
			if units.precipitation == Precipitation::probability { "mm" } else {units.precipitation.as_ref()},
		);

		let res = reqwest::get(url)
			.await?
			.json::<Self>()
			.await
			.with_context(|| "Weather data request failed.")?;

		Ok(res)
	}

	pub async fn get_date(date: NaiveDate, lat: f64, lon: f64, units: &Units) -> Result<OptionalWeather> {
		let url = format!(
			"https://archive-api.open-meteo.com/v1/archive?
latitude={lat}
&longitude={lon}
&start_date={date}
&end_date={date}
&temperature_unit={}
&windspeed_unit={}
&precipitation_unit={}
&hourly=temperature_2m,precipitation,weathercode
&daily=weathercode,temperature_2m_max,temperature_2m_min,apparent_temperature_max,apparent_temperature_min,sunrise,sunset,precipitation_sum
&timezone=auto",
			units.temperature.as_ref(),
			units.speed.as_ref(),
			if units.precipitation == Precipitation::probability { "mm" } else {units.precipitation.as_ref()},
		);

		let raw_res = reqwest::get(url)
			.await?
			.json::<Value>()
			.await
			.with_context(|| "Historical weather data request failed.")?;

		// It takes up to 5 days until temperature data is available in open-meteo's archive.
		// Therefore, we check for null values in the temperature.
		if raw_res["hourly"]["temperature_2m"]
			.as_array()
			.expect("Failed decoding temperature data for historical weather.")[0]
			.is_null()
		{
			return Err(anyhow!("The temperature for the requested day has not yet been archived."));
		}

		Ok(serde_json::from_value::<OptionalWeather>(raw_res)?)
	}

	pub async fn get_dates<'a>(
		dates: &'a HashSet<NaiveDate>,
		lat: f64,
		lon: f64,
		units: &Units,
	) -> Result<HashMap<&'a NaiveDate, OptionalWeather>> {
		let mut res = HashMap::new();
		for date in dates {
			res.insert(date, Self::get_date(*date, lat, lon, units).await?);
		}

		Ok(res)
	}
}

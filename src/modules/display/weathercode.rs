use anyhow::{anyhow, Result};

use crate::translation::translate;

pub struct WeatherCode {
	pub interpretation: String,
	pub icon: char,
}

impl WeatherCode {
	pub async fn resolve(weather_code: &f64, night: Option<bool>, lang: &str) -> Result<Self> {
		let is_night = night.unwrap_or_default();
		let c = *weather_code as u8;
		let (interpretation, icon);

		match c {
			0 => {
				interpretation = translate(lang, "Clear sky").await?;
				icon = if is_night { '' } else { '' };
			}
			1 => {
				interpretation = translate(lang, "Mostly Clear").await?;
				icon = if is_night { '' } else { '' };
			}
			2 => {
				interpretation = translate(lang, "Partly Cloudy").await?;
				icon = if is_night { '' } else { '' };
			}
			3 => {
				interpretation = translate(lang, "Overcast").await?;
				icon = '';
			}
			45 => {
				interpretation = translate(lang, "Fog").await?;
				icon = if is_night { '' } else { '' };
			}
			48 => {
				interpretation = translate(lang, "Depositing Rime Fog").await?;
				icon = '';
			}
			51 => {
				interpretation = translate(lang, "Light Drizzle").await?;
				icon = if is_night { '' } else { '' };
			}
			53 => {
				interpretation = translate(lang, "Moderate Drizzle").await?;
				icon = if is_night { '' } else { '' };
			}
			55 => {
				interpretation = translate(lang, "Dense Drizzle").await?;
				icon = if is_night { '' } else { '' };
			}
			56 => {
				interpretation = translate(lang, "Light Freezing Drizzle").await?;
				icon = if is_night { '' } else { 'ﭽ' };
			}
			57 => {
				interpretation = translate(lang, "Dense Freezing Drizzle").await?;
				icon = if is_night { '' } else { 'ﭽ' };
			}
			61 => {
				interpretation = translate(lang, "Slight Rain").await?;
				icon = if is_night { '' } else { '' };
			}
			63 => {
				interpretation = translate(lang, "Moderate Rain").await?;
				icon = if is_night { '' } else { '' };
			}
			65 => {
				interpretation = translate(lang, "Heavy Rain").await?;
				icon = if is_night { '' } else { '' };
			}
			66 => {
				interpretation = translate(lang, "Light Freezing Rain").await?;
				icon = if is_night { '' } else { '' };
			}
			67 => {
				interpretation = translate(lang, "Heavy Freezing Rain").await?;
				icon = if is_night { '' } else { '' };
			}
			71 => {
				interpretation = translate(lang, "Slight Snow Fall").await?;
				icon = if is_night { '' } else { '' };
			}
			73 => {
				interpretation = translate(lang, "Moderate Snow Fall").await?;
				icon = if is_night { '' } else { '' };
			}
			75 => {
				interpretation = translate(lang, "Heavy Snow Fall").await?;
				icon = if is_night { '' } else { '' };
			}
			77 => {
				interpretation = translate(lang, "Snow Grains").await?;
				icon = '';
			}
			80 => {
				interpretation = translate(lang, "Slight Rain Showers").await?;
				icon = if is_night { '' } else { '' };
			}
			81 => {
				interpretation = translate(lang, "Moderate Rain Showers").await?;
				icon = if is_night { '' } else { '' };
			}
			82 => {
				interpretation = translate(lang, "Violent Rain Showers").await?;
				icon = if is_night { '' } else { '' };
			}
			85 => {
				interpretation = translate(lang, "Slight Snow Showers").await?;
				icon = if is_night { '' } else { '' };
			}
			86 => {
				interpretation = translate(lang, "Heavy Snow Showers").await?;
				icon = if is_night { '' } else { '' };
			}
			95 => {
				interpretation = translate(lang, "Thunderstorm").await?;
				icon = if is_night { '' } else { '' };
			}
			96 => {
				interpretation = translate(lang, "Thunderstorm, Slight Hail").await?;
				icon = if is_night { '' } else { '' };
			}
			99 => {
				interpretation = translate(lang, "Thunderstorm, Heavy Hail").await?;
				icon = if is_night { '' } else { '' };
			}
			_ => return Err(anyhow!("Unknown weather code")),
		}

		Ok(WeatherCode { interpretation, icon })
	}
}

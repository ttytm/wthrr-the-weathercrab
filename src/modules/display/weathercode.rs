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
				match is_night {
					true => icon = '',
					_ => icon = '',
				}
			}
			1 => {
				interpretation = translate(lang, "Mostly Clear").await?;
				match is_night {
					true => icon = '',
					_ => icon = '',
				}
			}
			2 => {
				interpretation = translate(lang, "Partly Cloudy").await?;
				match is_night {
					true => icon = '',
					_ => icon = '',
				}
			}
			3 => {
				interpretation = translate(lang, "Overcast").await?;
				icon = '';
			}
			45 => {
				interpretation = translate(lang, "Fog").await?;
				match is_night {
					true => icon = '',
					_ => icon = '',
				}
			}
			48 => {
				interpretation = translate(lang, "Depositing Rime Fog").await?;
				icon = '';
			}
			51 => {
				interpretation = translate(lang, "Light Drizzle").await?;
				match is_night {
					true => icon = '',
					_ => icon = '',
				}
			}
			53 => {
				interpretation = translate(lang, "Moderate Drizzle").await?;
				match is_night {
					true => icon = '',
					_ => icon = '',
				}
			}
			55 => {
				interpretation = translate(lang, "Dense Drizzle").await?;
				match is_night {
					true => icon = '',
					_ => icon = '',
				}
			}
			56 => {
				interpretation = translate(lang, "Light Freezing Drizzle").await?;
				match is_night {
					true => icon = '',
					_ => icon = 'ﭽ',
				}
			}
			57 => {
				interpretation = translate(lang, "Dense Freezing Drizzle").await?;
				match is_night {
					true => icon = '',
					_ => icon = 'ﭽ',
				}
			}
			61 => {
				interpretation = translate(lang, "Slight Rain").await?;
				match is_night {
					true => icon = '',
					_ => icon = '',
				}
			}
			63 => {
				interpretation = translate(lang, "Moderate Rain").await?;
				match is_night {
					true => icon = '',
					_ => icon = '',
				}
			}
			65 => {
				interpretation = translate(lang, "Heavy Rain").await?;
				match is_night {
					true => icon = '',
					_ => icon = '',
				}
			}
			66 => {
				interpretation = translate(lang, "Light Freezing Rain").await?;
				match is_night {
					true => icon = '',
					_ => icon = '',
				}
			}
			67 => {
				interpretation = translate(lang, "Heavy Freezing Rain").await?;
				match is_night {
					true => icon = '',
					_ => icon = '',
				}
			}
			71 => {
				interpretation = translate(lang, "Slight Snow Fall").await?;
				match is_night {
					true => icon = '',
					_ => icon = '',
				}
			}
			73 => {
				interpretation = translate(lang, "Moderate Snow Fall").await?;
				match is_night {
					true => icon = '',
					_ => icon = '',
				}
			}
			75 => {
				interpretation = translate(lang, "Heavy Snow Fall").await?;
				match is_night {
					true => icon = '',
					_ => icon = '',
				}
			}
			77 => {
				interpretation = translate(lang, "Snow Grains").await?;
				icon = '';
			}
			80 => {
				interpretation = translate(lang, "Slight Rain Showers").await?;
				match is_night {
					true => icon = '',
					_ => icon = '',
				}
			}
			81 => {
				interpretation = translate(lang, "Moderate Rain Showers").await?;
				match is_night {
					true => icon = '',
					_ => icon = '',
				}
			}
			82 => {
				interpretation = translate(lang, "Violent Rain Showers").await?;
				match is_night {
					true => icon = '',
					_ => icon = '',
				}
			}
			85 => {
				interpretation = translate(lang, "Slight Snow Showers").await?;
				match is_night {
					true => icon = '',
					_ => icon = '',
				}
			}
			86 => {
				interpretation = translate(lang, "Heavy Snow Showers").await?;
				match is_night {
					true => icon = '',
					_ => icon = '',
				}
			}
			95 => {
				interpretation = translate(lang, "Thunderstorm").await?;
				match is_night {
					true => icon = '',
					_ => icon = '',
				}
			}
			96 => {
				interpretation = translate(lang, "Thunderstorm, Slight Hail").await?;
				match is_night {
					true => icon = '',
					_ => icon = '',
				}
			}
			99 => {
				interpretation = translate(lang, "Thunderstorm, Heavy Hail").await?;
				match is_night {
					true => icon = '',
					_ => icon = '',
				}
			}
			_ => return Err(anyhow!("Unknown weather code")),
		}

		Ok(WeatherCode { interpretation, icon })
	}
}

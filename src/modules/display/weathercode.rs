use anyhow::{anyhow, Result};

pub struct WeatherCode {
	pub interpretation: String,
	pub icon: String,
}

impl WeatherCode {
	pub fn resolve(weather_code: &f64, night: Option<bool>) -> Result<Self> {
		let is_night = night.unwrap_or_default();
		let c = *weather_code as u8;
		let (interpretation, icon);

		match c {
			0 => {
				interpretation = "Clear sky";
				match is_night {
					true => icon = "",
					false => icon = "",
				}
			}
			1 => {
				interpretation = "Mainly Clear";
				match is_night {
					true => icon = "",
					false => icon = "",
				}
			}
			2 => {
				interpretation = "Partly Cloudy";
				match is_night {
					true => icon = "",
					false => icon = "",
				}
			}
			3 => {
				interpretation = "Overcast";
				icon = "";
			}
			45 => {
				interpretation = "Fog";
				match is_night {
					true => icon = "",
					false => icon = "",
				}
			}
			48 => {
				interpretation = "Depositing Rime Fog";
				icon = "";
			}
			51 => {
				interpretation = "Light Drizzle";
				match is_night {
					true => icon = "",
					false => icon = "",
				}
			}
			53 => {
				interpretation = "Moderate Drizzle";
				match is_night {
					true => icon = "",
					false => icon = "",
				}
			}
			55 => {
				interpretation = "Dense Drizzle";
				match is_night {
					true => icon = "",
					false => icon = "",
				}
			}
			56 => {
				interpretation = "Light Freezing Drizzle";
				match is_night {
					true => icon = "",
					false => icon = "ﭽ",
				}
			}
			57 => {
				interpretation = "Dense Freezing Drizzle";
				match is_night {
					true => icon = "",
					false => icon = "ﭽ",
				}
			}
			61 => {
				interpretation = "Slight Rain";
				match is_night {
					true => icon = "",
					false => icon = "",
				}
			}
			63 => {
				interpretation = "Moderate Rain";
				match is_night {
					true => icon = "",
					false => icon = "",
				}
			}
			65 => {
				interpretation = "Heavy Rain";
				match is_night {
					true => icon = "",
					false => icon = "",
				}
			}
			66 => {
				interpretation = "Light Freezing Rain";
				match is_night {
					true => icon = "",
					false => icon = "",
				}
			}
			67 => {
				interpretation = "Heavy Freezing Rain";
				match is_night {
					true => icon = "",
					false => icon = "",
				}
			}
			71 => {
				interpretation = "Slight Snow Fall";
				match is_night {
					true => icon = "",
					false => icon = "",
				}
			}
			73 => {
				interpretation = "Moderate Snow Fall";
				match is_night {
					true => icon = "",
					false => icon = "",
				}
			}
			75 => {
				interpretation = "Heavy Snow Fall";
				match is_night {
					true => icon = "",
					false => icon = "",
				}
			}
			77 => {
				interpretation = "Snow Grains";
				icon = "";
			}
			80 => {
				interpretation = "Slight Rain Showers";
				match is_night {
					true => icon = "",
					false => icon = "",
				}
			}
			81 => {
				interpretation = "Moderate Rain Showers";
				match is_night {
					true => icon = "",
					false => icon = "",
				}
			}
			82 => {
				interpretation = "Violent Rain Showers";
				match is_night {
					true => icon = "",
					false => icon = "",
				}
			}
			85 => {
				interpretation = "Slight Snow Showers";
				match is_night {
					true => icon = "",
					false => icon = "",
				}
			}
			86 => {
				interpretation = "Heavy Snow Showers";
				match is_night {
					true => icon = "",
					false => icon = "",
				}
			}
			95 => {
				interpretation = "Thunderstorm";
				match is_night {
					true => icon = "",
					false => icon = "",
				}
			}
			96 => {
				interpretation = "Thunderstorm, Slight Hail";
				match is_night {
					true => icon = "",
					false => icon = "",
				}
			}
			99 => {
				interpretation = "Thunderstorm, Heavy Hail";
				match is_night {
					true => icon = "",
					false => icon = "",
				}
			}
			_ => return Err(anyhow!("Unknown weather code")),
		}

		Ok(WeatherCode {
			interpretation: interpretation.to_string(),
			icon: icon.to_string(),
		})
	}
}

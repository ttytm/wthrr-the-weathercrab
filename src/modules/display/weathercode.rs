use anyhow::{anyhow, Result};

use crate::modules::locales::WeatherCodeLocales;

pub struct WeatherCode {
	pub interpretation: String,
	pub icon: char,
}

impl WeatherCode {
	pub fn resolve(weather_code: &f64, night: Option<bool>, t: &WeatherCodeLocales) -> Result<Self> {
		let is_night = night.unwrap_or_default();
		let c = *weather_code as u8;
		let (interpretation, icon);

		match c {
			0 => {
				interpretation = t.clear_sky.clone();
				icon = if is_night { '' } else { '' };
			}
			1 => {
				interpretation = t.mostly_clear.clone();
				icon = if is_night { '' } else { '' };
			}
			2 => {
				interpretation = t.partly_cloudy.clone();
				icon = if is_night { '' } else { '' };
			}
			3 => {
				interpretation = t.overcast.clone();
				icon = '';
			}
			45 => {
				interpretation = t.fog.clone();
				icon = if is_night { '' } else { '' };
			}
			48 => {
				interpretation = t.depositing_rime_fog.clone();
				icon = '';
			}
			51 => {
				interpretation = t.light_drizzle.clone();
				icon = if is_night { '' } else { '' };
			}
			53 => {
				interpretation = t.moderate_drizzle.clone();
				icon = if is_night { '' } else { '' };
			}
			55 => {
				interpretation = t.dense_drizzle.clone();
				icon = if is_night { '' } else { '' };
			}
			56 => {
				interpretation = t.light_freezing_drizzle.clone();
				icon = if is_night { '' } else { 'ﭽ' };
			}
			57 => {
				interpretation = t.dense_freezing_drizzle.clone();
				icon = if is_night { '' } else { 'ﭽ' };
			}
			61 => {
				interpretation = t.slight_rain.clone();
				icon = if is_night { '' } else { '' };
			}
			63 => {
				interpretation = t.moderate_rain.clone();
				icon = if is_night { '' } else { '' };
			}
			65 => {
				interpretation = t.heavy_rain.clone();
				icon = if is_night { '' } else { '' };
			}
			66 => {
				interpretation = t.light_freezing_rain.clone();
				icon = if is_night { '' } else { '' };
			}
			67 => {
				interpretation = t.heavy_freezing_rain.clone();
				icon = if is_night { '' } else { '' };
			}
			71 => {
				interpretation = t.slight_snow_fall.clone();
				icon = if is_night { '' } else { '' };
			}
			73 => {
				interpretation = t.moderate_snow_fall.clone();
				icon = if is_night { '' } else { '' };
			}
			75 => {
				interpretation = t.heavy_snow_fall.clone();
				icon = if is_night { '' } else { '' };
			}
			77 => {
				interpretation = t.snow_grains.clone();
				icon = '';
			}
			80 => {
				interpretation = t.slight_rain_showers.clone();
				icon = if is_night { '' } else { '' };
			}
			81 => {
				interpretation = t.moderate_rain_showers.clone();
				icon = if is_night { '' } else { '' };
			}
			82 => {
				interpretation = t.violent_rain_showers.clone();
				icon = if is_night { '' } else { '' };
			}
			85 => {
				interpretation = t.slight_snow_showers.clone();
				icon = if is_night { '' } else { '' };
			}
			86 => {
				interpretation = t.heavy_snow_showers.clone();
				icon = if is_night { '' } else { '' };
			}
			95 => {
				interpretation = t.thunderstorm.clone();
				icon = if is_night { '' } else { '' };
			}
			96 => {
				interpretation = t.thunderstorm_slight_hail.clone();
				icon = if is_night { '' } else { '' };
			}
			99 => {
				interpretation = t.thunderstorm_heavy_hail.clone();
				icon = if is_night { '' } else { '' };
			}
			_ => return Err(anyhow!("Unknown weather code")),
		}

		Ok(WeatherCode { interpretation, icon })
	}
}

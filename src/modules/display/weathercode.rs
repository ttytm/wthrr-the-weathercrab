use anyhow::{anyhow, Result};

use crate::modules::locales::WeatherCodeLocales;

pub struct WeatherCode {
	pub interpretation: String,
	pub icon: char,
}

impl WeatherCode {
	pub fn resolve(weather_code: &f64, is_night: bool, t: &WeatherCodeLocales) -> Result<Self> {
		let res = match *weather_code as u8 {
			0 => (t.clear_sky.clone(), if is_night { '' } else { '' }),
			1 => (t.mostly_clear.clone(), if is_night { '' } else { '' }),
			2 => (t.partly_cloudy.clone(), if is_night { '' } else { '' }),
			3 => (t.overcast.clone(), ''),
			45 => (t.fog.clone(), if is_night { '' } else { '' }),
			48 => (t.depositing_rime_fog.clone(), ''),
			51 => (t.light_drizzle.clone(), if is_night { '' } else { '' }),
			53 => (t.moderate_drizzle.clone(), if is_night { '' } else { '' }),
			55 => (t.dense_drizzle.clone(), if is_night { '' } else { '' }),
			56 => (t.light_freezing_drizzle.clone(), if is_night { '' } else { 'ﭽ' }),
			57 => (t.dense_freezing_drizzle.clone(), if is_night { '' } else { 'ﭽ' }),
			61 => (t.slight_rain.clone(), if is_night { '' } else { '' }),
			63 => (t.moderate_rain.clone(), if is_night { '' } else { '' }),
			65 => (t.heavy_rain.clone(), if is_night { '' } else { '' }),
			66 => (t.light_freezing_rain.clone(), if is_night { '' } else { '' }),
			67 => (t.heavy_freezing_rain.clone(), if is_night { '' } else { '' }),
			71 => (t.slight_snow_fall.clone(), if is_night { '' } else { '' }),
			73 => (t.moderate_snow_fall.clone(), if is_night { '' } else { '' }),
			75 => (t.heavy_snow_fall.clone(), if is_night { '' } else { '' }),
			77 => (t.snow_grains.clone(), ''),
			80 => (t.slight_rain_showers.clone(), if is_night { '' } else { '' }),
			81 => (t.moderate_rain_showers.clone(), if is_night { '' } else { '' }),
			82 => (t.violent_rain_showers.clone(), if is_night { '' } else { '' }),
			85 => (t.slight_snow_showers.clone(), if is_night { '' } else { '' }),
			86 => (t.heavy_snow_showers.clone(), if is_night { '' } else { '' }),
			95 => (t.thunderstorm.clone(), if is_night { '' } else { '' }),
			96 => (t.thunderstorm_slight_hail.clone(), if is_night { '' } else { '' }),
			99 => (t.thunderstorm_heavy_hail.clone(), if is_night { '' } else { '' }),
			_ => return Err(anyhow!("Unknown weather code")),
		};

		Ok(WeatherCode { interpretation: res.0, icon: res.1 })
	}
}

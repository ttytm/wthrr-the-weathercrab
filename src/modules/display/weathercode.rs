use anyhow::{anyhow, Result};

use crate::modules::localization::WeatherCodeLocales;

pub struct WeatherCode {
	pub interpretation: String,
	pub icon: char,
}

impl WeatherCode {
	pub fn resolve(weather_code: u8, is_night: bool, t: &WeatherCodeLocales) -> Result<Self> {
		let res = match weather_code {
			0 => (&t.clear_sky, if is_night { '' } else { '' }),
			1 => (&t.mostly_clear, if is_night { '' } else { '' }),
			2 => (&t.partly_cloudy, if is_night { '' } else { '' }),
			3 => (&t.overcast, ''),
			45 => (&t.fog, if is_night { '' } else { '' }),
			48 => (&t.depositing_rime_fog, ''),
			51 => (&t.light_drizzle, if is_night { '' } else { '' }),
			53 => (&t.moderate_drizzle, if is_night { '' } else { '' }),
			55 => (&t.dense_drizzle, if is_night { '' } else { '' }),
			56 => (&t.light_freezing_drizzle, if is_night { '' } else { 'ﭽ' }),
			57 => (&t.dense_freezing_drizzle, if is_night { '' } else { 'ﭽ' }),
			61 => (&t.slight_rain, if is_night { '' } else { '' }),
			63 => (&t.moderate_rain, if is_night { '' } else { '' }),
			65 => (&t.heavy_rain, if is_night { '' } else { '' }),
			66 => (&t.light_freezing_rain, if is_night { '' } else { '' }),
			67 => (&t.heavy_freezing_rain, if is_night { '' } else { '' }),
			71 => (&t.slight_snow_fall, if is_night { '' } else { '' }),
			73 => (&t.moderate_snow_fall, if is_night { '' } else { '' }),
			75 => (&t.heavy_snow_fall, if is_night { '' } else { '' }),
			77 => (&t.snow_grains, ''),
			80 => (&t.slight_rain_showers, if is_night { '' } else { '' }),
			81 => (&t.moderate_rain_showers, if is_night { '' } else { '' }),
			82 => (&t.violent_rain_showers, if is_night { '' } else { '' }),
			85 => (&t.slight_snow_showers, if is_night { '' } else { '' }),
			86 => (&t.heavy_snow_showers, if is_night { '' } else { '' }),
			95 => (&t.thunderstorm, if is_night { '' } else { '' }),
			96 => (&t.thunderstorm_slight_hail, if is_night { '' } else { '' }),
			99 => (&t.thunderstorm_heavy_hail, if is_night { '' } else { '' }),
			_ => return Err(anyhow!("Unknown weather code")),
		};

		Ok(WeatherCode {
			interpretation: res.0.to_string(),
			icon: res.1,
		})
	}
}

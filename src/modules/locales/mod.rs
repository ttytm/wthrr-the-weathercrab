pub mod localization;

use optional_struct::*;
use serde::{Deserialize, Serialize};

#[optional_struct(LocalesFile)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Locales {
	pub greeting: String,
	pub search_station: String,
	#[optional_rename(ConfigLocalesFile)]
	pub config: ConfigLocales,
	#[optional_rename(WeatherLocalesFile)]
	pub weather: WeatherLocales,
}

#[optional_struct(ConfigLocalesFile)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigLocales {
	pub confirm: String,
	pub next_time: String,
	pub deny: String,
	pub always_auto: String,
	pub save_as_default: String,
	pub reset_config: String,
	pub no_selection: String,
}

#[optional_struct(WeatherLocalesFile)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeatherLocales {
	pub feels_like: String,
	pub humidity: String,
	pub dew_point: String,
	pub hourly_forecast: String,
	#[optional_rename(WeatherCodeLocalesFile)]
	pub weather_code: WeatherCodeLocales,
}

#[optional_struct(WeatherCodeLocalesFile)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeatherCodeLocales {
	pub clear_sky: String,
	pub mostly_clear: String,
	pub partly_cloudy: String,
	pub overcast: String,
	pub fog: String,
	pub depositing_rime_fog: String,
	pub light_drizzle: String,
	pub moderate_drizzle: String,
	pub dense_drizzle: String,
	pub light_freezing_drizzle: String,
	pub dense_freezing_drizzle: String,
	pub slight_rain: String,
	pub moderate_rain: String,
	pub heavy_rain: String,
	pub light_freezing_rain: String,
	pub heavy_freezing_rain: String,
	pub slight_snow_fall: String,
	pub moderate_snow_fall: String,
	pub heavy_snow_fall: String,
	pub snow_grains: String,
	pub slight_rain_showers: String,
	pub moderate_rain_showers: String,
	pub violent_rain_showers: String,
	pub slight_snow_showers: String,
	pub heavy_snow_showers: String,
	pub thunderstorm: String,
	pub thunderstorm_slight_hail: String,
	pub thunderstorm_heavy_hail: String,
}

impl Default for Locales {
	fn default() -> Self {
		Self {
			greeting: "Hey friend. I'm glad you are asking.".to_string(),
			search_station: "You didn't specify a city. Should I check for a weather station close to your location?"
				.to_string(),
			config: ConfigLocales::default(),
			weather: WeatherLocales::default(),
		}
	}
}

impl Default for ConfigLocales {
	fn default() -> Self {
		Self {
			confirm: "Yes please".to_string(),
			next_time: "No, ask me next time".to_string(),
			deny: "No, dont ask me again".to_string(),
			always_auto: "Always check for a weather station".to_string(),
			save_as_default: "Would you like to use this as your default?".to_string(),
			reset_config: "This will wipe wthrr's configuration. Continue?".to_string(),
			no_selection: "User did not select anything or exited".to_string(),
		}
	}
}

impl Default for WeatherLocales {
	fn default() -> Self {
		Self {
			feels_like: "Feels like".to_string(),
			humidity: "Humidity".to_string(),
			dew_point: "Dew Point".to_string(),
			hourly_forecast: "Hourly Forecast".to_string(),
			weather_code: WeatherCodeLocales::default(),
		}
	}
}

impl Default for WeatherCodeLocales {
	fn default() -> Self {
		Self {
			clear_sky: "Clear Sky".to_string(),
			mostly_clear: "Mostly Clear".to_string(),
			partly_cloudy: "Partly Cloudy".to_string(),
			overcast: "Overcast".to_string(),
			fog: "Fog".to_string(),
			depositing_rime_fog: "Depositing Rime Fog".to_string(),
			light_drizzle: "Light Drizzle".to_string(),
			moderate_drizzle: "Moderate Drizzle".to_string(),
			dense_drizzle: "Dense Drizzle".to_string(),
			light_freezing_drizzle: "Light Freezing Drizzle".to_string(),
			dense_freezing_drizzle: "Dense Freezing Drizzle".to_string(),
			slight_rain: "Slight Rain".to_string(),
			moderate_rain: "Moderate Rain".to_string(),
			heavy_rain: "Heavy Rain".to_string(),
			light_freezing_rain: "Light Freezing Rain".to_string(),
			heavy_freezing_rain: "Heavy Freezing Rain".to_string(),
			slight_snow_fall: "Slight Snow Fall".to_string(),
			moderate_snow_fall: "Moderate Snow Fall".to_string(),
			heavy_snow_fall: "Heavy Snow Fall".to_string(),
			snow_grains: "Snow Grains".to_string(),
			slight_rain_showers: "Slight Rain Showers".to_string(),
			moderate_rain_showers: "Moderate Rain Showers".to_string(),
			violent_rain_showers: "Violent Rain Showers".to_string(),
			slight_snow_showers: "Slight Snow Showers".to_string(),
			heavy_snow_showers: "Heavy Snow Showers".to_string(),
			thunderstorm: "Thunderstorm".to_string(),
			thunderstorm_slight_hail: "Thunderstorm, Slight Hail".to_string(),
			thunderstorm_heavy_hail: "Thunderstorm, Heavy Hail".to_string(),
		}
	}
}

use anyhow::{Context, Result};
use chrono::{Local, NaiveDate};
use directories::ProjectDirs;
use futures::{stream::FuturesOrdered, TryStreamExt};
use optional_struct::{optional_struct, Applicable};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
	fs::{self, File},
	io::Write,
	path::PathBuf,
};

#[optional_struct(LocalesFile)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(clippy::unsafe_derive_deserialize)]
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
	pub felt_like: String,
	pub humidity: String,
	pub dew_point: String,
	pub hourly_forecast: String,
	pub daily_overview: String,
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
			reset_config: "This will wipe ww's configuration. Continue?".to_string(),
			no_selection: "User did not select anything or exited".to_string(),
		}
	}
}

impl Default for WeatherLocales {
	fn default() -> Self {
		Self {
			feels_like: "Feels like".to_string(),
			felt_like: "Felt like".to_string(),
			humidity: "Humidity".to_string(),
			dew_point: "Dew Point".to_string(),
			hourly_forecast: "Hourly Forecast".to_string(),
			daily_overview: "Daily Overview".to_string(),
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

const DATETIME_LOCALES: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/locales/pure-rust-locales.txt"));

impl Locales {
	pub async fn get(lang: &str) -> Result<Self> {
		let mut texts = Self::default();
		let path = Self::get_path(lang);

		if let Ok(file) = fs::read_to_string(path) {
			if let Ok(contents) = serde_json::from_str::<LocalesFile>(&file) {
				contents.apply_to(&mut texts);
				return Ok(texts);
			};
		};

		if lang != "en_US" && lang != "en" {
			texts.translate_all(lang).await?;
		}

		Ok(texts)
	}

	#[allow(clippy::cast_ptr_alignment)]
	async fn translate_all(&mut self, lang: &str) -> Result<()> {
		let size = std::mem::size_of_val(self);
		let ptr = (self as *mut Self).cast::<u8>();

		// Iterate over each field in the struct, create a future to translate the current field's value
		let translated_values: Vec<_> = (0..size)
			.step_by(std::mem::size_of::<String>())
			.map(|offset| unsafe {
				let field_ptr = ptr.add(offset);
				let field_value = &*(field_ptr.cast::<String>());
				Self::translate_str(lang, field_value)
			})
			.collect::<FuturesOrdered<_>>()
			// Wait for each future in the stream to complete and store the translated values in a vector
			.try_collect()
			.await?;

		// Iterate over each field in the struct again, update current field value with the translated value
		for (offset, translated_value) in (0..size).step_by(std::mem::size_of::<String>()).zip(translated_values) {
			unsafe {
				let field_ptr = ptr.add(offset);
				field_ptr.cast::<String>().write(translated_value);
			}
		}

		Ok(())
	}

	async fn translate_str(lang: &str, input: &str) -> Result<String> {
		let url = Url::parse_with_params(
			"https://translate.googleapis.com/translate_a/single?client=gtx&ie=UTF-8&oe=UTF-8&dt=t&sl=en_US",
			&[("tl", lang), ("q", input)],
		)?;

		let res = reqwest::get(url)
			.await?
			.json::<Vec<Value>>()
			.await
			.with_context(|| "Translation request failed.")?;

		let output = res.first().map_or_else(String::new, |i| {
			i.as_array()
				.unwrap()
				.iter()
				.map(|s| s[0].as_str().unwrap())
				.collect::<Vec<&str>>()
				.join("")
		});

		Ok(output)
	}

	pub fn store(&self, lang: &str) {
		let path = Self::get_path(lang);
		let dir = path.parent().unwrap();
		if !dir.is_dir() {
			fs::create_dir(dir).unwrap();
		};

		let mut file = File::create(path).unwrap();
		file.write_all(serde_json::to_string_pretty(self).unwrap().as_bytes()).unwrap();
	}

	pub fn get_path(lang: &str) -> PathBuf {
		ProjectDirs::from("", "", crate::modules::config::CONFIG_DIR_NAME)
			.unwrap()
			.config_dir()
			.join("locales")
			.join(format!("{lang}.json"))
	}

	#[allow(clippy::unnecessary_wraps)]
	pub fn localize_date(dt: NaiveDate, lang: &str) -> Result<String> {
		let matching_locale = DATETIME_LOCALES.lines().skip(1).find(|line| line == &lang).or_else(|| {
			DATETIME_LOCALES.lines().skip(1).find(|line| {
				let short_lang_code = line.split('_').next().unwrap();
				short_lang_code == lang
			})
		});

		let format = format!("%a, %e %b{}", if dt < Local::now().date_naive() { " %Y" } else { "" });

		let date = if let Some(locale) = matching_locale {
			dt.format_localized(&format, locale.try_into().unwrap()).to_string()
		} else {
			dt.format(&format).to_string()
		};

		Ok(date)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn translate_string() -> Result<()> {
		let (target_lang, input) = ("de_DE", "tongue-twister");

		let res = Locales::translate_str(target_lang, input).await?;

		assert!(res.contains("Zungenbrecher"));

		Ok(())
	}
}

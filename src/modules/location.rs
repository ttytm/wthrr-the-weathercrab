use crate::modules::{config::Config, localization::Locales};
use anyhow::{anyhow, Result};
use dialoguer::{theme::ColorfulTheme, Confirm};
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Location {
	pub name: String,
	pub lat: f64,
	pub lon: f64,
}

#[derive(Deserialize)]
pub struct GeoIpLocation {
	pub latitude: f64,
	pub longitude: f64,
	pub city_name: String,
	pub country_code: String,
}

#[derive(Deserialize)]
struct OpenStreetMapGeoObj {
	// place_id: u64,
	// licence: String,
	// osm_type: String,
	// osm_id: u64,
	// boundingbox: Vec<String>,
	lat: String,
	lon: String,
	display_name: String,
	// place_rank: i32,
	// category: String,
	// #[serde(rename(deserialize = "type"))]
	// kind: String,
	// importance: f64,
	// icon: String,
}

#[derive(Deserialize)]
struct OpenMeteoResults {
	results: Vec<OpenMeteoGeoObj>,
}

#[derive(Deserialize)]
struct OpenMeteoGeoObj {
	// id: i32,
	name: String,
	latitude: f64,
	longitude: f64,
	// elevation: f64,
	// timezone: String,
	// feature_code: String,
	// country_code: String,
	// country: String,
	// country_id: i32,
	// population: i32,
	// admin1: String,
	// admin2: String,
	// admin3: String,
	// admin4: String,
	// admin1_id: i32,
	// admin2_id: i32,
	// admin3_id: i32,
	// admin4_id: i32,
	// postcodes: Vec<String>,
}

impl GeoIpLocation {
	pub async fn get() -> Result<GeoIpLocation> {
		let res = reqwest::get("https://api.geoip.rs")
			.await?
			.json::<GeoIpLocation>()
			.await?;

		Ok(res)
	}
}

impl Location {
	pub async fn get(address: &str, lang: &str) -> Result<Location> {
		let client = Client::builder().user_agent("wthrr-the-weathercrab").build()?;
		let results = Self::search_osm(&client, address, lang).await;

		match results {
			Ok(address) => Ok(address),
			Err(_) => Self::search_open_meteo(&client).await,
		}
	}

	async fn search_osm(client: &Client, address: &str, lang: &str) -> Result<Location> {
		let url = format!(
			"https://nominatim.openstreetmap.org/search?q={address}&accept-language={lang}&limit=1&format=jsonv2",
		);
		let results: Vec<OpenStreetMapGeoObj> = client.get(&url).send().await?.json().await?;
		let result = results.first().ok_or_else(|| anyhow!("Location request failed."))?;

		Ok(Location {
			name: result.display_name.clone(),
			lon: result.lon.parse::<f64>().unwrap(),
			lat: result.lat.parse::<f64>().unwrap(),
		})
	}

	async fn search_open_meteo(client: &Client) -> Result<Location> {
		let url = "https://geocoding-api.open-meteo.com/v1/search?name=Berlin&language=fr";
		let results: OpenMeteoResults = client.get(url).send().await?.json().await?;
		let result = results
			.results
			.first()
			.ok_or_else(|| anyhow!("Location request failed."))?;

		Ok(Location {
			name: result.name.clone(),
			lon: result.longitude,
			lat: result.latitude,
		})
	}

	pub async fn resolve_input(arg_address: &str, config: &Config, texts: &Locales) -> Result<String> {
		if arg_address.is_empty() && config.address == "arg_input" {
			return Err(anyhow!("Your configuration requires you to specify a city."));
		};

		let prompt_user = arg_address.is_empty() && config.address.is_empty();
		if config.gui.greeting {
			println!("{} 🦀  {}", if prompt_user { "" } else { " " }, texts.greeting);
		}

		if prompt_user {
			if !Confirm::with_theme(&ColorfulTheme::default())
				.with_prompt(&texts.search_station)
				.interact()?
			{
				std::process::exit(1)
			}
			let auto_loc = GeoIpLocation::get().await?;
			return Ok(format!("{},{}", auto_loc.city_name, auto_loc.country_code));
		}

		// Handle address from args or config
		if arg_address == "auto" || (arg_address.is_empty() && config.address == "auto") {
			let auto_loc = GeoIpLocation::get().await?;
			Ok(format!("{},{}", auto_loc.city_name, auto_loc.country_code))
		} else if !arg_address.is_empty() {
			Ok(arg_address.to_string())
		} else {
			Ok(config.address.to_string())
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn address_from_arg() -> Result<()> {
		let arg_address = "new york";
		let texts = Locales::default();
		let config = Config {
			address: "Berlin, DE".to_string(),
			..Default::default()
		};

		let address = Location::resolve_input(arg_address, &config, &texts).await?;
		assert!(address.contains("new york"));

		Ok(())
	}

	#[tokio::test]
	async fn address_from_cfg() -> Result<()> {
		let arg_address = "";
		let texts = Locales::default();
		let config = Config {
			address: "Berlin, DE".to_string(),
			..Default::default()
		};

		let address = Location::resolve_input(arg_address, &config, &texts).await?;
		assert!(address.contains("Berlin"));

		Ok(())
	}

	#[tokio::test]
	async fn geolocation_response() -> Result<()> {
		let (address, lang_de, lang_pl) = ("berlin", "de", "pl");

		let loc_de = Location::get(address, lang_de).await?;
		let loc_pl = Location::get(address, lang_pl).await?;

		assert!(loc_de.name.contains("Deutschland"));
		assert!(loc_pl.name.contains("Niemcy"));

		Ok(())
	}
}

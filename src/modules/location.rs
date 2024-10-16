use anyhow::{anyhow, Result};
use dialoguer::{theme::ColorfulTheme, Confirm};
use reqwest::Client;
use serde::Deserialize;

use super::{config::Config, localization::Locales};
use crate::modules::api::{Api, ApiName, ApiQuery, ErrorMessage};

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

impl From<GeoIpLocation> for Location {
	fn from(val: GeoIpLocation) -> Self {
		Self {
			name: val.city_name.clone(),
			lat: val.latitude,
			lon: val.longitude,
		}
	}
}

#[derive(Deserialize)]
struct OpenStreetMapGeoObj {
	lat: String,
	lon: String,
	display_name: String,
}

impl From<&OpenStreetMapGeoObj> for Location {
	fn from(val: &OpenStreetMapGeoObj) -> Self {
		Self {
			name: val.display_name.clone(),
			lon: val.lon.parse::<f64>().unwrap(),
			lat: val.lat.parse::<f64>().unwrap(),
		}
	}
}

#[derive(Deserialize)]
struct OpenMeteoGeoObj {
	name: String,
	latitude: f64,
	longitude: f64,
}

impl From<&OpenMeteoGeoObj> for Location {
	fn from(val: &OpenMeteoGeoObj) -> Self {
		Self {
			name: val.name.clone(),
			lon: val.longitude,
			lat: val.latitude,
		}
	}
}

impl Location {
	pub async fn get(address: &str, lang: &str) -> Result<Self> {
		let client = Client::builder().user_agent("ww").build()?;
		let results = Self::search_osm(&client, address, lang).await;

		match results {
			Ok(address) => Ok(address),
			Err(_) => Self::search_open_meteo(&client, address, lang).await,
		}
	}

	async fn search_osm(client: &Client, address: &str, language: &str) -> Result<Self> {
		client
			.get(
				ApiQuery::location(ApiName::OpenStreetMap, address, language)
					.convert()
					.assemble(),
			)
			.send()
			.await?
			.json::<Vec<OpenStreetMapGeoObj>>()
			.await?
			.first()
			.ok_or_else(|| anyhow!(Self::error_message()))
			.map(Self::from)
	}

	async fn search_open_meteo(client: &Client, address: &str, language: &str) -> Result<Self> {
		client
			.get(ApiQuery::location(ApiName::OpenMeteo, address, language).convert().assemble())
			.send()
			.await?
			.json::<Vec<OpenMeteoGeoObj>>()
			.await?
			.first()
			.ok_or_else(|| anyhow!(Self::error_message()))
			.map(Self::from)
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

			let auto_loc = ApiQuery::geo_ip().query::<GeoIpLocation>().await?;
			return Ok(format!("{},{}", auto_loc.city_name, auto_loc.country_code));
		}

		// Handle address from args or config
		if arg_address == "auto" || (arg_address.is_empty() && config.address == "auto") {
			let auto_loc = ApiQuery::geo_ip().query::<GeoIpLocation>().await?;
			Ok(format!("{},{}", auto_loc.city_name, auto_loc.country_code))
		} else if !arg_address.is_empty() {
			Ok(arg_address.to_string())
		} else {
			Ok(config.address.to_string())
		}
	}
}

impl ErrorMessage for Location {
	fn error_message() -> String {
		String::from("Location request failed.")
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

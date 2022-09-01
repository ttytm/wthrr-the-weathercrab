use anyhow::{anyhow, Result};
use reqwest::{header::USER_AGENT, Client, Url};

use serde::{Deserialize, Serialize};

// Geoip json
#[derive(Serialize, Deserialize, Debug)]
pub struct Geolocation {
	pub latitude: f64,
	pub longitude: f64,
	pub city_name: String,
	pub country_code: String,
}

// Open street map(OSM) json
#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
	place_id: u64,
	licence: String,
	osm_type: String,
	osm_id: u64,
	boundingbox: Vec<String>,
	pub lat: String,
	pub lon: String,
	pub display_name: String,
	class: String,
	#[serde(rename(deserialize = "type"))]
	kind: String,
	importance: f64,
}

impl Geolocation {
	pub async fn get() -> Result<Geolocation> {
		let url: String = "https://api.geoip.rs/".to_string();
		let url = Url::parse(&*url)?;

		let res = reqwest::get(url).await?.json::<Geolocation>().await?;

		Ok(res)
	}

	pub async fn search(address: &str, lang: &str) -> Result<Vec<Address>> {
		let url: String = format!(
			"https://nominatim.openstreetmap.org/search?q={}&accept-language={}&limit=1&format=json",
			address, lang
		);

		let client = Client::new();
		let res = client
			.get(&url)
			.header(USER_AGENT, "wthrr-the-weathercrab")
			.send()
			.await?
			.json::<Vec<Address>>()
			.await?;

		if res.is_empty() {
			return Err(anyhow!("Failed getting location information."));
		}

		Ok(res)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_german_address_response() -> Result<()> {
		let (address, lang) = ("berlin", "de");

		let loc = Geolocation::search(address, lang).await?;

		assert!(loc[0].display_name.contains("Deutschland"));

		Ok(())
	}

	#[tokio::test]
	async fn test_polish_address_response() -> Result<()> {
		let (address, lang) = ("berlin", "pl");

		let loc = Geolocation::search(address, lang).await?;

		assert!(loc[0].display_name.contains("Niemcy"));

		Ok(())
	}
}

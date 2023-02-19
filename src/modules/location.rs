use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Address {
	pub name: String,
	pub lat: String,
	pub lon: String,
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

impl Address {
	async fn search_osm(client: &Client, address: &str, lang: &str) -> Result<Address> {
		let url = format!(
			"https://nominatim.openstreetmap.org/search?q={address}&accept-language={lang}&limit=1&format=jsonv2",
		);
		let results: Vec<OpenStreetMapGeoObj> = client.get(&url).send().await?.json().await?;
		let result = results.first().ok_or_else(|| anyhow!("Location request failed."))?;

		Ok(Address {
			name: result.display_name.clone(),
			lon: result.lon.to_string(),
			lat: result.lat.to_string(),
		})
	}

	async fn search_open_meteo(client: &Client) -> Result<Address> {
		let url = "https://geocoding-api.open-meteo.com/v1/search?name=Berlin&language=fr";
		let results: OpenMeteoResults = client.get(url).send().await?.json().await?;
		let result = results
			.results
			.first()
			.ok_or_else(|| anyhow!("Location request failed."))?;

		Ok(Address {
			name: result.name.clone(),
			lon: result.longitude.to_string(),
			lat: result.latitude.to_string(),
		})
	}

	pub async fn search(address: &str, lang: &str) -> Result<Address> {
		let client = Client::builder().user_agent("wthrr-the-weathercrab").build()?;
		let results = Self::search_osm(&client, address, lang).await;

		match results {
			Ok(address) => Ok(address),
			Err(_) => Self::search_open_meteo(&client).await,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn geolocation_response() -> Result<()> {
		let (address, lang_de, lang_pl) = ("berlin", "de", "pl");

		let loc_de = Address::search(address, lang_de).await?;
		let loc_pl = Address::search(address, lang_pl).await?;

		assert!(loc_de.name.contains("Deutschland"));
		assert!(loc_pl.name.contains("Niemcy"));

		Ok(())
	}
}

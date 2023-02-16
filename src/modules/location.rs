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
		let url = Url::parse("https://api.geoip.rs")?;

		let res = reqwest::get(url).await?.json::<Geolocation>().await?;

		Ok(res)
	}

	pub async fn search(address: &str, lang: &str) -> Result<Vec<Address>> {
		let url = format!(
			"https://nominatim.openstreetmap.org/search?q={address}&accept-language={lang}&limit=1&format=json"
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
			return Err(anyhow!("Location request failed."));
		}

		Ok(res)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn geolocation_response() -> Result<()> {
		let (address, lang_de, lang_pl) = ("berlin", "de", "pl");

		let loc_de = Geolocation::search(address, lang_de).await?;
		let loc_pl = Geolocation::search(address, lang_pl).await?;

		assert!(loc_de[0].display_name.contains("Deutschland"));
		assert!(loc_pl[0].display_name.contains("Niemcy"));

		Ok(())
	}
}

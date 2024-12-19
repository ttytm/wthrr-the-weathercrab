pub trait Api {
	fn assemble(&self) -> String;
}

impl Api for Box<dyn '_ + Api> {
	fn assemble(&self) -> String {
		self.as_ref().assemble()
	}
}

pub enum ApiName {
	GeoIp,
	OpenMeteo,
	OpenStreetMap,
}

pub struct ApiQuery<'a> {
	api: ApiName,
	address: &'a str,
	language: &'a str,
}

impl<'a> ApiQuery<'a> {
	pub fn convert(&self) -> Box<dyn 'a + Api> {
		match &self.api {
			ApiName::GeoIp => Box::new(GeoIpLocationQuery {}),
			ApiName::OpenMeteo => Box::new(OpenMeteoLocationQuery {
				address: self.address,
				language: self.language,
			}),
			ApiName::OpenStreetMap => Box::new(OpenStreetMapLocationQuery {
				address: self.address,
				language: self.language,
			}),
		}
	}

	pub fn geo_ip() -> Self {
		Self {
			api: ApiName::GeoIp,
			..Default::default()
		}
	}

	pub const fn location(api: ApiName, address: &'a str, language: &'a str) -> Self {
		Self { api, address, language }
	}

	pub async fn query<T>(&self) -> reqwest::Result<T>
	where
		T: for<'de> serde::Deserialize<'de>,
	{
		reqwest::get(self.convert().assemble()).await?.json::<T>().await
	}
}

impl Default for ApiQuery<'_> {
	fn default() -> Self {
		Self {
			api: ApiName::OpenMeteo,
			address: "",
			language: "",
		}
	}
}

pub trait ErrorMessage {
	fn error_message() -> String;
}

pub struct GeoIpLocationQuery;

impl Api for GeoIpLocationQuery {
	fn assemble(&self) -> String {
		String::from("https://api.geoip.rs")
	}
}

pub struct OpenMeteoLocationQuery<'a> {
	address: &'a str,
	language: &'a str,
}

impl Api for OpenMeteoLocationQuery<'_> {
	fn assemble(&self) -> String {
		format!(
			"https://geocoding-api.open-meteo.com/v1/search?name={}&language={}",
			self.address, self.language
		)
	}
}

pub struct OpenStreetMapLocationQuery<'a> {
	address: &'a str,
	language: &'a str,
}

impl Api for OpenStreetMapLocationQuery<'_> {
	fn assemble(&self) -> String {
		format!(
			"https://nominatim.openstreetmap.org/search?q={}&accept-language={}&limit=1&format=jsonv2",
			self.address, self.language
		)
	}
}

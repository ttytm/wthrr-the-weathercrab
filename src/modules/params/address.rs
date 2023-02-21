use anyhow::{anyhow, Result};
use dialoguer::{theme::ColorfulTheme, Confirm};

use crate::modules::location::GeoIpLocation;

use super::Params;

impl Params {
	pub async fn resolve_address(&mut self, arg_address: &str) -> Result<()> {
		if arg_address.is_empty() && self.config.address == "arg_input" {
			return Err(anyhow!("Your configuration requires you to specify a city."));
		};

		// Handle auto address via confirmation prompt
		if arg_address.is_empty() && self.config.address.is_empty() {
			if self.config.gui.greeting {
				// greeting without indentation to match dialoger prompt
				println!(" 🦀  {}", self.texts.greeting);
			}
			if Confirm::with_theme(&ColorfulTheme::default())
				.with_prompt(self.texts.search_station.clone())
				.interact()?
			{
				let auto_loc = GeoIpLocation::get().await?;
				self.config.address = format!("{},{}", auto_loc.city_name, auto_loc.country_code);
				return Ok(());
			} else {
				std::process::exit(1)
			};
		};

		// Handle address from args or config
		if self.config.gui.greeting {
			// greeting with indentation to match overall style
			println!("  🦀  {}", self.texts.greeting);
		}
		if arg_address == "auto" || (arg_address.is_empty() && self.config.address == "auto") {
			let auto_loc = GeoIpLocation::get().await?;
			self.config.address = format!("{},{}", auto_loc.city_name, auto_loc.country_code);
		} else if !arg_address.is_empty() {
			self.config.address = arg_address.to_string()
		};

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::modules::locales::Locales;
	use crate::modules::params::Config;

	#[tokio::test]
	async fn address_from_arg() -> Result<()> {
		let arg_address = "new york";
		let texts = Locales::default();
		let config = Config {
			address: "Berlin, DE".to_string(),
			..Default::default()
		};

		let mut params = Params { config, texts };
		params.resolve_address(arg_address).await?;

		assert!(params.config.address.contains("new york"));

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

		let mut params = Params { config, texts };
		params.resolve_address(arg_address).await?;

		assert!(params.config.address.contains("Berlin"));

		Ok(())
	}
}

use anyhow::{anyhow, Result};
use dialoguer::{theme::ColorfulTheme, Confirm};

use crate::modules::{display::greeting, location::GeoIpLocation, translation::translate};

use super::Params;

impl Params {
	pub async fn resolve_address(&mut self, arg_address: &str) -> Result<()> {
		if arg_address.is_empty() && self.address == "arg_input" {
			return Err(anyhow!("Your configuration requires you to specify a city."));
		};

		// Handle auto address via confirmation prompt
		if arg_address.is_empty() && self.address.is_empty() {
			// greeting without indentation to match dialoger prompt
			greeting::handle_greeting(self.gui.greeting, &self.language, false).await?;
			if Confirm::with_theme(&ColorfulTheme::default())
				.with_prompt(
					translate(
						&self.language,
						"You didn't specify a city. Should I check for a weather station close to your location?",
					)
					.await?,
				)
				.interact()?
			{
				let auto_loc = GeoIpLocation::get().await?;
				self.address = format!("{},{}", auto_loc.city_name, auto_loc.country_code);
				return Ok(());
			} else {
				std::process::exit(1)
			};
		};

		// Handle address from args or config
		// greeting with indentation to match overall style
		greeting::handle_greeting(self.gui.greeting, &self.language, true).await?;
		if arg_address == "auto" || (arg_address.is_empty() && self.address == "auto") {
			let auto_loc = GeoIpLocation::get().await?;
			self.address = format!("{},{}", auto_loc.city_name, auto_loc.country_code);
		} else if !arg_address.is_empty() {
			self.address = arg_address.to_string()
		};

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::modules::params::Params;

	#[tokio::test]
	async fn address_from_arg() -> Result<()> {
		let arg_address = "new york";
		let mut config = Params {
			address: "Berlin, DE".to_string(),
			..Default::default()
		};

		config.resolve_address(arg_address).await?;

		assert!(config.address.contains("new york"));

		Ok(())
	}

	#[tokio::test]
	async fn address_from_cfg() -> Result<()> {
		let arg_address = "";
		let mut config = Params {
			address: "Berlin, DE".to_string(),
			..Default::default()
		};

		config.resolve_address(arg_address).await?;

		assert!(config.address.contains("Berlin"));

		Ok(())
	}
}

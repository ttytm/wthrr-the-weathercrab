use anyhow::{anyhow, Result};
use dialoguer::{theme::ColorfulTheme, Confirm};

use crate::modules::{location::Geolocation, translation::translate};

pub async fn get(arg_address: &str, cfg_address: &str, lang: &str) -> Result<String> {
	let address = if arg_address == "auto" || (arg_address.is_empty() && cfg_address == "auto") {
		let auto_loc = Geolocation::get().await?;
		format!("{},{}", auto_loc.city_name, auto_loc.country_code)
	} else if arg_address.is_empty() && cfg_address == "arg_input" {
		return Err(anyhow!("Please specify a city."));
	} else if arg_address.is_empty() && cfg_address.is_empty() {
		if Confirm::with_theme(&ColorfulTheme::default())
			.with_prompt(
				translate(
					lang,
					"You didn't specify a city. Should I check for a weather station close to your location?",
				)
				.await?,
			)
			.interact()?
		{
			let auto_loc = Geolocation::get().await?;
			format!("{},{}", auto_loc.city_name, auto_loc.country_code)
		} else {
			std::process::exit(1)
		}
	} else if arg_address.is_empty() && !cfg_address.is_empty() {
		cfg_address.to_string()
	} else {
		arg_address.to_string()
	};

	Ok(address)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::modules::params::Params;

	#[tokio::test]
	async fn address_from_arg() -> Result<()> {
		let arg_address = "new york";
		let config = Params {
			address: "Berlin, DE".to_string(),
			..Default::default()
		};

		let res = get(arg_address, &config.address, &config.language).await?;

		assert!(res.contains("new york"));

		Ok(())
	}

	#[tokio::test]
	async fn address_from_cfg() -> Result<()> {
		let arg_address = "";
		let config = Params {
			address: "Berlin, DE".to_string(),
			..Default::default()
		};

		let res = get(arg_address, &config.address, &config.language).await?;

		assert!(res.contains("Berlin"));

		Ok(())
	}
}

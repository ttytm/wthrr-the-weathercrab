use anyhow::{anyhow, Result};
use dialoguer::{theme::ColorfulTheme, Confirm};

use crate::{location::Geolocation, translation::translate};

pub async fn get(address_arg: &str, address_cfg: &str, lang: &str) -> Result<String> {
	let address = if address_arg == "auto" || (address_arg.is_empty() && address_cfg == "auto") {
		let auto_loc = Geolocation::get().await?;
		format!("{},{}", auto_loc.city_name, auto_loc.country_code)
	} else if address_arg.is_empty() && address_cfg == "arg_input" {
		return Err(anyhow!("Please specify a city."));
	} else if address_arg.is_empty() && address_cfg.is_empty() {
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
	} else if address_arg.is_empty() && !address_cfg.is_empty() {
		address_cfg.to_string()
	} else {
		address_arg.to_string()
	};

	Ok(address)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::config::Config;

	#[tokio::test]
	async fn address_from_arg() -> Result<()> {
		let address_arg = "new york";
		let config = Config {
			address: Some("Berlin, DE".to_string()),
			..Default::default()
		};

		let res = get(
			address_arg,
			config.address.as_deref().unwrap_or_default(),
			&config.language.unwrap(),
		)
		.await?;

		assert!(res.contains("new york"));

		Ok(())
	}

	#[tokio::test]
	async fn address_from_cfg() -> Result<()> {
		let address_arg = "";
		let config = Config {
			address: Some("Berlin, DE".to_string()),
			..Default::default()
		};

		let res = get(
			address_arg,
			config.address.as_deref().unwrap_or_default(),
			&config.language.unwrap(),
		)
		.await?;

		assert!(res.contains("Berlin"));

		Ok(())
	}
}

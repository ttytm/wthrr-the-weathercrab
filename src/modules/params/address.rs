use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm};

use crate::{location::Geolocation, translation::translate};

pub async fn get(args_address: &str, config_address: &str, lang: &str) -> Result<String> {
	let address = if (args_address.is_empty() && config_address.is_empty())
		|| args_address == "auto"
		|| (args_address.is_empty() && config_address == "auto")
	{
		if (args_address.is_empty() && config_address.is_empty())
			&& !Confirm::with_theme(&ColorfulTheme::default())
				.with_prompt(
					translate(
						lang,
						"You didn't specify a city. Should I check for a weather station close to your location?",
					)
					.await?,
				)
				.interact()?
		{
			std::process::exit(1);
		}
		let auto_loc = Geolocation::get().await?;
		format!("{},{}", auto_loc.city_name, auto_loc.country_code)
	} else if args_address.is_empty() && !config_address.is_empty() {
		config_address.to_string()
	} else {
		args_address.to_string()
	};

	Ok(address)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::config::Config;

	#[tokio::test]
	async fn address_from_arg() -> Result<()> {
		let arg_address = "new york";
		let config = Config {
			address: Some("Berlin, DE".to_string()),
			..Default::default()
		};

		let res = get(arg_address, config.address.as_deref().unwrap_or_default(), "en").await?;

		assert!(res.contains("new york"));

		Ok(())
	}

	#[tokio::test]
	async fn address_from_cfg() -> Result<()> {
		let arg_address = "";
		let config = Config {
			address: Some("Berlin, DE".to_string()),
			..Default::default()
		};

		let res = get(arg_address, config.address.as_deref().unwrap_or_default(), "en").await?;

		assert!(res.contains("Berlin"));

		Ok(())
	}
}

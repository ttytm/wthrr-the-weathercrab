use anyhow::{anyhow, Result};
use dialoguer::{theme::ColorfulTheme, Confirm};

use crate::args::Args;
use crate::config::Config;
use crate::location::Geolocation;

pub async fn get(args: &Args, config: &Config) -> Result<Config> {
	let address = prep_address(args.address.as_deref().unwrap_or_default().to_string(), config).await?;
	let unit = prep_unit(args.unit.to_string(), config.unit.as_ref())?;

	Ok(Config {
		address: Some(address),
		unit: Some(unit),
		method: None,
	})
}

async fn prep_address(args_address: String, config: &Config) -> Result<String> {
	if args_address.is_empty() && config.method.as_deref().unwrap_or_default() == "manual" {
		return Err(anyhow!("Please specify a city."));
	}

	let address = if args_address == "auto"
		|| config.method.as_deref().unwrap_or_default() == "auto"
		|| args_address.is_empty() && config.address.is_none()
	{
		if args_address.is_empty() {
			let auto_location_prompt = Confirm::with_theme(&ColorfulTheme::default())
				.with_prompt(
					"You didn't specified a city. Should I check for a weather station close to your location?",
				)
				.interact()?;
			if !auto_location_prompt {
				std::process::exit(1);
			}
		}
		let auto_loc = Geolocation::get().await?;
		format!("{},{}", &auto_loc.city_name, &auto_loc.country_code)
	} else if args_address.is_empty() && config.address.is_some() {
		config.address.as_deref().unwrap().to_string()
	} else {
		args_address
	};

	Ok(address)
}

fn prep_unit(args_unit: String, config_unit: Option<&String>) -> Result<String> {
	let unit = if !args_unit.is_empty() && config_unit.is_some() {
		match config_unit {
			unit if unit == Some(&"°F".to_string()) => "fahrenheit",
			_ => "celsius",
		}
	} else if args_unit == "f" || args_unit == "fahrenheit" {
		"fahrenheit"
	} else {
		"celsius"
	};
	Ok(unit.to_string())
}

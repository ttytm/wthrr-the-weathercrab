use anyhow::{anyhow, Result};
use dialoguer::{theme::ColorfulTheme, Confirm};

use crate::{
	args::Args,
	config::{Config, TempUnit},
	location::Geolocation,
};

pub async fn get(args: &Args, config: &Config) -> Result<Config> {
	let address = prep_address(args.address.as_deref().unwrap_or_default().to_string(), &config).await?;
	let unit = prep_unit(
		args.unit.as_deref().unwrap_or_default().to_string(),
		config.unit.as_ref(),
	)?;

	Ok(Config {
		address: Some(address),
		unit: Some(unit),
		..Config::clone(config)
	})
}

async fn prep_address(args_address: String, config: &Config) -> Result<String> {
	if args_address.is_empty() && config.method.as_deref().unwrap_or_default() == "manual" {
		return Err(anyhow!("Please specify a city."));
	}

	let address = if args_address == "auto"
		|| args_address.is_empty()
			&& (config.method.as_deref().unwrap_or_default() == "auto" || config.method.is_none())
	{
		if args_address.is_empty() && config.method.is_none() {
			let auto_location_prompt = Confirm::with_theme(&ColorfulTheme::default())
				.with_prompt("You didn't specify a city. Should I check for a weather station close to your location?")
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

fn prep_unit(args_unit: String, config_unit: Option<&TempUnit>) -> Result<TempUnit> {
	let unit = if args_unit.is_empty() && config_unit.is_some() {
		match config_unit {
			unit if unit == Some(&TempUnit::Fahrenheit) => TempUnit::Fahrenheit,
			_ => TempUnit::Celsius,
		}
	} else if args_unit == "f" || args_unit == "fahrenheit" {
		TempUnit::Fahrenheit
	} else {
		TempUnit::Celsius
	};

	Ok(unit)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_temp_unit_from_arg() -> Result<()> {
		let arg_unit = "f".to_string();
		let cfg_unit = TempUnit::Celsius;

		assert_eq!(prep_unit(arg_unit, Some(&cfg_unit))?, TempUnit::Fahrenheit);

		Ok(())
	}

	#[test]
	fn test_temp_unit_from_cfg() -> Result<()> {
		let arg_unit = String::new();
		let cfg_unit = TempUnit::Fahrenheit;

		assert_eq!(prep_unit(arg_unit, Some(&cfg_unit))?, TempUnit::Fahrenheit);

		Ok(())
	}

	#[test]
	fn test_temp_unit_fallback() -> Result<()> {
		let arg_unit = "a".to_string();

		assert_eq!(prep_unit(arg_unit, None)?, TempUnit::Celsius);

		Ok(())
	}

	#[tokio::test]
	async fn test_address_from_arg() -> Result<()> {
		let arg_address = "new york".to_string();
		let config = Config {
			address: Some("Berlin, DE".to_string()),
			unit: Some(TempUnit::Fahrenheit),
			..Default::default()
		};

		assert!(prep_address(arg_address, &config).await?.contains("new york"));

		Ok(())
	}

	#[tokio::test]
	async fn test_address_from_cfg() -> Result<()> {
		let arg_address = String::new();
		let config = Config {
			address: Some("Berlin, DE".to_string()),
			unit: Some(TempUnit::Fahrenheit),
			..Default::default()
		};

		assert!(prep_address(arg_address, &config).await?.contains("Berlin"));

		Ok(())
	}
}

use anyhow::{anyhow, Result};
use dialoguer::{theme::ColorfulTheme, Confirm};
use strum_macros::Display;

use crate::{args::Args, config::Config, location::Geolocation};

pub struct Params {
	pub address: String,
	pub unit: TempUnit,
}

#[derive(Display)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum TempUnit {
	Fahrenheit,
	Celsius,
}

pub async fn get(args: &Args, config: &Config) -> Result<Params> {
	let address = prep_address(args.address.as_deref().unwrap_or_default().to_string(), config).await?;
	let unit = prep_unit(
		args.unit.as_deref().unwrap_or_default().to_string(),
		config.unit.as_ref(),
	)?;

	Ok(Params { address, unit })
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

fn prep_unit(args_unit: String, config_unit: Option<&String>) -> Result<TempUnit> {
	let unit = if args_unit.is_empty() && config_unit.is_some() {
		match config_unit {
			unit if unit == Some(&String::from("Fahrenheit")) => TempUnit::Fahrenheit,
			// Support configs prior params unit enum. Deprecate in the future.
			unit if unit == Some(&String::from("°F")) => TempUnit::Fahrenheit,
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
		let cfg_unit = Some("°C".to_string());

		assert_eq!(prep_unit(arg_unit, cfg_unit.as_ref())?, TempUnit::Fahrenheit);

		Ok(())
	}

	#[test]
	fn test_temp_unit_from_cfg() -> Result<()> {
		let arg_unit = String::new();
		let cfg_unit = Some("°F".to_string());

		assert_eq!(prep_unit(arg_unit, cfg_unit.as_ref())?, TempUnit::Fahrenheit);

		Ok(())
	}

	#[test]
	fn test_temp_unit_fallback() -> Result<()> {
		let arg_unit = "a".to_string();
		let cfg_unit = Some(String::new());

		assert_eq!(prep_unit(arg_unit, cfg_unit.as_ref())?, TempUnit::Celsius);

		Ok(())
	}

	#[tokio::test]
	async fn test_address_from_arg() -> Result<()> {
		let arg_address = "new york".to_string();
		let config = Config {
			address: Some("Berlin, DE".to_string()),
			unit: Some("°F".to_string()),
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
			unit: Some("°F".to_string()),
			method: Some("default".to_string()),
			..Default::default()
		};

		assert!(prep_address(arg_address, &config).await?.contains("Berlin"));

		Ok(())
	}
}

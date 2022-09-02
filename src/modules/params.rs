use anyhow::{anyhow, Result};
use dialoguer::{theme::ColorfulTheme, Confirm};

use crate::{
	args::Args,
	config::{Config, TempUnit},
	location::Geolocation,
	translation::translate,
};

pub async fn get(args: &Args, config: &Config) -> Result<Config> {
	let lang = prep_lang(
		args.language.as_deref().unwrap_or_default(),
		config.language.as_deref().unwrap_or_default(),
	)?;

	let address = prep_address(
		args.address.as_deref().unwrap_or_default(),
		config.address.as_deref().unwrap_or_default(),
		config.method.as_deref().unwrap_or_default(),
		&lang,
	)
	.await?;

	let unit = prep_unit(args.unit.as_deref().unwrap_or_default(), config.unit.as_ref())?;

	let greeting = prep_greeting(args.greeting, config.greeting)?;

	Ok(Config {
		address: Some(address),
		unit: Some(unit),
		language: Some(lang.to_string()),
		greeting: Some(greeting),
		..Config::clone(config)
	})
}

async fn prep_address(args_address: &str, config_address: &str, config_method: &str, lang: &str) -> Result<String> {
	if args_address.is_empty() && config_method == "manual" {
		return Err(anyhow!(translate(&lang, "Please specify a city.").await?));
	}

	let address =
		if args_address == "auto" || args_address.is_empty() && (config_method == "auto" || config_method.is_empty()) {
			if args_address.is_empty() && config_method.is_empty() {
				let auto_location_prompt = Confirm::with_theme(&ColorfulTheme::default())
					.with_prompt(
						translate(
							&lang,
							"You didn't specify a city. Should I check for a weather station close to your location?",
						)
						.await?,
					)
					.interact()?;
				if !auto_location_prompt {
					std::process::exit(1);
				}
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

fn prep_unit(args_unit: &str, config_unit: Option<&TempUnit>) -> Result<TempUnit> {
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

fn prep_lang(args_lang: &str, config_lang: &str) -> Result<String> {
	let lang = if !args_lang.is_empty() {
		args_lang
	} else if args_lang.is_empty() && !config_lang.is_empty() {
		config_lang
	} else {
		"en"
	};

	Ok(lang.to_string())
}

fn prep_greeting(args_toggle_greeting: bool, config_greet: Option<bool>) -> Result<bool> {
	if !args_toggle_greeting && config_greet.is_none() {
		return Ok(true);
	}

	let greet = match args_toggle_greeting {
		true => !config_greet.unwrap(),
		_ => config_greet.unwrap(),
	};

	Ok(greet)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_temp_unit_from_arg() -> Result<()> {
		let arg_unit = "f";
		let cfg_unit = TempUnit::Celsius;

		assert_eq!(prep_unit(arg_unit, Some(&cfg_unit))?, TempUnit::Fahrenheit);

		Ok(())
	}

	#[test]
	fn test_temp_unit_from_cfg() -> Result<()> {
		let arg_unit = "";
		let cfg_unit = TempUnit::Fahrenheit;

		assert_eq!(prep_unit(arg_unit, Some(&cfg_unit))?, TempUnit::Fahrenheit);

		Ok(())
	}

	#[test]
	fn test_temp_unit_fallback() -> Result<()> {
		let arg_unit = "a";

		assert_eq!(prep_unit(arg_unit, None)?, TempUnit::Celsius);

		Ok(())
	}

	#[test]
	fn test_handle_greeting_from_arg() -> Result<()> {
		// Toggle flag while greeting enabled in cfg
		assert_eq!(prep_greeting(true, Some(true))?, false);
		// Toggle flag while greeting disabled in cfg
		assert_eq!(prep_greeting(true, Some(false))?, true);

		Ok(())
	}

	#[test]
	fn test_handle_greeting_from_cfg() -> Result<()> {
		assert_eq!(prep_greeting(false, Some(true))?, true);
		assert_eq!(prep_greeting(false, Some(false))?, false);

		Ok(())
	}

	#[tokio::test]
	async fn test_address_from_arg() -> Result<()> {
		let arg_address = "new york";
		let config = Config {
			address: Some("Berlin, DE".to_string()),
			unit: Some(TempUnit::Fahrenheit),
			..Default::default()
		};

		assert!(prep_address(
			arg_address,
			config.address.as_deref().unwrap_or_default(),
			config.method.as_deref().unwrap_or_default(),
			"en"
		)
		.await?
		.contains("new york"));

		Ok(())
	}

	#[tokio::test]
	async fn test_address_from_cfg() -> Result<()> {
		let arg_address = "";
		let config = Config {
			address: Some("Berlin, DE".to_string()),
			unit: Some(TempUnit::Fahrenheit),
			..Default::default()
		};

		assert!(prep_address(
			arg_address,
			config.address.as_deref().unwrap_or_default(),
			config.method.as_deref().unwrap_or_default(),
			"en"
		)
		.await?
		.contains("Berlin"));

		Ok(())
	}

	#[test]
	fn test_lang_from_arg() -> Result<()> {
		let arg_lang = "pl";
		let config = Config {
			language: Some("de".to_string()),
			..Default::default()
		};

		assert!(prep_lang(arg_lang, config.language.as_deref().unwrap_or_default())?.contains("pl"));

		Ok(())
	}

	#[test]
	fn test_lang_from_cfg() -> Result<()> {
		let arg_lang = "";
		let config = Config {
			language: Some("de".to_string()),
			..Default::default()
		};

		assert!(prep_lang(arg_lang, config.language.as_deref().unwrap_or_default())?.contains("de"));

		Ok(())
	}
}

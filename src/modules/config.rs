use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use serde::{Deserialize, Serialize};

use crate::{args::Args, confy::lib, params::TempUnit, Product};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
	pub address: Option<String>,
	pub unit: Option<String>,
	pub method: Option<String>,
	pub greeting: Option<bool>,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			address: None,
			unit: Some(TempUnit::Celsius.to_string()),
			method: Some("default".to_string()),
			greeting: Some(true),
		}
	}
}

impl Config {
	pub fn handle_next(args: &Args, config: Config, product: Product) -> Result<()> {
		if !args.save_config && (config.address.is_some() || config.method.as_deref().unwrap_or_default() == "manual") {
			return Ok(());
		}

		let unit = if product.weather.hourly_units.temperature_2m.contains("F") {
			TempUnit::Fahrenheit
		} else {
			TempUnit::Celsius
		};

		let new_config = Config {
			address: Some(product.address),
			unit: Some(unit.to_string()),
			..Default::default()
		};

		if args.save_config {
			lib::store("weathercrab", "wthrr", &new_config)?;
		} else if config.address.is_none() {
			Config::save_prompt(new_config, args.address.as_deref().unwrap_or_default().to_string())?;
		}

		Ok(())
	}

	fn save_prompt(mut new_config: Config, args_address: String) -> Result<()> {
		let include_auto_location = args_address.is_empty() || args_address == "auto";

		let mut items = vec!["Yes please", "No, ask me next time", "No, dont ask me again"];
		if include_auto_location {
			items.push("Always check for a weather station")
		}

		let selection = Select::new()
			.with_prompt("Would you like to use this as your default location?")
			.items(&items)
			.default(0)
			.interact()?;

		match selection {
			0 => {}
			1 => return Ok(()),
			2 => {
				new_config = Config {
					address: None,
					method: Some("manual".to_string()),
					..new_config
				}
			}
			3 => new_config.method = Some("auto".to_string()),
			_ => println!("User did not select anything or exited using Esc or q"),
		}

		lib::store("weathercrab", "wthrr", &new_config)?;

		Ok(())
	}

	pub fn reset() -> Result<()> {
		let confirmation = Confirm::with_theme(&ColorfulTheme::default())
			.with_prompt("This will wipe wthrr's configuration. Continue?")
			.interact()?;

		if confirmation {
			let file = lib::get_configuration_file_path("weathercrab", "wthrr")?;

			std::fs::remove_dir_all(file.parent().unwrap()).with_context(|| "Error resetting config file.")?;
		}

		Ok(())
	}
}

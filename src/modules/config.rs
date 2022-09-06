use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use serde::{Deserialize, Serialize};
use std::convert::AsRef;
use strum_macros::{AsRefStr, Display, EnumString};

use crate::{args::Args, confy::lib, translation::translate};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
	pub address: Option<String>,
	pub unit: Option<String>,
	pub method: Option<String>,
	pub greeting: Option<bool>,
	pub language: Option<String>,
}

impl Default for Config {
	fn default() -> Self {
		Config {
			address: None,
			unit: Some(TempUnit::Celsius.as_ref().to_string()),
			method: Some("default".to_string()),
			greeting: Some(true),
			language: Some("en".to_string()),
		}
	}
}

#[derive(Display, EnumString, Serialize, Deserialize, Debug, PartialEq, Clone, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum TempUnit {
	Celsius,
	Fahrenheit,
}

impl Config {
	pub async fn handle_next(&self, args: Args, params: Config) -> Result<()> {
		if !args.save_config && (self.address.is_some() || self.method.as_deref().unwrap_or_default() == "manual") {
			return Ok(());
		}

		let new_config = Config {
			address: Some(params.address.unwrap()),
			unit: Some(params.unit.unwrap()),
			language: Some(params.language.unwrap()),
			..Default::default()
		};

		if args.save_config {
			lib::store("weathercrab", "wthrr", &new_config)?;
		} else if self.address.is_none() {
			Self::save_prompt(new_config, args.address.as_deref().unwrap_or_default().to_string()).await?;
		}

		Ok(())
	}

	async fn save_prompt(mut new_config: Config, args_address: String) -> Result<()> {
		let include_auto_location = args_address.is_empty() || args_address == "auto";

		let mut items = vec![
			translate(new_config.language.as_ref().unwrap(), "Yes please").await?,
			translate(new_config.language.as_ref().unwrap(), "No, ask me next time").await?,
			translate(new_config.language.as_ref().unwrap(), "No, dont ask me again").await?,
		];

		if include_auto_location {
			items.push(
				translate(
					new_config.language.as_ref().unwrap(),
					"Always check for a weather station",
				)
				.await?,
			)
		}

		let prompt_translated = translate(
			new_config.language.as_ref().unwrap(),
			"Would you like to use this as your default location?",
		)
		.await?;

		let selection = Select::new()
			.with_prompt(prompt_translated)
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

	pub async fn reset(lang: &str) -> Result<()> {
		let confirmation = Confirm::with_theme(&ColorfulTheme::default())
			.with_prompt(translate(lang, "This will wipe wthrr's configuration. Continue?").await?)
			.interact()?;

		if confirmation {
			let file = lib::get_configuration_file_path("weathercrab", "wthrr")?;

			std::fs::remove_dir_all(file.parent().unwrap()).with_context(|| "Error resetting config file.")?;
		}

		Ok(())
	}
}

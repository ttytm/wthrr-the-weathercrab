use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use serde::{Deserialize, Serialize};

use crate::{
	args::Cli,
	params::{
		unit::{SpeedUnit, TempUnit},
		Params,
	},
	translation::translate,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
	pub address: Option<String>,
	pub units: Option<String>,
	pub greeting: Option<bool>,
	pub language: Option<String>,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			address: None,
			units: Some(format!(
				"{},{}",
				TempUnit::default().as_ref(),
				SpeedUnit::default().as_ref(),
			)),
			greeting: Some(true),
			language: Some("en".to_string()),
		}
	}
}

impl Config {
	pub async fn handle_next(&self, args: Cli, params: Params) -> Result<()> {
		if !args.save && self.address.is_some() {
			return Ok(());
		}

		let new_config = Config {
			address: if self.address.is_some() && args.address.as_deref().unwrap_or_default() == "auto" {
				Some("auto".to_string())
			} else {
				Some(params.address)
			},
			units: Some(format!(
				"{},{}",
				params.units.temperature.as_ref(),
				params.units.speed.as_ref()
			)),
			greeting: Some(params.greeting),
			language: Some(params.language),
		};

		if self.address.is_none() {
			Self::save_prompt(new_config, args.address.as_deref().unwrap_or_default().to_string()).await?;
		} else {
			confy::store("weathercrab", "wthrr", &new_config)?;
		}

		Ok(())
	}

	async fn save_prompt(mut new_config: Config, args_address: String) -> Result<()> {
		let mut items = vec![
			translate(new_config.language.as_ref().unwrap(), "Yes please").await?,
			translate(new_config.language.as_ref().unwrap(), "No, ask me next time").await?,
			translate(new_config.language.as_ref().unwrap(), "No, dont ask me again").await?,
		];

		if args_address.is_empty() || args_address == "auto" {
			items.push(
				translate(
					new_config.language.as_ref().unwrap(),
					"Always check for a weather station",
				)
				.await?,
			)
		}

		let selection = Select::new()
			.with_prompt(
				translate(
					new_config.language.as_ref().unwrap(),
					"Would you like to use this as your default?",
				)
				.await?,
			)
			.items(&items)
			.default(0)
			.interact()?;

		match selection {
			0 => {}
			1 => return Ok(()),
			2 => new_config.address = None,
			3 => new_config.address = Some("auto".to_string()),
			_ => println!("User did not select anything or exited using Esc or q"),
		}

		confy::store("weathercrab", "wthrr", &new_config)?;

		Ok(())
	}

	pub async fn reset(lang: &str) -> Result<()> {
		let confirmation = Confirm::with_theme(&ColorfulTheme::default())
			.with_prompt(translate(lang, "This will wipe wthrr's configuration. Continue?").await?)
			.interact()?;

		if confirmation {
			let file = confy::get_configuration_file_path("weathercrab", "wthrr")?;

			std::fs::remove_dir_all(file.parent().unwrap()).with_context(|| "Error resetting config file.")?;
		}

		Ok(())
	}
}

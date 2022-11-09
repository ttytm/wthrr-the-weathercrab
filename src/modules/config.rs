use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use serde::{Deserialize, Serialize};

use crate::{
	args::{Cli, Forecast},
	display::border::BorderVariant,
	params::{units::Units, Params},
	translation::translate,
};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Config {
	pub address: Option<String>,
	pub language: Option<String>,
	pub forecast: Option<Vec<Forecast>>,
	pub units: Option<Units>,
	pub gui: Option<Gui>,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			address: None,
			forecast: None,
			language: Some("en".to_string()),
			units: Some(Units::default()),
			gui: Some(Gui::default()),
		}
	}
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Gui {
	pub border: Option<BorderVariant>,
	pub greeting: Option<bool>,
}

impl Default for Gui {
	fn default() -> Self {
		Self {
			border: Some(BorderVariant::default()),
			greeting: Some(true),
		}
	}
}

impl Config {
	pub async fn handle_next(args: Cli, params: Params) -> Result<()> {
		let config: Config = confy::load("weathercrab", "wthrr")?;

		if !args.save && config.address.is_some() {
			return Ok(());
		}

		let new_config = Config {
			address: if config.address.is_some() && args.address.as_deref().unwrap_or_default() == "auto" {
				Some("auto".to_string())
			} else {
				Some(params.address)
			},
			language: Some(params.language),
			forecast: if !params.forecast.is_empty() {
				Some(params.forecast)
			} else {
				None
			},
			units: Some(params.units),
			gui: Some(params.gui),
		};

		if config.address.is_none() {
			new_config
				.save_prompt(args.address.as_deref().unwrap_or_default().to_string())
				.await?;
		} else {
			confy::store("weathercrab", "wthrr", &new_config)?;
		}

		Ok(())
	}

	async fn save_prompt(mut self, args_address: String) -> Result<()> {
		let mut items = vec![
			translate(self.language.as_ref().unwrap(), "Yes please").await?,
			translate(self.language.as_ref().unwrap(), "No, ask me next time").await?,
			translate(self.language.as_ref().unwrap(), "No, dont ask me again").await?,
		];

		if args_address.is_empty() || args_address == "auto" {
			items.push(translate(self.language.as_ref().unwrap(), "Always check for a weather station").await?)
		}

		let selection = Select::new()
			.with_prompt(
				translate(
					self.language.as_ref().unwrap(),
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
			2 => self.address = None,
			3 => self.address = Some("auto".to_string()),
			_ => println!("User did not select anything or exited using Esc or q"),
		}

		confy::store("weathercrab", "wthrr", &self)?;

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

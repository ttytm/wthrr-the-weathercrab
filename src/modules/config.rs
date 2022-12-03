use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use serde::{Deserialize, Serialize};

use crate::modules::{
	args::{Cli, Forecast},
	display::{border::BorderVariant, graph::GraphVariant},
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
			language: Some("en_US".to_string()),
			units: Some(Units::default()),
			gui: Some(Gui::default()),
		}
	}
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Gui {
	pub border: Option<BorderVariant>,
	pub color: Option<ColorVariant>,
	pub graph: Option<GraphVariant>,
	pub greeting: Option<bool>,
}

impl Default for Gui {
	fn default() -> Self {
		Self {
			border: Some(BorderVariant::default()),
			color: Some(ColorVariant::default),
			graph: Some(GraphVariant::default()),
			greeting: Some(true),
		}
	}
}

#[derive(Default, Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ColorVariant {
	#[default]
	default,
	plain,
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

	async fn save_prompt(mut self, address_arg: String) -> Result<()> {
		let mut items = vec![
			translate(self.language.as_ref().unwrap(), "Yes please").await?,
			translate(self.language.as_ref().unwrap(), "No, ask me next time").await?,
			translate(self.language.as_ref().unwrap(), "No, dont ask me again").await?,
		];

		if address_arg.is_empty() || address_arg == "auto" {
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
			2 => self.address = Some("arg_input".to_string()),
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

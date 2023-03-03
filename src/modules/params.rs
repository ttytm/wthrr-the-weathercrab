use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use optional_struct::Applyable;
use serde::{Deserialize, Serialize};

use crate::modules::{
	args::{Cli, Forecast},
	config::Config,
	localization::{ConfigLocales, Locales},
	location::Location,
	units::Units,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
	pub config: Config,
	pub texts: Locales,
}

impl Params {
	pub async fn merge(config: &Config, args: &Cli) -> Result<Self> {
		let language = match &args.language {
			Some(lang) => lang.to_string(),
			_ => config.language.to_string(),
		};

		let texts = Locales::get(&language).await?;

		if args.reset {
			Self::reset(&texts.config).await?;
			std::process::exit(1);
		}

		let forecast = match !args.forecast.is_empty() {
			true => args.forecast.to_vec(),
			_ => config.forecast.to_vec(),
		};

		let units = Units::merge(&args.units, &config.units);

		let address = Location::resolve_input(args.address.as_deref().unwrap_or_default(), config, &texts).await?;

		Ok(Self {
			config: Config {
				language,
				forecast,
				units,
				address,
				gui: config.gui.clone(),
			},
			texts,
		})
	}

	pub async fn handle_next(mut self, args: Cli, mut config_file: Config) -> Result<()> {
		if !args.save && !config_file.address.is_empty() {
			return Ok(());
		}

		self.config.forecast.retain(|forecast| *forecast != Forecast::disable);

		if config_file.address.is_empty() {
			// offer to save
			self.config.apply_to(&mut config_file);
			self.config = config_file;
			self.save_prompt(&args.address.unwrap_or_default()).await?;
		} else {
			// handle explicit save call
			self.config.apply_to(&mut config_file);
			config_file.store();
			self.texts.store(&config_file.language);
		}

		Ok(())
	}

	async fn save_prompt(mut self, arg_address: &str) -> Result<()> {
		let mut items = vec![
			&self.texts.config.confirm,
			&self.texts.config.next_time,
			&self.texts.config.deny,
		];

		if arg_address.is_empty() || arg_address == "auto" {
			items.push(&self.texts.config.always_auto);
		}

		let selection = Select::with_theme(&ColorfulTheme::default())
			.with_prompt(&self.texts.config.save_as_default)
			.items(&items)
			.default(0)
			.interact()?;

		match selection {
			0 => {}
			1 => return Ok(()),
			2 => self.config.address = "arg_input".to_string(),
			3 => self.config.address = "auto".to_string(),
			_ => println!("{}", self.texts.config.no_selection),
		}

		self.config.store();
		self.texts.store(&self.config.language);

		Ok(())
	}

	pub async fn reset(t: &ConfigLocales) -> Result<()> {
		let confirmation = Confirm::with_theme(&ColorfulTheme::default())
			.with_prompt(&t.reset_config)
			.interact()?;

		if confirmation {
			let path = Config::get_path();

			std::fs::remove_dir_all(path.parent().unwrap()).with_context(|| "Error resetting config file.")?;
		}

		Ok(())
	}
}

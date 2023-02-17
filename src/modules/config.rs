use colored::{Color::Yellow, Colorize};
use directories::ProjectDirs;
use optional_struct::Applyable;
use ron::{
	extensions::Extensions,
	ser::{to_string_pretty, PrettyConfig},
	Options,
};
use std::{
	fs::{self, File},
	io::Write,
	path::PathBuf,
};

use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};

use crate::modules::{
	args::Cli,
	params::{ConfigFile, Params},
	translation::translate,
};

const CONFIG_DIR_NAME: &str = "weathercrab";
const CONFIG_FILE_NAME: &str = "wthrr.ron";

impl Params {
	fn get_path() -> PathBuf {
		ProjectDirs::from("", "", CONFIG_DIR_NAME)
			.unwrap()
			.config_dir()
			.join(CONFIG_FILE_NAME)
	}

	pub fn get_config_file() -> Params {
		let mut config = Params::default();
		let path = Self::get_path();

		if let Ok(file) = fs::read_to_string(&path) {
			let cfg_form_file: ConfigFile = match Options::default()
				.with_default_extension(Extensions::IMPLICIT_SOME)
				.from_str(&file)
			{
				Ok(contents) => contents,
				Err(error) => {
					let warning_sign = " Warning:".color(Yellow);
					println!(
						"{warning_sign} {}\n{: >4}At: {error}.\n{: >4}Falling back to default values.\n",
						path.display(),
						"",
						"",
					);
					return config;
				}
			};

			cfg_form_file.apply_to(&mut config);
		}

		config
	}

	fn store(&self) {
		let path = Self::get_path();

		let cfg_dir = path.parent().unwrap();
		if !cfg_dir.is_dir() {
			fs::create_dir(cfg_dir).unwrap();
		};

		let mut file = File::create(path).unwrap();
		let output = to_string_pretty(self, PrettyConfig::default()).unwrap();
		file.write_all(output.as_bytes()).unwrap();
	}

	pub async fn handle_next(self, args: Cli, mut config: Self) -> Result<()> {
		if !args.save && !config.address.is_empty() {
			return Ok(());
		}

		if config.address.is_empty() {
			self.apply_to(&mut config);
			config
				.save_prompt(args.address.as_deref().unwrap_or_default().to_string())
				.await?;
		} else {
			self.apply_to(&mut config);
			Self::store(&config);
		}

		Ok(())
	}

	async fn save_prompt(mut self, arg_address: String) -> Result<()> {
		let mut items = vec![
			translate(&self.language, "Yes please").await?,
			translate(&self.language, "No, ask me next time").await?,
			translate(&self.language, "No, dont ask me again").await?,
		];

		if arg_address.is_empty() || arg_address == "auto" {
			items.push(translate(&self.language, "Always check for a weather station").await?)
		}

		let selection = Select::with_theme(&ColorfulTheme::default())
			.with_prompt(translate(&self.language, "Would you like to use this as your default?").await?)
			.items(&items)
			.default(0)
			.interact()?;

		match selection {
			0 => {}
			1 => return Ok(()),
			2 => self.address = "arg_input".to_string(),
			3 => self.address = "auto".to_string(),
			_ => println!("User did not select anything or exited using Esc or q"),
		}

		Self::store(&self);

		Ok(())
	}

	pub async fn reset(lang: &str) -> Result<()> {
		let confirmation = Confirm::with_theme(&ColorfulTheme::default())
			.with_prompt(translate(lang, "This will wipe wthrr's configuration. Continue?").await?)
			.interact()?;

		if confirmation {
			let path = Self::get_path();

			std::fs::remove_dir_all(path.parent().unwrap()).with_context(|| "Error resetting config file.")?;
		}

		Ok(())
	}
}

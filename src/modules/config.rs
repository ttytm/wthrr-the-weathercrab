use anyhow::Result;
use dialoguer::console::style;
use directories::ProjectDirs;
use optional_struct::{optional_struct, Applicable};
use ron::{
	extensions::Extensions,
	ser::{to_string_pretty, PrettyConfig},
	Options,
};
use serde::{Deserialize, Serialize};
use std::{
	collections::HashSet,
	fs::{self, File},
	io::Write,
	path::PathBuf,
};

use super::{
	args::Forecast,
	display::gui_config::{ConfigFileGui, Gui},
	units::{ConfigFileUnits, Units},
};

#[optional_struct(ConfigFile)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
	pub address: String,
	pub language: String,
	pub forecast: HashSet<Forecast>,
	#[optional_rename(ConfigFileUnits)]
	pub units: Units,
	#[optional_rename(ConfigFileGui)]
	pub gui: Gui,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			address: String::new(),
			forecast: HashSet::new(),
			language: "en_US".to_string(),
			units: Units::default(),
			gui: Gui::default(),
		}
	}
}

pub const CONFIG_DIR_NAME: &str = "weathercrab";
const CONFIG_FILE_NAME: &str = "wthrr.ron";

impl Config {
	pub fn get() -> Self {
		let mut config = Self::default();
		let path = Self::get_path();

		if let Ok(file) = fs::read_to_string(&path) {
			match Options::default()
				.with_default_extension(Extensions::IMPLICIT_SOME)
				.from_str::<ConfigFile>(&file)
			{
				Ok(contents) => contents.apply_to(&mut config),
				Err(error) => {
					let warning_sign = style(" Warning:").yellow();
					eprintln!(
						"{warning_sign} {}\n{: >4}At: {error}.\n{: >4}Falling back to default values.\n",
						path.display(),
						"",
						"",
					);
					return config;
				}
			};
		}

		config
	}

	pub fn store(&self) -> Result<()> {
		let path = Self::get_path();

		let cfg_dir = path.parent().unwrap();
		if !cfg_dir.is_dir() {
			fs::create_dir_all(cfg_dir)?;
		};

		let mut file = File::create(path)?;
		let output = to_string_pretty(self, PrettyConfig::default()).unwrap();
		file.write_all(output.as_bytes())?;

		Ok(())
	}

	pub fn get_path() -> PathBuf {
		ProjectDirs::from("", "", CONFIG_DIR_NAME)
			.unwrap()
			.config_dir()
			.join(CONFIG_FILE_NAME)
	}
}

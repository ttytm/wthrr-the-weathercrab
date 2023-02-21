use colored::{Color::Yellow, Colorize};
use directories::ProjectDirs;
use optional_struct::*;
use ron::{
	extensions::Extensions,
	ser::{to_string_pretty, PrettyConfig},
	Options,
};
use serde::{Deserialize, Serialize};
use std::{
	fs::{self, File},
	io::Write,
	path::PathBuf,
};

use crate::modules::{
	args::Forecast,
	params::{
		gui::{ConfigFileGui, Gui},
		units::{ConfigFileUnits, Units},
	},
};

#[optional_struct(ConfigFile)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
	pub address: String,
	pub language: String,
	pub forecast: Vec<Forecast>,
	#[optional_rename(ConfigFileUnits)]
	pub units: Units,
	#[optional_rename(ConfigFileGui)]
	pub gui: Gui,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			address: String::new(),
			forecast: vec![],
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
			let cfg_form_file: ConfigFile = match Options::default()
				.with_default_extension(Extensions::IMPLICIT_SOME)
				.from_str(&file)
			{
				Ok(contents) => contents,
				Err(error) => {
					let warning_sign = " Warning:".color(Yellow);
					eprintln!(
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

	pub fn store(&self) {
		let path = Self::get_path();

		let cfg_dir = path.parent().unwrap();
		if !cfg_dir.is_dir() {
			fs::create_dir(cfg_dir).unwrap();
		};

		let mut file = File::create(path).unwrap();
		let output = to_string_pretty(self, PrettyConfig::default()).unwrap();
		file.write_all(output.as_bytes()).unwrap();
	}

	pub fn get_path() -> PathBuf {
		ProjectDirs::from("", "", CONFIG_DIR_NAME)
			.unwrap()
			.config_dir()
			.join(CONFIG_FILE_NAME)
	}
}

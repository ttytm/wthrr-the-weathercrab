use std::collections::HashMap;

use anyhow::{Context, Result};
use chrono::{Datelike, Local, Weekday};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use optional_struct::Applyable;
use serde::{Deserialize, Serialize};

use super::{
	args::{Cli, Forecast},
	config::Config,
	localization::{ConfigLocales, Locales},
	location::Location,
	units::Units,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
	pub config: Config,
	pub forecast_indices: [bool; 9],
	pub texts: Locales,
}

impl Params {
	pub async fn merge(config: &Config, args: &Cli) -> Result<Self> {
		let language = match &args.language {
			Some(lang) => lang.to_string(),
			_ => config.language.to_owned(),
		};

		let texts = Locales::get(&language).await?;

		if args.reset {
			Self::reset(&texts.config).await?;
			std::process::exit(1);
		}

		// Declare as mutable to disable time_indicator other days then current day
		let mut gui = config.gui.to_owned();

		let units = Units::merge(&args.units, &config.units);

		let address = Location::resolve_input(args.address.as_deref().unwrap_or_default(), config, &texts).await?;

		let mut forecast = if !args.forecast.is_empty() {
			args.forecast.to_owned()
		} else {
			config.forecast.to_owned()
		};

		// Create a map of indices for forecasts that should be rendered.
		// It mainly serves as navigator in the arrays of the api response.
		// [0] = current day; [1..7] = week days; [7] = week overview ; [8] = disable
		// Until there is a more concise solution this is a working and fairly slim approach.
		let mut forecast_indices = [false; 9];
		if !forecast.is_empty() {
			// Remove duplicates
			let mut seen: HashMap<Forecast, bool> = HashMap::new();
			forecast.retain(|&x| seen.insert(x, true).is_none());

			let todays_index = Local::now().weekday().number_from_monday() as i8;

			for val in &forecast {
				match val {
					Forecast::disable => forecast_indices[8] = true,
					Forecast::day => forecast_indices[0] = true,
					Forecast::week => forecast_indices[7] = true,
					// Forecast weekdays
					Forecast::mo => {
						forecast_indices[Self::get_day_index(todays_index, Weekday::Mon.number_from_monday() as i8)] =
							true;
					}
					Forecast::tu => {
						forecast_indices[Self::get_day_index(todays_index, Weekday::Tue.number_from_monday() as i8)] =
							true;
					}
					Forecast::we => {
						forecast_indices[Self::get_day_index(todays_index, Weekday::Wed.number_from_monday() as i8)] =
							true;
					}
					Forecast::th => {
						forecast_indices[Self::get_day_index(todays_index, Weekday::Thu.number_from_monday() as i8)] =
							true;
					}
					Forecast::fr => {
						forecast_indices[Self::get_day_index(todays_index, Weekday::Fri.number_from_monday() as i8)] =
							true;
					}
					Forecast::sa => {
						forecast_indices[Self::get_day_index(todays_index, Weekday::Sat.number_from_monday() as i8)] =
							true;
					}
					Forecast::su => {
						forecast_indices[Self::get_day_index(todays_index, Weekday::Sun.number_from_monday() as i8)] =
							true;
					}
				}
				match val {
					Forecast::mo
					| Forecast::tu
					| Forecast::we
					| Forecast::th
					| Forecast::fr
					| Forecast::sa
					| Forecast::su => gui.graph.time_indicator = false,
					_ => (),
				}
			}
		};

		if forecast_indices[8] {
			forecast.clear()
		}

		Ok(Self {
			config: Config {
				language,
				forecast,
				units,
				address,
				gui,
			},
			forecast_indices,
			texts,
		})
	}

	fn get_day_index(todays_index: i8, weekday_index: i8) -> usize {
		(((weekday_index - todays_index) % 7 + 7) % 7).try_into().unwrap()
	}

	pub async fn handle_next(mut self, args: Cli, mut config_file: Config) -> Result<()> {
		if !args.save && !config_file.address.is_empty() {
			return Ok(());
		}

		// Restore time_indicator config setting in case it was disabled for a weekday / historical forecast
		self.config.gui.graph.time_indicator = config_file.gui.graph.time_indicator;

		if config_file.address.is_empty() {
			// Prompt to save
			self.config.apply_to(&mut config_file);
			self.config = config_file;
			self.save_prompt(&args.address.unwrap_or_default()).await?;
		} else {
			// Handle explicit save call
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

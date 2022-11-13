use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Forecast {
	pub week: Option<bool>,
	pub day: Option<bool>,
}

impl Default for Forecast {
	fn default() -> Self {
		Self {
			week: Some(false),
			day: Some(false),
		}
	}
}

pub fn get(args_commands: Option<Forecast>, config_forecast: Option<Forecast>) -> Result<Forecast> {
	let forecast = match args_commands {
		Some(Forecast {
			day: Some(false),
			week: Some(false),
		}) => Forecast {
			day: Some(true),
			week: Some(true),
		},
		Some(_) => args_commands.unwrap(),
		_ => match config_forecast {
			Some(Forecast {
				day: Some(_),
				week: Some(_),
			}) => config_forecast.unwrap(),
			Some(Forecast {
				day: Some(_),
				week: None,
			}) => Forecast {
				day: config_forecast.unwrap().day,
				week: Config::default().forecast.unwrap().week,
			},
			Some(Forecast {
				day: None,
				week: Some(_),
			}) => Forecast {
				day: Config::default().forecast.unwrap().day,
				week: config_forecast.unwrap().week,
			},
			_ => config_forecast.unwrap_or_else(|| Config::default().forecast.unwrap()),
		},
	};

	Ok(forecast)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn forecast_from_args() -> Result<()> {
		let forecast_args = Some(Forecast {
			day: Some(true),
			week: Some(true),
		});
		let forecast_cfg = Some(Forecast {
			day: Some(false),
			week: Some(false),
		});

		assert_eq!(
			get(forecast_args, forecast_cfg)?,
			Forecast {
				day: Some(true),
				week: Some(true)
			}
		);

		Ok(())
	}

	#[test]
	fn forecast_from_cfg() -> Result<()> {
		let forecast_args = None;
		let forecast_cfg = Some(Forecast {
			day: Some(true),
			week: Some(true),
		});

		assert_eq!(
			get(forecast_args, forecast_cfg)?,
			Forecast {
				day: Some(true),
				week: Some(true)
			}
		);

		Ok(())
	}

	#[test]
	fn forecast_from_cfg_partial() -> Result<()> {
		let forecast_args = None;
		let forecast_cfg = Some(Forecast {
			day: Some(true),
			week: None,
		});

		assert_eq!(
			get(forecast_args, forecast_cfg)?,
			Forecast {
				day: Some(true),
				week: Config::default().forecast.unwrap().week,
			}
		);

		Ok(())
	}
}

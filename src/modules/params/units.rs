use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::VariantNames;
use strum_macros::{AsRefStr, EnumString, EnumVariantNames};

use crate::args::ArgUnits;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Units {
	pub temperature: Option<Temperature>,
	pub speed: Option<Speed>,
	pub time: Option<TimeFormat>,
}

impl Default for Units {
	fn default() -> Self {
		Self {
			temperature: Some(Temperature::celsius),
			speed: Some(Speed::kmh),
			time: Some(TimeFormat::military),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, AsRefStr, EnumVariantNames, EnumString)]
#[allow(non_camel_case_types)]
pub enum Temperature {
	celsius,
	fahrenheit,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, AsRefStr, EnumVariantNames, EnumString)]
#[allow(non_camel_case_types)]
pub enum Speed {
	kmh,
	mph,
	knots,
	ms,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, AsRefStr, EnumVariantNames, EnumString)]
#[allow(non_camel_case_types)]
pub enum TimeFormat {
	am_pm,
	military,
}

pub fn get(arg_units: &[ArgUnits], config_units: &Units) -> Result<Units> {
	let mut units = assign_arg_units(arg_units)?;

	// TODO: do those assignments programmatically
	if units.temperature.is_none() && config_units.temperature.is_some() {
		units.temperature = config_units.temperature
	} else if units.temperature.is_none() && config_units.temperature.is_none() {
		units.temperature = Units::default().temperature
	}

	if units.speed.is_none() && config_units.speed.is_some() {
		units.speed = config_units.speed
	} else if units.speed.is_none() && config_units.speed.is_none() {
		units.speed = Units::default().speed
	}

	if units.time.is_none() && config_units.time.is_some() {
		units.time = config_units.time
	} else if units.time.is_none() && config_units.time.is_none() {
		units.time = Units::default().time
	}

	Ok(units)
}

pub fn assign_arg_units(arg_units: &[ArgUnits]) -> Result<Units> {
	let mut units = Units {
		temperature: None,
		speed: None,
		time: None,
	};

	for val in arg_units {
		if Temperature::VARIANTS.as_ref().contains(&val.as_ref()) {
			units.temperature = Some(Temperature::from_str(val.as_ref()).unwrap())
		}
		if Speed::VARIANTS.as_ref().contains(&val.as_ref()) {
			units.speed = Some(Speed::from_str(val.as_ref()).unwrap())
		}
		if TimeFormat::VARIANTS.as_ref().contains(&val.as_ref()) {
			units.time = Some(TimeFormat::from_str(val.as_ref()).unwrap())
		}
	}

	Ok(units)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn units_from_args() -> Result<()> {
		let arg_units = [ArgUnits::Fahrenheit, ArgUnits::Mph, ArgUnits::AmPm];
		let cfg_units = Units {
			temperature: Some(Temperature::celsius),
			speed: Some(Speed::kmh),
			time: Some(TimeFormat::military),
		};

		assert_eq!(
			get(&arg_units, &cfg_units)?,
			Units {
				temperature: Some(Temperature::fahrenheit),
				speed: Some(Speed::mph),
				time: Some(TimeFormat::am_pm),
			}
		);

		Ok(())
	}

	#[test]
	fn units_from_cfg() -> Result<()> {
		let arg_units = [];
		let cfg_units = Units {
			temperature: Some(Temperature::fahrenheit),
			speed: Some(Speed::knots),
			time: Some(TimeFormat::am_pm),
		};

		assert_eq!(get(&arg_units, &cfg_units)?, cfg_units);

		Ok(())
	}

	#[test]
	fn units_split_from_args_cfg() -> Result<()> {
		let arg_units = [ArgUnits::Fahrenheit, ArgUnits::AmPm];
		let cfg_units = Units {
			temperature: Some(Temperature::celsius),
			speed: Some(Speed::ms),
			time: None,
		};

		assert_eq!(
			get(&arg_units, &cfg_units)?,
			Units {
				temperature: Some(Temperature::fahrenheit),
				speed: cfg_units.speed,
				time: Some(TimeFormat::am_pm),
			}
		);

		Ok(())
	}

	#[test]
	fn units_fallback() -> Result<()> {
		let arg_units = [];
		let cfg_units = Units::default();

		assert_eq!(get(&arg_units, &cfg_units)?, Units::default());

		Ok(())
	}
}

use anyhow::Result;
use std::str::FromStr;
use strum::VariantNames;
use strum_macros::{AsRefStr, EnumString, EnumVariantNames};

use crate::args::ArgUnits;

#[derive(Debug, PartialEq)]
pub struct Units {
	pub temperature: TempUnit,
	pub speed: SpeedUnit,
}

#[derive(Default, Debug, PartialEq, AsRefStr, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum TempUnit {
	None,
	#[default]
	Celsius,
	Fahrenheit,
}

#[derive(Default, Debug, PartialEq, AsRefStr, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum SpeedUnit {
	None,
	#[default]
	Kmh,
	Mph,
	Kn,
	Ms,
}

pub fn get(arg_units: &[ArgUnits], config_units: &str) -> Result<Units> {
	let arg_units = assign_arg_units(arg_units)?;
	let temperature = arg_units.temperature.get(config_units)?;
	let speed = arg_units.speed.get(config_units)?;

	Ok(Units { temperature, speed })
}

pub fn assign_arg_units(arg_units: &[ArgUnits]) -> Result<Units> {
	let (mut temperature, mut speed) = (TempUnit::None, SpeedUnit::None);

	for val in arg_units {
		if TempUnit::VARIANTS.contains(&val.as_ref()) {
			temperature = TempUnit::from_str(val.as_ref()).unwrap()
		};
		if SpeedUnit::VARIANTS.contains(&val.as_ref()) {
			speed = SpeedUnit::from_str(val.as_ref()).unwrap()
		}
	}

	Ok(Units { temperature, speed })
}

impl TempUnit {
	fn get(self, config_units: &str) -> Result<Self> {
		let res = if self == Self::None && !config_units.is_empty() {
			match config_units {
				unit if unit.contains(Self::Fahrenheit.as_ref()) => Self::Fahrenheit,
				unit if unit.contains(Self::Celsius.as_ref()) => Self::Celsius,
				_ => Self::default(),
			}
		} else {
			match self {
				Self::None => Self::default(),
				_ => self,
			}
		};

		Ok(res)
	}
}

impl SpeedUnit {
	fn get(self, config_units: &str) -> Result<Self> {
		let res = if self == Self::None && !config_units.is_empty() {
			match config_units {
				unit if unit.contains(Self::Kmh.as_ref()) => Self::Kmh,
				unit if unit.contains(Self::Mph.as_ref()) => Self::Mph,
				unit if unit.contains(Self::Kn.as_ref()) => Self::Kn,
				unit if unit.contains(Self::Ms.as_ref()) => Self::Ms,
				_ => Self::default(),
			}
		} else {
			match self {
				Self::None => Self::default(),
				_ => self,
			}
		};

		Ok(res)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn units_from_args() -> Result<()> {
		let arg_units = [ArgUnits::fahrenheit, ArgUnits::mph].to_vec();
		let cfg_units = "celsius,knots";

		assert_eq!(
			get(&arg_units, cfg_units)?,
			Units {
				temperature: TempUnit::Fahrenheit,
				speed: SpeedUnit::Mph,
			}
		);

		Ok(())
	}

	#[test]
	fn units_from_cfg() -> Result<()> {
		let arg_units: Vec<ArgUnits> = [].to_vec();
		let cfg_units = "fahrenheit,knots";

		assert_eq!(
			get(&arg_units, cfg_units)?,
			Units {
				temperature: TempUnit::Fahrenheit,
				speed: SpeedUnit::Kn,
			}
		);

		Ok(())
	}

	#[test]
	fn units_split_from_args_cfg() -> Result<()> {
		let arg_units = [ArgUnits::fahrenheit].to_vec();
		let cfg_units = "celsius,ms";

		assert_eq!(
			get(&arg_units, cfg_units)?,
			Units {
				temperature: TempUnit::Fahrenheit,
				speed: SpeedUnit::Ms,
			}
		);

		Ok(())
	}

	#[test]
	fn units_fallback() -> Result<()> {
		let arg_units = [].to_vec();
		let cfg_units = "non_variant";

		assert_eq!(
			get(&arg_units, cfg_units)?,
			Units {
				temperature: TempUnit::default(),
				speed: SpeedUnit::default(),
			}
		);

		Ok(())
	}
}

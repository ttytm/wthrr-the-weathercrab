use anyhow::Result;

use crate::args::ArgUnits;

#[derive(Debug, PartialEq)]
pub struct Units {
	pub temperature: ArgUnits,
	pub speed: ArgUnits,
}

impl Default for Units {
	fn default() -> Self {
		Self {
			temperature: ArgUnits::Celsius,
			speed: ArgUnits::Kmh,
		}
	}
}

pub fn get(arg_units: &[ArgUnits], config_units: &str) -> Result<Units> {
	let mut units = assign_arg_units(arg_units)?;

	if units.temperature == ArgUnits::None {
		match config_units {
			unit if unit.contains(ArgUnits::Fahrenheit.as_ref()) => units.temperature = ArgUnits::Fahrenheit,
			unit if unit.contains(ArgUnits::Celsius.as_ref()) => units.temperature = ArgUnits::Celsius,
			_ => units.temperature = Units::default().temperature,
		}
	}
	if units.speed == ArgUnits::None {
		match config_units {
			unit if unit.contains(ArgUnits::Kmh.as_ref()) => units.speed = ArgUnits::Kmh,
			unit if unit.contains(ArgUnits::Mph.as_ref()) => units.speed = ArgUnits::Mph,
			unit if unit.contains(ArgUnits::Knots.as_ref()) => units.speed = ArgUnits::Knots,
			unit if unit.contains(ArgUnits::Ms.as_ref()) => units.speed = ArgUnits::Ms,
			_ => units.speed = Units::default().speed,
		}
	}

	Ok(units)
}

pub fn assign_arg_units(arg_units: &[ArgUnits]) -> Result<Units> {
	let mut units = Units {
		temperature: ArgUnits::None,
		speed: ArgUnits::None,
	};

	for val in arg_units {
		if let ArgUnits::Celsius | ArgUnits::Fahrenheit = val {
			units.temperature = *val
		} else {
			units.speed = *val
		}
	}

	Ok(units)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn units_from_args() -> Result<()> {
		let arg_units = [ArgUnits::Fahrenheit, ArgUnits::Mph].to_vec();
		let cfg_units = "celsius,knots";

		assert_eq!(
			get(&arg_units, cfg_units)?,
			Units {
				temperature: ArgUnits::Fahrenheit,
				speed: ArgUnits::Mph,
			}
		);

		Ok(())
	}

	#[test]
	fn units_from_cfg() -> Result<()> {
		let arg_units: Vec<ArgUnits> = [].to_vec();
		let cfg_units = "fahrenheit,kn";

		assert_eq!(
			get(&arg_units, cfg_units)?,
			Units {
				temperature: ArgUnits::Fahrenheit,
				speed: ArgUnits::Knots,
			}
		);

		Ok(())
	}

	#[test]
	fn units_split_from_args_cfg() -> Result<()> {
		let arg_units = [ArgUnits::Fahrenheit].to_vec();
		let cfg_units = "celsius,ms";

		assert_eq!(
			get(&arg_units, cfg_units)?,
			Units {
				temperature: ArgUnits::Fahrenheit,
				speed: ArgUnits::Ms,
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
				temperature: Units::default().temperature,
				speed: Units::default().speed,
			}
		);

		Ok(())
	}
}

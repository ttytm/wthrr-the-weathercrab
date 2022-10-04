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
			temperature: ArgUnits::celsius,
			speed: ArgUnits::kmh,
		}
	}
}

pub fn get(arg_units: &[ArgUnits], config_units: &str) -> Result<Units> {
	let mut units = assign_arg_units(arg_units)?;

	if units.temperature == ArgUnits::None {
		match config_units {
			unit if unit.contains(ArgUnits::fahrenheit.as_ref()) => units.temperature = ArgUnits::fahrenheit,
			unit if unit.contains(ArgUnits::celsius.as_ref()) => units.temperature = ArgUnits::celsius,
			_ => units.temperature = Units::default().temperature,
		}
	}
	if units.speed == ArgUnits::None {
		match config_units {
			unit if unit.contains(ArgUnits::kmh.as_ref()) => units.speed = ArgUnits::kmh,
			unit if unit.contains(ArgUnits::mph.as_ref()) => units.speed = ArgUnits::mph,
			unit if unit.contains(ArgUnits::kn.as_ref()) => units.speed = ArgUnits::kn,
			unit if unit.contains(ArgUnits::ms.as_ref()) => units.speed = ArgUnits::ms,
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
		if let ArgUnits::celsius | ArgUnits::fahrenheit = val {
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
		let arg_units = [ArgUnits::fahrenheit, ArgUnits::mph].to_vec();
		let cfg_units = "celsius,knots";

		assert_eq!(
			get(&arg_units, cfg_units)?,
			Units {
				temperature: ArgUnits::fahrenheit,
				speed: ArgUnits::mph,
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
				temperature: ArgUnits::fahrenheit,
				speed: ArgUnits::kn,
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
				temperature: ArgUnits::fahrenheit,
				speed: ArgUnits::ms,
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

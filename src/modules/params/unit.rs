use anyhow::Result;

use crate::config::TempUnit;

pub fn get(args_unit: &str, config_unit: Option<&TempUnit>) -> Result<TempUnit> {
	let unit = if args_unit.is_empty() && config_unit.is_some() {
		match config_unit {
			unit if unit == Some(&TempUnit::Fahrenheit) => TempUnit::Fahrenheit,
			_ => TempUnit::Celsius,
		}
	} else if args_unit == "f" || args_unit == "fahrenheit" {
		TempUnit::Fahrenheit
	} else {
		TempUnit::Celsius
	};

	Ok(unit)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn temp_unit_from_arg() -> Result<()> {
		let arg_unit = "f";
		let cfg_unit = TempUnit::Celsius;

		assert_eq!(get(arg_unit, Some(&cfg_unit))?, TempUnit::Fahrenheit);

		Ok(())
	}

	#[test]
	fn temp_unit_from_cfg() -> Result<()> {
		let arg_unit = "";
		let cfg_unit = TempUnit::Fahrenheit;

		assert_eq!(get(arg_unit, Some(&cfg_unit))?, TempUnit::Fahrenheit);

		Ok(())
	}

	#[test]
	fn temp_unit_fallback() -> Result<()> {
		let arg_unit = "a";

		assert_eq!(get(arg_unit, None)?, TempUnit::Celsius);

		Ok(())
	}
}

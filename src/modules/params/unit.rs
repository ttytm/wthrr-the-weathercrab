use anyhow::Result;

use crate::config::TempUnit;

pub fn get(args_unit: &str, config_unit: &str) -> Result<TempUnit> {
	let unit = if args_unit.is_empty() && !config_unit.is_empty() {
		match config_unit {
			unit if unit == TempUnit::Fahrenheit.as_ref() => TempUnit::Fahrenheit,
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
		let cfg_unit = "celsius";

		assert_eq!(get(arg_unit, cfg_unit)?, TempUnit::Fahrenheit);

		Ok(())
	}

	#[test]
	fn temp_unit_from_cfg() -> Result<()> {
		let arg_unit = "";
		let cfg_unit = "fahrenheit";

		assert_eq!(get(arg_unit, cfg_unit)?, TempUnit::Fahrenheit);

		Ok(())
	}

	#[test]
	fn temp_unit_fallback() -> Result<()> {
		let arg_unit = "a";
		let cfg_unit = "";

		assert_eq!(get(arg_unit, cfg_unit)?, TempUnit::Celsius);

		Ok(())
	}
}

use optional_struct::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::VariantNames;
use strum_macros::{AsRefStr, EnumString, EnumVariantNames};

use crate::modules::args::UnitArg;

#[optional_struct(ConfigFileUnits)]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Units {
	pub temperature: Temperature,
	pub speed: Speed,
	pub time: Time,
	pub precipitation: Precipitation,
}

#[derive(
	Default, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, AsRefStr, EnumVariantNames, EnumString,
)]
#[allow(non_camel_case_types)]
pub enum Temperature {
	#[default]
	celsius,
	fahrenheit,
}

#[derive(
	Default, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, AsRefStr, EnumVariantNames, EnumString,
)]
#[allow(non_camel_case_types)]
pub enum Speed {
	#[default]
	kmh,
	mph,
	knots,
	ms,
}

#[derive(
	Default, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, AsRefStr, EnumVariantNames, EnumString,
)]
#[allow(non_camel_case_types)]
pub enum Time {
	am_pm,
	#[default]
	military,
}

#[derive(
	Default, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, AsRefStr, EnumVariantNames, EnumString,
)]
#[allow(non_camel_case_types)]
pub enum Precipitation {
	#[default]
	probability,
	mm,
	inch,
}

impl Units {
	pub fn get(arg_units: &[UnitArg], cfg_units: &Units) -> Units {
		cfg_units.assign_unit_args(arg_units)
	}

	pub fn assign_unit_args(mut self, arg_units: &[UnitArg]) -> Units {
		for val in arg_units {
			if Temperature::VARIANTS.as_ref().contains(&val.as_ref()) {
				self.temperature = Temperature::from_str(val.as_ref()).unwrap()
			}
			if Speed::VARIANTS.as_ref().contains(&val.as_ref()) {
				self.speed = Speed::from_str(val.as_ref()).unwrap()
			}
			if Time::VARIANTS.as_ref().contains(&val.as_ref()) {
				self.time = Time::from_str(val.as_ref()).unwrap()
			}
			if Precipitation::VARIANTS.as_ref().contains(&val.as_ref()) {
				self.precipitation = Precipitation::from_str(val.as_ref()).unwrap()
			}
		}

		self
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn units_from_args() {
		let arg_units = [UnitArg::Fahrenheit, UnitArg::Mph, UnitArg::AmPm, UnitArg::Inch];
		let cfg_units = Units::default();

		assert_eq!(
			Units::get(&arg_units, &cfg_units),
			Units {
				temperature: Temperature::fahrenheit,
				speed: Speed::mph,
				time: Time::am_pm,
				precipitation: Precipitation::inch,
			}
		);
	}

	#[test]
	fn units_from_cfg() {
		let arg_units = [];
		let cfg_units = Units {
			temperature: Temperature::fahrenheit,
			speed: Speed::knots,
			time: Time::am_pm,
			precipitation: Precipitation::inch,
		};

		assert_eq!(Units::get(&arg_units, &cfg_units), cfg_units);
	}

	#[test]
	fn units_split_from_args_and_cfg() {
		let arg_units = [UnitArg::Fahrenheit, UnitArg::AmPm];
		let cfg_units = Units {
			temperature: Temperature::celsius,
			speed: Speed::ms,
			time: Time::military,
			precipitation: Precipitation::inch,
		};

		assert_eq!(
			Units::get(&arg_units, &cfg_units),
			Units {
				temperature: Temperature::fahrenheit,
				speed: cfg_units.speed,
				time: Time::am_pm,
				precipitation: cfg_units.precipitation,
			}
		);
	}
}

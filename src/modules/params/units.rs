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
	mm,
	inch,
}

impl Units {
	pub fn get(unit_args: &[UnitArg], unit_cfg: &Units) -> Units {
		unit_cfg.assign_unit_args(unit_args)
	}

	pub fn assign_unit_args(mut self, unit_args: &[UnitArg]) -> Units {
		for val in unit_args {
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
		let unit_args = [UnitArg::Fahrenheit, UnitArg::Mph, UnitArg::AmPm, UnitArg::Inch];
		let unit_cfg = Units::default();

		assert_eq!(
			Units::get(&unit_args, &unit_cfg),
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
		let unit_args = [];
		let unit_cfg = Units {
			temperature: Temperature::fahrenheit,
			speed: Speed::knots,
			time: Time::am_pm,
			precipitation: Precipitation::inch,
		};

		assert_eq!(Units::get(&unit_args, &unit_cfg), unit_cfg);
	}

	#[test]
	fn units_split_from_args_and_cfg() {
		let unit_args = [UnitArg::Fahrenheit, UnitArg::AmPm];
		let unit_cfg = Units {
			temperature: Temperature::celsius,
			speed: Speed::ms,
			time: Time::military,
			precipitation: Precipitation::inch,
		};

		assert_eq!(
			Units::get(&unit_args, &unit_cfg),
			Units {
				temperature: Temperature::fahrenheit,
				speed: unit_cfg.speed,
				time: Time::am_pm,
				precipitation: unit_cfg.precipitation,
			}
		);
	}
}

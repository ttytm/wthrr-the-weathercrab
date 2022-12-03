use anyhow::Result;

use crate::modules::args::Forecast;

pub fn get(forecast_args: &[Forecast], forecast_cfg: Option<Vec<Forecast>>) -> Result<Vec<Forecast>> {
	let forecast = if !forecast_args.is_empty() {
		forecast_args.to_vec()
	} else {
		forecast_cfg.unwrap_or_default()
	};

	Ok(forecast)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn forecast_from_args() -> Result<()> {
		let forecast_args = &[Forecast::day, Forecast::week];
		let forecast_cfg = None;

		assert_eq!(get(forecast_args, forecast_cfg)?, [Forecast::day, Forecast::week]);

		Ok(())
	}

	#[test]
	fn forecast_from_cfg() -> Result<()> {
		let forecast_args = &[];
		let forecast_cfg = Some([Forecast::day, Forecast::week].to_vec());

		assert_eq!(get(forecast_args, forecast_cfg)?, [Forecast::day, Forecast::week]);

		Ok(())
	}

	#[test]
	fn forecast_from_cfg_partial() -> Result<()> {
		let forecast_args = &[];
		let forecast_cfg = Some([Forecast::day].to_vec());

		assert_eq!(get(forecast_args, forecast_cfg)?, [Forecast::day]);

		Ok(())
	}
}

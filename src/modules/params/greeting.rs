use super::Config;
use anyhow::Result;

pub fn get(args_toggle_greeting: bool, config_greet: Option<bool>) -> Result<bool> {
	// the default is true, so unwrapping it cannot panic
	let should_toggle = config_greet.unwrap_or_else(|| Config::default().greeting.unwrap());
	let greet = match args_toggle_greeting {
		true => !should_toggle,
		_ => should_toggle,
	};
	Ok(greet)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn handle_greeting_state_from_arg() -> Result<()> {
		// Toggle flag while greeting enabled in cfg
		assert_eq!(get(true, Some(true))?, false);
		// Toggle flag while greeting disabled in cfg
		assert_eq!(get(true, Some(false))?, true);

		Ok(())
	}

	#[test]
	fn handle_greeting_state_from_cfg() -> Result<()> {
		assert_eq!(get(false, Some(true))?, true);
		assert_eq!(get(false, Some(false))?, false);

		Ok(())
	}
}

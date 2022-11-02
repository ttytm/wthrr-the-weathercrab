use anyhow::Result;

pub fn get(args_toggle_greeting: bool, config_greet: Option<bool>) -> Result<bool> {
	if let Some(config_greet_value) = config_greet {
		let greet = match args_toggle_greeting {
			true => !config_greet_value,
			_ => config_greet_value,
		};
		Ok(greet)
	} else {
		Ok(!args_toggle_greeting)
	}
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

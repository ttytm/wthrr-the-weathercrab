use anyhow::Result;

pub fn get(args_lang: &str, config_lang: &str) -> Result<String> {
	let lang = if !args_lang.is_empty() {
		args_lang
	} else if args_lang.is_empty() && !config_lang.is_empty() {
		config_lang
	} else {
		"en"
	};

	Ok(lang.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	use crate::modules::config::Config;

	#[test]
	fn lang_from_arg() -> Result<()> {
		let arg_lang = "pl";
		let config = Config {
			language: Some("de".to_string()),
			..Default::default()
		};

		assert!(get(arg_lang, config.language.as_deref().unwrap_or_default())?.contains("pl"));

		Ok(())
	}

	#[test]
	fn lang_from_cfg() -> Result<()> {
		let arg_lang = "";
		let config = Config {
			language: Some("de".to_string()),
			..Default::default()
		};

		assert!(get(arg_lang, config.language.as_deref().unwrap_or_default())?.contains("de"));

		Ok(())
	}
}

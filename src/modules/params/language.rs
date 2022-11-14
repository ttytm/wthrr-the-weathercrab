use anyhow::Result;

use crate::config::Config;

pub fn get(lang_arg: &str, lang_cfg: &str) -> Result<String> {
	let lang = if !lang_arg.is_empty() {
		lang_arg.to_string()
	} else if lang_arg.is_empty() && !lang_cfg.is_empty() {
		lang_cfg.to_string()
	} else {
		Config::default().language.unwrap()
	};

	Ok(lang)
}

#[cfg(test)]
mod tests {
	use super::*;

	use crate::modules::config::Config;

	#[test]
	fn lang_from_arg() -> Result<()> {
		let lang_arg = "pl";
		let config = Config {
			language: Some("de".to_string()),
			..Default::default()
		};

		assert!(get(lang_arg, config.language.as_deref().unwrap_or_default())?.contains("pl"));

		Ok(())
	}

	#[test]
	fn lang_from_cfg() -> Result<()> {
		let lang_arg = "";
		let config = Config {
			language: Some("de".to_string()),
			..Default::default()
		};

		assert!(get(lang_arg, config.language.as_deref().unwrap_or_default())?.contains("de"));

		Ok(())
	}
}

use anyhow::{Context, Result};
use directories::ProjectDirs;
use optional_struct::Applyable;
use reqwest::Url;
use serde_json::Value;
use std::{
	fs::{self, File},
	io::Write,
	path::PathBuf,
};

use super::{Locales, LocalesFile};

impl Locales {
	pub async fn get(lang: &str) -> Result<Self> {
		let texts = Locales::get_translations_file(lang).await?;

		Ok(texts)
	}

	async fn translate_all(&mut self, lang: &str) -> Result<()> {
		let size = std::mem::size_of_val(self);
		let ptr = self as *mut Self as *mut u8;

		for offset in (0..size).step_by(std::mem::size_of::<String>()) {
			let field_ptr = unsafe { (ptr.add(offset)) as *mut String };
			let field_value = unsafe { &*field_ptr };
			let new_value = Self::translate_str(lang, field_value).await?;
			unsafe { *field_ptr = new_value };
		}

		Ok(())
	}

	async fn translate_str(lang: &str, input: &str) -> Result<String> {
		let url = Url::parse_with_params(
			"https://translate.googleapis.com/translate_a/single?client=gtx&ie=UTF-8&oe=UTF-8&dt=t&sl=en_US",
			&[("tl", lang), ("q", input)],
		)?;

		let res = reqwest::get(url)
			.await?
			.json::<Vec<Value>>()
			.await
			.with_context(|| "Translation request failed.")?;

		let output = match res.first() {
			Some(i) => i
				.as_array()
				.unwrap()
				.iter()
				.map(|s| s[0].as_str().unwrap())
				.collect::<Vec<&str>>()
				.join(""),
			_ => String::new(),
		};

		Ok(output)
	}

	async fn get_translations_file(lang: &str) -> Result<Locales> {
		let mut texts = Locales::default();
		let path = Self::get_path(lang);

		if let Ok(file) = fs::read_to_string(path) {
			let locales_from_file: LocalesFile = match serde_json::from_str(&file) {
				Ok(contents) => contents,
				Err(_) => {
					if lang == "en_US" || lang == "en" {
						return Ok(texts);
					}
					texts.translate_all(lang).await?;
					return Ok(texts);
				}
			};

			locales_from_file.apply_to(&mut texts);
		};

		Ok(texts)
	}

	pub fn store(&self, lang: &str) {
		let path = Self::get_path(lang);
		let dir = path.parent().unwrap();
		if !dir.is_dir() {
			fs::create_dir(dir).unwrap();
		};

		let mut file = File::create(path).unwrap();
		file.write_all(serde_json::to_string_pretty(self).unwrap().as_bytes())
			.unwrap();
	}

	pub fn get_path(lang: &str) -> PathBuf {
		ProjectDirs::from("", "", crate::modules::config::CONFIG_DIR_NAME)
			.unwrap()
			.config_dir()
			.join("locales")
			.join(format!("{lang}.json"))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn translate_string() -> Result<()> {
		let (target_lang, input) = ("de_DE", "tongue-twister");

		let res = Locales::translate_str(target_lang, input).await?;

		assert!(res.contains("Zungenbrecher"));

		Ok(())
	}
}

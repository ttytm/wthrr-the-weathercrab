use anyhow::{Context, Result};
use chrono::prelude::*;
use directories::ProjectDirs;
use futures::{stream::FuturesOrdered, StreamExt};
use optional_struct::Applyable;
use reqwest::Url;
use serde_json::Value;
use std::{
	fs::{self, File},
	io::Write,
	path::PathBuf,
};

use super::{Locales, LocalesFile};

const DATETIME_LOCALES: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/locales/pure-rust-locales.txt"));

impl Locales {
	pub async fn get(lang: &str) -> Result<Self> {
		let mut texts = Locales::default();
		let path = Self::get_path(lang);

		match fs::read_to_string(path) {
			Ok(file) => {
				match serde_json::from_str::<LocalesFile>(&file) {
					Ok(contents) => contents.apply_to(&mut texts),
					Err(_) => {
						if !(lang == "en_US" || lang == "en") {
							texts.translate_all(lang).await?;
						}
						return Ok(texts);
					}
				};
			}
			Err(_) => {
				if !(lang == "en_US" || lang == "en") {
					texts.translate_all(lang).await?;
				}
			}
		};

		Ok(texts)
	}

	async fn translate_all(&mut self, lang: &str) -> Result<()> {
		let size = std::mem::size_of_val(self);
		let ptr = self as *mut Self as *mut u8;

		let mut translated_values = Vec::new();
		let mut futures = FuturesOrdered::new();

		// Iterate over each field in the struct, create a future to translate the current field's value
		for offset in (0..size).step_by(std::mem::size_of::<String>()) {
			let field_ptr = unsafe { (ptr.add(offset)) as *mut String };
			let field_value = unsafe { &*field_ptr };
			let future = Self::translate_str(lang, field_value);
			futures.push_back(future);
		}

		// Wait for each future in the stream to complete and store the translated values in a vector
		while let Some(result) = futures.next().await {
			let translated_value = result?;
			translated_values.push(translated_value);
		}

		// Iterate over each field in the struct again, update current field value with the translated value
		for (offset, translated_value) in (0..size)
			.step_by(std::mem::size_of::<String>())
			.zip(translated_values.into_iter())
		{
			let field_ptr = unsafe { (ptr.add(offset)) as *mut String };
			unsafe { *field_ptr = translated_value };
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

	pub fn localize_date(dt: DateTime<Utc>, lang: &str) -> Result<String> {
		let mut matching_locale: Option<&str> = None;

		for line in DATETIME_LOCALES.lines().skip(1) {
			if line == lang {
				matching_locale = Some(line);
				break;
			}
		}

		if matching_locale.is_none() {
			for line in DATETIME_LOCALES.lines().skip(1) {
				let short_lang_code: Vec<&str> = line.split('_').collect();

				if short_lang_code[0] == lang {
					matching_locale = Some(line);
					break;
				}
			}
		}

		let date = if let Some(locale) = matching_locale {
			dt.format_localized("%a, %e %b", locale.try_into().unwrap()).to_string()
		} else {
			dt.format("%a, %e %b").to_string()
		};

		Ok(date)
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

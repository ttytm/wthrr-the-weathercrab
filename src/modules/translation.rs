use anyhow::{Context, Result};
use reqwest::Url;
use serde_json::Value;

pub async fn translate(target_lang: &str, input: &str) -> Result<String> {
	if target_lang == "en" {
		return Ok(input.to_string());
	}

	let url = Url::parse_with_params(
		"https://translate.googleapis.com/translate_a/single?client=gtx&ie=UTF-8&oe=UTF-8&dt=t&sl=en",
		&[("tl", target_lang), ("q", input)],
	)?;

	let res = reqwest::get(url)
		.await?
		.json::<Vec<Value>>()
		.await
		.with_context(|| "Translation request failed.")?;

	let output;
	match res.first() {
		Some(i) => {
			let result = i
				.as_array()
				.unwrap()
				.iter()
				.map(|s| s[0].as_str().unwrap())
				.collect::<Vec<&str>>()
				.join("");

			output = result;
		}
		_ => {
			output = String::new();
		}
	}

	Ok(output)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_translation() -> Result<()> {
		let (target_lang, input) = ("de", "tounge-twister");

		let res = translate(target_lang, input).await?;

		assert!(res.contains("Zungenbrecher"));

		Ok(())
	}
}

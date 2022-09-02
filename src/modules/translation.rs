use anyhow::{Context, Result};
use serde_json::Value;

pub async fn translate(target_lang: &str, input: &str) -> Result<String> {
	let url: String = format!(
		"https://translate.googleapis.com/translate_a/single?client=gtx&ie=UTF-8&oe=UTF-8&dt=t&sl=en&tl={}&q={}",
		target_lang, input
	);

	let res = reqwest::get(url)
		.await?
		.json::<Vec<Value>>()
		.await
		.with_context(|| "Translation request failed.")?;

	let translated_str;
	match res.first() {
		Some(i) => {
			let result = i
				.as_array()
				.unwrap()
				.iter()
				.map(|s| s[0].as_str().unwrap())
				.collect::<Vec<&str>>()
				.join("");

			translated_str = result;
		}
		_ => {
			translated_str = "".to_string();
			eprintln!("{}", ("Error..."))
		}
	}

	Ok(translated_str)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_translation() -> Result<()> {
		let (target_lang, input) = ("de", "tounge-twister");

		let res = translate(target_lang, input).await?;
		println!("res {}", res);

		assert!(res.contains("Zungenbrecher"));

		Ok(())
	}
}

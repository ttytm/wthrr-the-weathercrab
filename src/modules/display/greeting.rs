use anyhow::Result;

use crate::modules::translation::translate;

pub async fn handle_greeting(greet: bool, lang: &str, indent: bool) -> Result<()> {
	if !greet {
		return Ok(());
	}

	let greeting = translate(lang, "Hey friend. I'm glad you are asking.").await?;

	println!("{} 🦀  {greeting}", if indent { " " } else { "" });

	Ok(())
}

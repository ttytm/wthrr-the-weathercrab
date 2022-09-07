use anyhow::Result;

use crate::{args::Args, config::Config};

mod address;
mod greeting;
mod language;
mod unit;

pub async fn get(args: &Args, config: &Config) -> Result<Config> {
	let lang = language::get(
		args.language.as_deref().unwrap_or_default(),
		config.language.as_deref().unwrap_or_default(),
	)?;

	if args.reset_config {
		Config::reset(&lang).await?;
		std::process::exit(1);
	}

	let address = address::get(
		args.address.as_deref().unwrap_or_default(),
		config.address.as_deref().unwrap_or_default(),
		&lang,
	)
	.await?;

	let unit = unit::get(args.unit.as_deref().unwrap_or_default(), config.unit.as_ref().unwrap())?;

	let greeting = greeting::get(args.greeting, config.greeting)?;

	Ok(Config {
		address: Some(address),
		unit: Some(unit.as_ref().to_string()),
		language: Some(lang.to_string()),
		greeting: Some(greeting),
		..Config::clone(config)
	})
}

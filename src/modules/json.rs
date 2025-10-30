use super::{location::Location, units::Units, weather::Weather};

use anyhow::Result;
use serde_json;
use std::fs;

pub async fn to_json(address: &str, language: &str, units: &Units) -> Result<()> {
	let loc = Location::get(address, language).await?;
	let weather = Weather::get(loc.lat, loc.lon, units).await?;

	let data = serde_json::to_string(&weather)?;
	writing(data);

	Ok(())
}

fn writing(data: String) {
	fs::write("test.json", data).expect("failed to write to test.json");
}

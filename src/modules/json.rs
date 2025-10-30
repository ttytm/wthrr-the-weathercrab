use super::{config::CONFIG_DIR_NAME, location::Location, units::Units, weather::Weather};

use anyhow::Result;
use serde_json;
use std::{fs, io::Write, path::PathBuf};

const JSON_FILE_NAME: &str = "wthrr.json";

pub async fn to_json(address: &str, language: &str, units: &Units) -> Result<()> {
	let path = get_path();

	let cfg_dir = path.parent().unwrap();
	if !cfg_dir.is_dir() {
		fs::create_dir_all(cfg_dir)?;
	}

	let mut file = fs::File::create(path)?;
	let loc = Location::get(address, language).await?;
	let weather = Weather::get(loc.lat, loc.lon, units).await?;

	let data = serde_json::to_string(&weather)?;

	file.write_all(data.as_bytes())?;

	Ok(())
}

fn get_path() -> PathBuf {
	dirs::config_dir().unwrap().join(CONFIG_DIR_NAME).join(JSON_FILE_NAME)
}

use anyhow::{anyhow, Result};
use strum_macros::Display;

#[derive(Display)]
pub enum WindDirection {
	NW,
	N,
	NE,
	E,
	SE,
	S,
	SW,
	W,
}

impl WindDirection {
	pub fn get_direction(wd: f32) -> Result<Self> {
		let direction = match wd % 360.0 {
			wd if (337.5..=360.0).contains(&wd) || (0.0..22.5).contains(&wd) => Self::N,
			wd if (22.5..67.5).contains(&wd) => Self::NE,
			wd if (67.5..112.5).contains(&wd) => Self::E,
			wd if (112.5..157.5).contains(&wd) => Self::SE,
			wd if (157.5..202.5).contains(&wd) => Self::S,
			wd if (202.5..247.5).contains(&wd) => Self::SW,
			wd if (247.5..292.5).contains(&wd) => Self::W,
			wd if (292.5..337.5).contains(&wd) => Self::NW,
			_ => return Err(anyhow!("Wind from another dimension")),
		};

		Ok(direction)
	}

	pub fn get_icon(&self) -> char {
		match *self {
			Self::N => '',
			Self::NE => '',
			Self::E => '',
			Self::SE => '',
			Self::S => '',
			Self::SW => '',
			Self::W => '',
			Self::NW => '',
		}
	}
}

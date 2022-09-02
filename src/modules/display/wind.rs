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
	pub fn get_direction(wd: f64) -> Result<Self> {
		match wd % 360.0 {
			wd if (337.5..=360.0).contains(&wd) || (0.0..22.5).contains(&wd) => Ok(WindDirection::N),
			wd if (22.5..67.5).contains(&wd) => Ok(WindDirection::NE),
			wd if (67.5..112.5).contains(&wd) => Ok(WindDirection::E),
			wd if (112.5..157.5).contains(&wd) => Ok(WindDirection::SE),
			wd if (157.5..202.5).contains(&wd) => Ok(WindDirection::S),
			wd if (202.5..247.5).contains(&wd) => Ok(WindDirection::SW),
			wd if (247.5..292.5).contains(&wd) => Ok(WindDirection::W),
			wd if (292.5..337.5).contains(&wd) => Ok(WindDirection::NW),
			_ => return Err(anyhow!("Wind from another dimension")),
		}
	}

	pub fn get_icon(&self) -> char {
		match *self {
			WindDirection::N => '',
			WindDirection::NE => '',
			WindDirection::E => '',
			WindDirection::SE => '',
			WindDirection::S => '',
			WindDirection::SW => '',
			WindDirection::W => '',
			WindDirection::NW => '',
		}
	}
}

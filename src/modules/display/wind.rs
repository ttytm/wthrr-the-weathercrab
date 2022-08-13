use anyhow::Result;

pub struct Wind {
	pub direction: WindDirection,
	pub icon: String,
}

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
	pub fn fmt(&self) -> &str {
		match self {
			WindDirection::NW => "NW",
			WindDirection::N => "N",
			WindDirection::NE => "NE",
			WindDirection::E => "E",
			WindDirection::SE => "SE",
			WindDirection::S => "S",
			WindDirection::SW => "SW",
			WindDirection::W => "W",
		}
	}
}

impl Wind {
	pub fn get_direction(wd: f64) -> Result<Self> {
		let (direction, icon);
		match wd % 360.0 {
			wd if (337.5..=360.0).contains(&wd) || (0.0..22.5).contains(&wd) => {
				direction = WindDirection::N;
				icon = "";
			}
			wd if (22.5..67.5).contains(&wd) => {
				direction = WindDirection::NE;
				icon = "";
			}
			wd if (67.5..112.5).contains(&wd) => {
				direction = WindDirection::E;
				icon = "";
			}
			wd if (112.5..157.5).contains(&wd) => {
				direction = WindDirection::SE;
				icon = "";
			}
			wd if (157.5..202.5).contains(&wd) => {
				direction = WindDirection::S;
				icon = "";
			}
			wd if (202.5..247.5).contains(&wd) => {
				direction = WindDirection::SW;
				icon = "";
			}
			wd if (247.5..292.5).contains(&wd) => {
				direction = WindDirection::W;
				icon = "";
			}
			wd if (292.5..337.5).contains(&wd) => {
				direction = WindDirection::NW;
				icon = "";
			}
			_ => panic!("Wind from another dimension"),
		}

		Ok(Wind {
			direction,
			icon: icon.to_string(),
		})
	}
}

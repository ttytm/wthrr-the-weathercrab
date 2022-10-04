use anyhow::Result;
use strum_macros::Display;

#[derive(Display)]
pub enum Border {
	#[strum(serialize = "╭")]
	TL,
	#[strum(serialize = "─")]
	T,
	#[strum(serialize = "╮")]
	TR,
	#[strum(serialize = "│")]
	R,
	#[strum(serialize = "╯")]
	BR,
	#[strum(serialize = "─")]
	B,
	#[strum(serialize = "╰")]
	BL,
	#[strum(serialize = "│")]
	L,
}

pub enum Separator {
	Blank,
	Line,
	_Dotted,
}

impl Separator {
	pub fn fmt(self, width: usize) -> Result<()> {
		match self {
			Self::Blank => Ok(println!("{}{: >width$}{}", Border::L, "", Border::R)),
			Self::Line => Ok(println!("├{:─>width$}┤", "")),
			Self::_Dotted => Ok(println!("├{:┈>width$}┤", "")),
		}
	}
}

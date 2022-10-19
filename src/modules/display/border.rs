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
	Dotted,
}

impl Separator {
	pub fn fmt(self, width: usize) -> String {
		match self {
			Self::Blank => format!("{}{: >width$}{}", Border::L, "", Border::R),
			Self::Line => format!("├{:─>width$}┤", ""),
			Self::Dotted => format!("├{:┄>width$}┤", ""),
		}
	}
}

use strum_macros::Display;

#[derive(Display)]
pub enum BorderGlyph {
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

#[derive(Default)]
pub enum BorderVariant {
	Square,
	SquareHeavy,
	Double,
	#[default]
	Rounded,
}

pub enum Border {
	Top,
	Bottom,
}

pub enum Separator {
	Blank,
	Square,
	SquareHeavy,
	Double,
	Dashed,
}

impl BorderGlyph {
	pub fn fmt(&self, variant: &BorderVariant) -> char {
		match self {
			BorderGlyph::TL => match variant {
				BorderVariant::Square => '┌',
				BorderVariant::SquareHeavy => '┏',
				BorderVariant::Double => '╔',
				_ => '╭',
			},
			BorderGlyph::T | BorderGlyph::B => match variant {
				BorderVariant::Double => '═',
				BorderVariant::SquareHeavy => '━',
				_ => '─',
			},
			BorderGlyph::TR => match variant {
				BorderVariant::Square => '┐',
				BorderVariant::SquareHeavy => '┓',
				BorderVariant::Double => '╗',
				_ => '╮',
			},
			BorderGlyph::R | BorderGlyph::L => match variant {
				BorderVariant::Double => '║',
				BorderVariant::SquareHeavy => '┃',
				_ => '│',
			},
			BorderGlyph::BR => match variant {
				BorderVariant::Square => '┘',
				BorderVariant::SquareHeavy => '┛',
				BorderVariant::Double => '╝',
				_ => '╯',
			},
			BorderGlyph::BL => match variant {
				BorderVariant::Square => '└',
				BorderVariant::SquareHeavy => '┗',
				BorderVariant::Double => '╚',
				_ => '╰',
			},
		}
	}
}

impl Border {
	pub fn fmt(self, width: usize, variant: &BorderVariant) -> String {
		match self {
			Self::Top => format!(
				"{}{: >width$}{}",
				BorderGlyph::TL.fmt(variant),
				BorderGlyph::T.fmt(variant).to_string().repeat(width),
				BorderGlyph::TR.fmt(variant),
			),
			Self::Bottom => format!(
				"{}{: >width$}{}",
				BorderGlyph::BL.fmt(variant),
				BorderGlyph::B.fmt(variant).to_string().repeat(width),
				BorderGlyph::BR.fmt(variant),
			),
		}
	}
}

impl Separator {
	pub fn fmt(self, width: usize, border_variant: &BorderVariant) -> String {
		match self {
			Self::Blank => format!(
				"{}{: >width$}{}",
				BorderGlyph::L.fmt(border_variant),
				"",
				BorderGlyph::R.fmt(border_variant)
			),
			Self::Dashed => format!("├{:┈>width$}┤", ""),
			Self::Square => format!("{}{:─>width$}{}", '├', "", '┤'),
			Self::SquareHeavy => format!("┠{:─>width$}┨", ""),
			Self::Double => format!("╟{:─>width$}╢", ""),
			// Self::Double => format!("╠{:═>width$}╣", ""),
		}
	}
}

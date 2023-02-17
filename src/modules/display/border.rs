use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display};

#[derive(Display, AsRefStr)]
pub enum Border {
	TL,
	T,
	TR,
	R,
	BR,
	B,
	BL,
	L,
}

impl Border {
	pub fn fmt<'a>(&self, variant: &BorderStyle) -> &'a str {
		match self {
			Border::TL => match variant {
				BorderStyle::single => "┌",
				BorderStyle::solid => "┏",
				BorderStyle::double => "╔",
				_ => "╭",
			},
			Border::T | Border::B => match variant {
				BorderStyle::double => "═",
				BorderStyle::solid => "━",
				_ => "─",
			},
			Border::TR => match variant {
				BorderStyle::single => "┐",
				BorderStyle::solid => "┓",
				BorderStyle::double => "╗",
				_ => "╮",
			},
			Border::R | Border::L => match variant {
				BorderStyle::double => "║",
				BorderStyle::solid => "┃",
				_ => "│",
			},
			Border::BR => match variant {
				BorderStyle::single => "┘",
				BorderStyle::solid => "┛",
				BorderStyle::double => "╝",
				_ => "╯",
			},
			Border::BL => match variant {
				BorderStyle::single => "└",
				BorderStyle::solid => "┗",
				BorderStyle::double => "╚",
				_ => "╰",
			},
		}
	}
}

#[derive(Default, Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum BorderStyle {
	#[default]
	rounded,
	single,
	solid,
	double,
}

pub enum Edge {
	Top,
	Bottom,
}

impl Edge {
	pub fn fmt(self, width: usize, variant: &BorderStyle) -> String {
		match self {
			Self::Top => format!(
				"{}{: >width$}{}",
				Border::TL.fmt(variant),
				Border::T.fmt(variant).repeat(width),
				Border::TR.fmt(variant),
			),
			Self::Bottom => format!(
				"{}{: >width$}{}",
				Border::BL.fmt(variant),
				Border::B.fmt(variant).repeat(width),
				Border::BR.fmt(variant),
			),
		}
	}
}

pub enum Separator {
	Blank,
	Single,
	Solid,
	Double,
	Dashed,
}

impl Separator {
	pub fn fmt(self, width: usize, border_variant: &BorderStyle) -> String {
		match self {
			Self::Blank => format!(
				"{}{: >width$}{}",
				Border::L.fmt(border_variant),
				"",
				Border::R.fmt(border_variant)
			),
			Self::Dashed => format!("├{:┈>width$}┤", ""),
			Self::Single => format!("├{:─>width$}┤", ""),
			Self::Solid => format!("┠{:─>width$}┨", ""),
			Self::Double => format!("╟{:─>width$}╢", ""),
		}
	}
}

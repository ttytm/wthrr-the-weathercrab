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
	pub fn fmt<'a>(&self, style: &BorderStyle) -> &'a str {
		match self {
			Self::TL => match style {
				BorderStyle::single => "┌",
				BorderStyle::solid => "┏",
				BorderStyle::double => "╔",
				BorderStyle::rounded => "╭",
			},
			Self::T | Self::B => match style {
				BorderStyle::double => "═",
				BorderStyle::solid => "━",
				_ => "─",
			},
			Self::TR => match style {
				BorderStyle::single => "┐",
				BorderStyle::solid => "┓",
				BorderStyle::double => "╗",
				BorderStyle::rounded => "╮",
			},
			Self::R | Self::L => match style {
				BorderStyle::double => "║",
				BorderStyle::solid => "┃",
				_ => "│",
			},
			Self::BR => match style {
				BorderStyle::single => "┘",
				BorderStyle::solid => "┛",
				BorderStyle::double => "╝",
				BorderStyle::rounded => "╯",
			},
			Self::BL => match style {
				BorderStyle::single => "└",
				BorderStyle::solid => "┗",
				BorderStyle::double => "╚",
				BorderStyle::rounded => "╰",
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
	pub fn fmt(self, width: usize, style: &BorderStyle) -> String {
		match self {
			Self::Top => format!(
				"{}{: >width$}{}",
				Border::TL.fmt(style),
				Border::T.fmt(style).repeat(width),
				Border::TR.fmt(style),
			),
			Self::Bottom => format!(
				"{}{: >width$}{}",
				Border::BL.fmt(style),
				Border::B.fmt(style).repeat(width),
				Border::BR.fmt(style),
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
	pub fn fmt(self, width: usize, style: &BorderStyle) -> String {
		match self {
			Self::Blank => format!("{}{: >width$}{}", Border::L.fmt(style), "", Border::R.fmt(style)),
			Self::Dashed => format!("├{:┈>width$}┤", ""),
			Self::Single => format!("├{:─>width$}┤", ""),
			Self::Solid => format!("┠{:─>width$}┨", ""),
			Self::Double => format!("╟{:─>width$}╢", ""),
		}
	}
}

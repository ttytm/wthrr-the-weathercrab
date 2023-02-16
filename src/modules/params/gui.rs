use colored::{Color, ColoredString, Colorize};
use optional_struct::*;
use serde::{Deserialize, Serialize};

use crate::modules::display::{border::BorderVariant, graph::GraphVariant};

#[optional_struct(ConfigFileGui)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Gui {
	pub border: BorderVariant,
	pub color: ColorVariant,
	pub graph: GraphVariant,
	pub greeting: bool,
}

#[derive(Default, Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ColorVariant {
	#[default]
	default,
	plain,
}

impl Default for Gui {
	fn default() -> Self {
		Self {
			border: BorderVariant::default(),
			color: ColorVariant::default,
			graph: GraphVariant::default(),
			greeting: true,
		}
	}
}

pub trait ColorOption {
	fn color_option(self, color: Color, color_cfg: &ColorVariant) -> ColoredString;
}

impl<'a> ColorOption for &'a str {
	fn color_option(self, color: Color, color_cfg: &ColorVariant) -> ColoredString {
		match color_cfg {
			&ColorVariant::plain => self.normal(),
			_ => self.color(color),
		}
	}
}

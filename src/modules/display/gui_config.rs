use dialoguer::console::{style, StyledObject};
use optional_struct::{optional_struct, Applyable};
use serde::{Deserialize, Serialize};

use crate::modules::display::{
	border::BorderStyle,
	graph::{ConfigFileGraphOpts, GraphOpts},
};

#[optional_struct(ConfigFileGui)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Gui {
	pub border: BorderStyle,
	pub color: ColorVariant,
	#[optional_rename(ConfigFileGraphOpts)]
	pub graph: GraphOpts,
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
			border: BorderStyle::default(),
			color: ColorVariant::default,
			graph: GraphOpts::default(),
			greeting: true,
		}
	}
}

pub trait ConfigurableColor<'a> {
	fn plain_or_bright_black(self, config_color: &ColorVariant) -> StyledObject<&'a str>;
	fn plain_or_yellow(self, config_color: &ColorVariant) -> StyledObject<&'a str>;
	fn plain_or_blue(self, config_color: &ColorVariant) -> StyledObject<&'a str>;
}

impl<'a> ConfigurableColor<'a> for &'a str {
	fn plain_or_bright_black(self, config_color: &ColorVariant) -> StyledObject<&'a str> {
		match config_color {
			ColorVariant::plain => style(self),
			ColorVariant::default => style(self).black().bright(),
		}
	}
	fn plain_or_yellow(self, config_color: &ColorVariant) -> StyledObject<&'a str> {
		match config_color {
			ColorVariant::plain => style(self),
			ColorVariant::default => style(self).yellow(),
		}
	}
	fn plain_or_blue(self, config_color: &ColorVariant) -> StyledObject<&'a str> {
		match config_color {
			ColorVariant::plain => style(self),
			ColorVariant::default => style(self).blue(),
		}
	}
}

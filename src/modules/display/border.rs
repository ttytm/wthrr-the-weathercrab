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
	pub fn fmt(&self) -> &str {
		match self {
			Border::TL => "╭",
			Border::T => "─",
			Border::TR => "╮",
			Border::R => "│",
			Border::BR => "╯",
			Border::B => "─",
			Border::BL => "╰",
			Border::L => "│",
		}
	}
}

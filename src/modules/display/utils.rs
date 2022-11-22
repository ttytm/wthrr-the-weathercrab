use anyhow::Result;
use colored::{Color, ColoredString, Colorize};
use regex::Regex;

pub fn adjust_lang_width(string: &str, lang: &str) -> usize {
	let correction = match lang {
		"zh" => {
			let re = Regex::new(r"\p{han}").unwrap();
			re.find_iter(string).count()
		}
		"ko" => {
			let re = Regex::new(r"[\u3131-\uD79D\w]").unwrap();
			let nu = Regex::new(r"[0-9\.]").unwrap();
			re.find_iter(string).count() - nu.find_iter(string).count()
		}
		"ja" => {
			let re = Regex::new(r"[ぁ-んァ-ン\w]").unwrap();
			let nu = Regex::new(r"[0-9\.]").unwrap();
			re.find_iter(string).count() - nu.find_iter(string).count()
		}
		_ => 0,
	};

	correction
}

pub fn style_number(mut num: i32, sub: bool) -> Result<String> {
	const SUPERSCRIPT_DIGITS: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
	const SUBSCRIPT_DIGITS: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];

	let mut result = String::new();

	if num == 0 {
		result.push(match sub {
			true => SUBSCRIPT_DIGITS[0],
			_ => SUPERSCRIPT_DIGITS[0],
		});
		return Ok(result);
	}

	if num < 0 {
		num = -num;
		result.push(if sub { '₋' } else { '⁻' });
	}

	let mut started = false;
	let mut power_of_ten = 1_000_000_000;
	for _ in 0..10 {
		let digit = num / power_of_ten;
		num -= digit * power_of_ten;
		power_of_ten /= 10;
		if digit != 0 || started {
			started = true;
			result.push(match sub {
				true => SUBSCRIPT_DIGITS[digit as usize],
				_ => SUPERSCRIPT_DIGITS[digit as usize],
			})
		}
	}

	Ok(result)
}

pub trait ColorOption {
	fn color_option(self, color: Color) -> ColoredString;
}

impl<'a> ColorOption for &'a str {
	fn color_option(self, color: Color) -> ColoredString {
		let use_color = true;
		if use_color {
			self.color(color)
		} else {
			self.normal()
		}
	}
}

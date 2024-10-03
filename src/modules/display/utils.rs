use regex::Regex;
use unicode_width::UnicodeWidthStr;

use crate::modules::display::product::Product;

impl Product<'_> {
	pub fn trunc_address(mut address: String, max_width: usize) -> String {
		let address_len = address.chars().count();

		address = if address_len > max_width {
			// For most locations with overly long addresses, the results seem to be better if
			// truncated between the first and second comma instead the penultimate and last comma.
			// let last_comma = title.matches(',').count();
			let prep_re = format!("^((?:[^,]*,){{{}}})[^,]*,(.*)", 1);
			let re = Regex::new(&prep_re).unwrap();

			re.replace(&address, "$1$2").to_string()
		} else {
			address
		};

		if address_len > max_width {
			address = Self::trunc_address(address, max_width);
		}

		address
	}
}

pub fn pad_string_to_width(s: &str, total_width: usize) -> String {
	let current_width = s.width(); // Effective width of the string
	if current_width >= total_width {
		s.to_string() // No padding needed if already wide enough
	} else {
		let padding = total_width - current_width;
		format!("{}{}", s, " ".repeat(padding))
	}
}

pub fn style_number(mut num: i32, sub: bool) -> String {
	const SUPERSCRIPT_DIGITS: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
	const SUBSCRIPT_DIGITS: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];

	let mut result = String::new();

	if num == 0 {
		result.push(if sub {
			SUBSCRIPT_DIGITS[0]
		} else {
			SUPERSCRIPT_DIGITS[0]
		});
		return result;
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
			result.push(if sub {
				SUBSCRIPT_DIGITS[digit as usize]
			} else {
				SUPERSCRIPT_DIGITS[digit as usize]
			});
		}
	}

	result
}

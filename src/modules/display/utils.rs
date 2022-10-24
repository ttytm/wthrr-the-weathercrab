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

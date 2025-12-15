pub struct Translit;

impl Translit {
	pub fn convert(input: Option<String>) -> String {
		if input.is_none() {
			return String::new();
		}

		let input = input.unwrap();
		let mut result = String::new();

		for ch in input.chars() {
			let replacement = match ch {
				'а' => "a",
				'б' => "b",
				'в' => "v",
				'г' => "g",
				'д' => "d",
				'е' => "e",
				'ё' => "yo",
				'ж' => "zh",
				'з' => "z",
				'и' => "i",
				'й' => "y",
				'к' => "k",
				'л' => "l",
				'м' => "m",
				'н' => "n",
				'о' => "o",
				'п' => "p",
				'р' => "r",
				'с' => "s",
				'т' => "t",
				'у' => "u",
				'ф' => "f",
				'х' => "kh",
				'ц' => "ts",
				'ч' => "ch",
				'ш' => "sh",
				'щ' => "shch",
				'ы' => "y",
				'э' => "e",
				'ю' => "yu",
				'я' => "ya",

				'А' => "A",
				'Б' => "B",
				'В' => "V",
				'Г' => "G",
				'Д' => "D",
				'Е' => "E",
				'Ё' => "Yo",
				'Ж' => "Zh",
				'З' => "Z",
				'И' => "I",
				'Й' => "Y",
				'К' => "K",
				'Л' => "L",
				'М' => "M",
				'Н' => "N",
				'О' => "O",
				'П' => "P",
				'Р' => "R",
				'С' => "S",
				'Т' => "T",
				'У' => "U",
				'Ф' => "F",
				'Х' => "Kh",
				'Ц' => "Ts",
				'Ч' => "Ch",
				'Ш' => "Sh",
				'Щ' => "Shch",
				'Ы' => "Y",
				'Э' => "E",
				'Ю' => "Yu",
				'Я' => "Ya",

				_ => &ch.to_string(),
			};

			result.push_str(replacement);
		}

		result
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_transliteration() {
		assert_eq!(Translit::convert(Some("привет".to_string())), "privet");
		assert_eq!(Translit::convert(Some("Москва".to_string())), "Moskva");
		assert_eq!(Translit::convert(Some("Hello".to_string())), "Hello");
	}
}

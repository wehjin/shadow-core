#[cfg(test)]
mod tests {
	use crate::{Line, Shadow};

	#[test]
	fn create() {
		let shadow = Shadow {
			title: "Bob's Speech".to_string(),
			lines: vec![Line {
				speaker: "Bob".to_string(),
				description: "Hello world!".to_string(),
				start: 1.25,
				end: 1.77,
			}],
			audio_url: "https://example.com/bob.mp3".to_string(),
		};
		assert_eq!(shadow.title, "Bob's Speech");
	}
}

#[derive(Clone, Debug)]
pub struct Shadow {
	pub title: String,
	pub lines: Vec<Line>,
	pub audio_url: String,
}

#[derive(Clone, Debug)]
pub struct Line {
	pub speaker: String,
	pub description: String,
	pub start: f64,
	pub end: f64,
}

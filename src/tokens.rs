use std::{fmt::Display, slice::Iter};

use crate::token::Token;

pub struct Tokens {
	items: Vec<Token>,
}

impl Default for Tokens {
	fn default() -> Self {
		Self { items: vec![] }
	}
}

impl Tokens {
	pub fn push(&mut self, token: Token) {
		self.items.push(token);
	}

	pub fn iter(&self) -> Iter<Token> {
		self.items.iter()
	}
}

impl Display for Tokens {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = writeln!(f, "Tokens:");

		for t in self.iter() { let _ = writeln!(f, "\t{t}"); }

		Ok(())
	}
}

use std::{fmt::Display, slice::Iter};

use crate::token::Token;

pub struct Tokens {
	items: Vec<Token>,
	meta:  Vec<(u64, u64)>,
}

impl Default for Tokens {
	fn default() -> Self {
		Self { items: vec![], meta: vec![] }
	}
}

impl Tokens {
	pub fn peek(&self, index: u64) -> Option<&Token> {
		if self.items.len() > index as usize {
			Some(&self.items[index as usize])
		}
		else { None}
	}

	pub fn push(&mut self, token: Token, line: u64, column: u64) {
		self.items.push(token);
		self.meta.push((line, column));
	}

	pub fn iter(&self) -> Iter<Token> {
		self.items.iter()
	}

	pub fn meta(&self, item: u64) -> &(u64, u64) {
		&self.meta[item as usize]
	}
}

impl Display for Tokens {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = writeln!(f, "Tokens:");

		for t in self.iter() { let _ = writeln!(f, "\t{t}"); }

		Ok(())
	}
}

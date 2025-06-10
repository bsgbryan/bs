use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Util {
  Print,
}

impl Display for Util {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Util::Print	=> write!(f, "util::print"),
		}
	}
}

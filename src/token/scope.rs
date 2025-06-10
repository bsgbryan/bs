use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Scope {
  CloseBracet,
  CloseParen,
  Colon,
  Comma,
  Dot,
  OpenBracet,
  OpenParen,
  SELF,
}

impl Display for Scope {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Scope::CloseBracet 	=> write!(f, "scope::close-bracket(])"),
			Scope::CloseParen 	=> write!(f, "scope::close-paren())"),
			Scope::Colon 				=> write!(f, "scope::colon(:)"),
			Scope::Comma 				=> write!(f, "scope::comma(,)"),
			Scope::Dot 					=> write!(f, "scope::dot(.)"),
			Scope::OpenBracet 	=> write!(f, "scope::open-bracket([)"),
			Scope::OpenParen 		=> write!(f, "scope::open-paren(()"),
			Scope::SELF 				=> write!(f, "scope::self(self)"),
		}
	}
}

use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Literal {
	Bool(bool),
  Identifier(String),
  InterpolatedString(String),
  Number(String),
  String(String),
}

impl Display for Literal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Literal::Bool(b)							 => write!(f, "literal::bool({})", b),
			Literal::Identifier(i) 				 => write!(f, "literal::identifier({})", i),
			Literal::InterpolatedString(i) => write!(f, "literal::interpolated-string({})", i),
			Literal::Number(n) 						 => write!(f, "literal::number({})", n),
			Literal::String(s) 						 => write!(f, "literal::string({})", s),
		}
	}
}

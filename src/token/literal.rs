use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Literal {
  Identifier(String),
  InterpolatedString(String),
  String(String),
  Number(String),
}

impl Display for Literal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Literal::Identifier(i) 				 => write!(f, "literal::identifier({})", i),
			Literal::InterpolatedString(i) => write!(f, "literal::interpolated-string({})", i),
			Literal::Number(n) 						 => write!(f, "literal::number({})", n),
			Literal::String(s) 						 => write!(f, "literal::string({})", s),
		}
	}
}

use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Equality {
  Equal,
  Greater,
  GreaterOrEqual,
  Less,
  LessOrEqual,
  NotEqual,
}

impl Display for Equality {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Equality::Equal 				 => write!(f, "equality::equal(==)"),
			Equality::Greater 			 => write!(f, "equality::greater(> )"),
			Equality::GreaterOrEqual => write!(f, "equality::greater-or-equal(>=)"),
			Equality::Less 					 => write!(f, "equality::less(< )"),
			Equality::LessOrEqual 	 => write!(f, "equality::less-or-equal(<=)"),
			Equality::NotEqual 			 => write!(f, "equality::equal(!=)"),
		}
	}
}

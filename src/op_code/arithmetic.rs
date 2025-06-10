use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum Arithmetic {
  Add,
  Divide,
  Multiply,
  Negate,
  Subtract,
}

impl Display for Arithmetic {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Arithmetic::Add 		 => write!(f, "arithmetic::add"),
			Arithmetic::Divide 	 => write!(f, "arithmetic::divide"),
			Arithmetic::Multiply => write!(f, "arithmetic::multiply"),
			Arithmetic::Negate 	 => write!(f, "arithmetic::negate"),
			Arithmetic::Subtract => write!(f, "arithmetic::subtract"),
		}
	}
}

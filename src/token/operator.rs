use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Operator {
  Add,
  Assign,
  Divide,
  Invert,
  Multiply,
  Negate,
}

impl Display for Operator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Operator::Add 		 => write!(f, "operator::add(+)"),
			Operator::Assign 	 => write!(f, "operator::assign(=)"),
			Operator::Divide 	 => write!(f, "operator::divide(/)"),
			Operator::Invert 	 => write!(f, "operator::invert(!)"),
			Operator::Multiply => write!(f, "operator::multiply(*)"),
			Operator::Negate 	 => write!(f, "operator::negate(-)"),
		}
	}
}

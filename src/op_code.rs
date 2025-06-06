use std::fmt::Display;

use crate::value::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum OpCode {
  Literal(Value),
  ControlFlow(ControlFlow),
  Arithmetic(Arithmetic)
}

impl Display for OpCode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Arithmetic(a) 	=> write!(f, "{a}"),
			Self::ControlFlow(cf) => write!(f, "{cf}"),
			Self::Literal(l) 			=> write!(f, "literal({l})"),
		}
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum Arithmetic {
  Negate,
  Add,
  Divide,
  Multiply,
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

#[derive(Clone, Debug, PartialEq)]
pub enum ControlFlow {
  Return,
}

impl Display for ControlFlow {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ControlFlow::Return => write!(f, "control-flow::return"),
		}
	}
}

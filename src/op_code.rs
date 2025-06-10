use std::fmt::Display;

use crate::value::Value;

mod arithmetic;
mod control_flow;
mod util;

pub use arithmetic::Arithmetic;
pub use control_flow::ControlFlow;
pub use util::Util;

#[derive(Clone, Debug, PartialEq)]
pub enum OpCode {
  Arithmetic(Arithmetic),
  ControlFlow(ControlFlow),
  Literal(Value),
  Util(Util),
}

impl Display for OpCode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Arithmetic(a) 	=> write!(f, "{a}"),
			Self::ControlFlow(cf) => write!(f, "{cf}"),
			Self::Literal(l) 			=> write!(f, "literal({l})"),
			Self::Util(u)					=> write!(f, "{u}"),
		}
	}
}

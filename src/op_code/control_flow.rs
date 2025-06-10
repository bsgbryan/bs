use std::fmt::Display;

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

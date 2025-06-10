use std::fmt::Display;


#[derive(Clone, Debug, PartialEq)]
pub enum Command {
	Invert,
}

impl Display for Command {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Command::Invert => write!(f, "command::invert(!)"),
		}
	}
}

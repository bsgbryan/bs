use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Error {
  Invalid(String),
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::Invalid(e) => write!(f, "error:invalid({})", e)
		}
	}
}

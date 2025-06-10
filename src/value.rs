use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
  Number(f64),
  Bool(bool),
  String(String),
}

impl Display for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Bool(b) 	=> { let _ = write!(f, "bool({})", 			 b); }
			Self::Number(n) => { let _ = write!(f, "number({})", 		 n); }
			Self::String(s) => { let _ = write!(f, "string(\"{}\")", s); }
		}

		Ok(())
	}
}

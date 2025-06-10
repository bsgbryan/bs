use std::fmt::Display;

mod equality;
mod error;
mod keyword;
mod literal;
mod operator;
mod scope;
mod util;

pub use equality::Equality;
pub use error::Error;
pub use keyword::Keyword;
pub use literal::Literal;
pub use operator::Operator;
pub use scope::Scope;
pub use util::Util;

#[derive(Debug, PartialEq)]
pub enum Token {
  Equality(Equality),
  Error(Error),
  Keyword(Keyword),
  Literal(Literal),
  Operator(Operator),
  Scope(Scope),
  Util(Util),
}

impl Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Equality(e) => write!(f, "{e}"),
			Self::Error(e) 		=> write!(f, "{e}"),
			Self::Keyword(k) 	=> write!(f, "{k}"),
			Self::Literal(l) 	=> write!(f, "{l}"),
			Self::Operator(o) => write!(f, "{o}"),
			Self::Scope(s) 		=> write!(f, "{s}"),
			Self::Util(u)			=> write!(f, "{u}"),
		}
	}
}

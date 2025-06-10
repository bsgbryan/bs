use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Keyword {
  Const,
  Else,
  Fun,
  If,
  Loop,
  Print,
  Return,
  Var,
  While,
}

impl Display for Keyword {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Keyword::Const  => write!(f, "keyword::const"),
			Keyword::Else 	=> write!(f, "keyword::else"),
			Keyword::Fun 	 	=> write!(f, "keyword::fun"),
			Keyword::If 		=> write!(f, "keyword::if"),
			Keyword::Loop 	=> write!(f, "keyword::loop"),
			Keyword::Print  => write!(f, "keyword::print"),
			Keyword::Return => write!(f, "keyword::return"),
			Keyword::Var 	 	=> write!(f, "keyword::var"),
			Keyword::While  => write!(f, "keyword::while"),
		}
	}
}

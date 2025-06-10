use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Keyword {
  // And,
  Const,
  Else,
  False,
  Fun,
  If,
  Loop,
  // Or,
  Print,
  Return,
  True,
  Var,
  While,
}

impl Display for Keyword {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			// Keyword::And 	 	=> write!(f, "keyword::and"),
			Keyword::Const  => write!(f, "keyword::const"),
			Keyword::Else 	=> write!(f, "keyword::else"),
			Keyword::False  => write!(f, "keyword::false"),
			Keyword::Fun 	 	=> write!(f, "keyword::fun"),
			Keyword::If 		=> write!(f, "keyword::if"),
			Keyword::Loop 	=> write!(f, "keyword::loop"),
			// Keyword::Or 		=> write!(f, "keyword::or"),
			Keyword::Print  => write!(f, "keyword::print"),
			Keyword::Return => write!(f, "keyword::return"),
			Keyword::True 	=> write!(f, "keyword::true"),
			Keyword::Var 	 	=> write!(f, "keyword::var"),
			Keyword::While  => write!(f, "keyword::while"),
		}
	}
}

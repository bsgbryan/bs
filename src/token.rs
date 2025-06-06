use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Token {
  Equality(Equality),
  Error(Error),
  Keyword(Keyword),
  Literal(Literal),
  Operator(Operator),
  Scope(Scope),
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
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum Equality {
  Equal,
  Greater,
  GreaterOrEqual,
  Less,
  LessOrEqual,
  NotEqual,
}

impl Display for Equality {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Equality::Equal 				 => write!(f, "equality::equal(==)"),
			Equality::Greater 			 => write!(f, "equality::greater(> )"),
			Equality::GreaterOrEqual => write!(f, "equality::greater-or-equal(>=)"),
			Equality::Less 					 => write!(f, "equality::less(< )"),
			Equality::LessOrEqual 	 => write!(f, "equality::less-or-equal(<=)"),
			Equality::NotEqual 			 => write!(f, "equality::equal(!=)"),
		}
	}
}

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

#[derive(Debug, PartialEq)]
pub enum Keyword {
  And,
  Const,
  Else,
  False,
  Fun,
  If,
  Loop,
  Or,
  Print,
  Return,
  True,
  Var,
  While,
}

impl Display for Keyword {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Keyword::And 	 	=> write!(f, "keyword::and"),
			Keyword::Const  => write!(f, "keyword::const"),
			Keyword::Else 	=> write!(f, "keyword::else"),
			Keyword::False  => write!(f, "keyword::false"),
			Keyword::Fun 	 	=> write!(f, "keyword::fun"),
			Keyword::If 		=> write!(f, "keyword::if"),
			Keyword::Loop 	=> write!(f, "keyword::loop"),
			Keyword::Or 		=> write!(f, "keyword::or"),
			Keyword::Print  => write!(f, "keyword::print"),
			Keyword::Return => write!(f, "keyword::return"),
			Keyword::True 	=> write!(f, "keyword::true"),
			Keyword::Var 	 	=> write!(f, "keyword::var"),
			Keyword::While  => write!(f, "keyword::while"),
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum Literal {
  Identifier(String),
  InterpolatedString(String),
  String(String),
  Number(String),
}

impl Display for Literal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Literal::Identifier(i) 				 => write!(f, "literal::identifier({})", i),
			Literal::InterpolatedString(i) => write!(f, "literal::interpolated-string({})", i),
			Literal::Number(n) 						 => write!(f, "literal::number({})", n),
			Literal::String(s) 						 => write!(f, "literal::string({})", s),
		}
	}
}

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

#[derive(Debug, PartialEq)]
pub enum Scope {
  CloseBracet,
  CloseParen,
  Colon,
  Comma,
  Dot,
  OpenBracet,
  OpenParen,
  SELF,
}

impl Display for Scope {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Scope::CloseBracet 	=> write!(f, "scope::close-bracket(])"),
			Scope::CloseParen 	=> write!(f, "scope::close-paren())"),
			Scope::Colon 				=> write!(f, "scope::colon(:)"),
			Scope::Comma 				=> write!(f, "scope::comma(,)"),
			Scope::Dot 					=> write!(f, "scope::dot(.)"),
			Scope::OpenBracet 	=> write!(f, "scope::open-bracket([)"),
			Scope::OpenParen 		=> write!(f, "scope::open-paren(()"),
			Scope::SELF 				=> write!(f, "scope::self(self)"),
		}
	}
}

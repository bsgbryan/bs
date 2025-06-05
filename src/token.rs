#[derive(Debug, PartialEq)]
pub enum Token {
  Equality(Equality),
  Error(Error),
  Keyword(Keyword),
  Literal(Literal),
  Operator(Operator),
  Scope(Scope),
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

#[derive(Debug, PartialEq)]
pub enum Error {
  Invalid(String),
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

#[derive(Debug, PartialEq)]
pub enum Literal {
  Identifier(String),
  InterpolatedString(String),
  String(String),
  Number(String),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
  Add,
  Assign,
  Divide,
  Invert,
  Multiply,
  Negate,
  Not,
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
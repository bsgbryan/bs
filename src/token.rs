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
  Equal(u64),
  Greater(u64),
  GreaterOrEqual(u64),
  Less(u64),
  LessOrEqual(u64),
  NotEqual(u64),
}

#[derive(Debug, PartialEq)]
pub enum Error {
  Invalid(String, u64),
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
  And(u64),
  Const(u64),
  Else(u64),
  False,
  Fun(u64),
  If(u64),
  Loop(u64),
  Or(u64),
  Print(u64),
  Return(u64),
  True,
  Var(u64),
  While(u64),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
  Identifier(String, u64),
  InterpolatedString(String, u64),
  String(String, u64),
  Number(String, u64),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
  Add,
  Assign(u64),
  Divide,
  Invert,
  Multiply,
  Negate,
  Not(u64),
}

#[derive(Debug, PartialEq)]
pub enum Scope {
  CloseBracet(u64),
  CloseParen(u64),
  Colon(u64),
  Comma(u64),
  Dot(u64),
  OpenBracet(u64),
  OpenParen(u64),
  SELF(u64),
}
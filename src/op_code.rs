use crate::value::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum OpCode {
  Literal(Value),
  ControlFlow(ControlFlow),
  Arithmetic(Arithmetic)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Arithmetic {
  Negate,
  Add,
  Divide,
  Multiply,
  Subtract,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ControlFlow {
  Return,
}
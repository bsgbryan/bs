use crate::value::{
  Value,
  // ValuePool,
};

#[derive(Clone, Debug)]
pub enum OpCode {
  // Constant(ValuePool),
  Literal(Value),
  Return,
  // --- Arithmetic --- //
  // Unary
  Negate,
  // Binary
  Add,
  Divide,
  Multiply,
  Subtract,
}
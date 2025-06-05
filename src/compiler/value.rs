use crate::{
  chunk::Chunk,
  op_code::OpCode,
  token::Literal,
};

pub fn expression(token: &Literal, chunk: &mut Chunk) {
  match token {
    Literal::Number(n) =>
      if let Ok(value) = n.parse::<f64>() {
        chunk.append(&OpCode::Literal(value));
      }
    _ => (/* TODO Implement support for other literal types */)
  }
}

#[cfg(test)]
mod validate {
  use crate::{
    compiler::{
      Chunk,
      value::expression,
    },
    op_code::OpCode,
    token::Literal::Number,
  };

  #[test]
  fn code() {
    let value = "5".to_string();
    let token = Number(value);

    let mut chunk = Chunk::new();

    let _ = expression(&token, &mut chunk);

    assert_eq!(chunk.codes[0], OpCode::Literal(5.0));
  }
}
use crate::{
  chunk::Chunk,
  op_code::OpCode,
  token::Literal,
};

pub fn expression(token: &Literal, chunk: &mut Chunk) -> u64 {
  let mut out = 0;

  match token {
    Literal::Number(n, line) =>
      if let Ok(value) = n.parse::<f64>() {
        out = *line;
        chunk.append(&OpCode::Literal(value), *line);
      }
    _ => (/* TODO Implement support for other literal types */)
  }

  out
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
    let token = Number(value, 0);

    let mut chunk = Chunk::new();

    let _ = expression(&token, &mut chunk);

    assert_eq!(chunk.codes[0], OpCode::Literal(5.0));
  }

  #[test]
  fn line() {
    let value = "5".to_string();
    let token = Number(value, 1);

    let mut chunk = Chunk::new();

    let _ = expression(&token, &mut chunk);

    assert_eq!(chunk.lines[0], 1);
  }

  #[test]
  fn returned_value() {
    let value = "5".to_string();
    let token = Number(value, 1);

    let mut chunk = Chunk::new();

    let line = expression(&token, &mut chunk);

    assert_eq!(line, 1);
  }
}
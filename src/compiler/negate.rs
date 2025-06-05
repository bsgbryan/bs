use crate::{
  chunk::Chunk,
  op_code::OpCode,
  token::Token,
};

use super::value::expression as value;

pub fn expression(
  token: &    Token,
  chunk: &mut Chunk,
) -> u64 {
  let mut line = 0;

  match token {
    Token::Literal(l) => { line = value(l, chunk) }
    _ => (/* TODO Implement error reporting here */)
  }

  use crate::op_code::Arithmetic::Negate;

  chunk.append(&OpCode::Arithmetic(Negate), line);

  line
}

#[cfg(test)]
mod validate {
  mod codes {
    use crate::{
      compiler::{
        Chunk,
        negate::expression,
      },
      op_code::OpCode,
      token::{
        Literal::Number,
        Token,
      },
    };

    #[test]
    fn correctly_ordered() {
      let value = "5".to_string();
      let token = Token::Literal(Number(value, 0));

      let mut chunk = Chunk::new();

      expression(&token, &mut chunk);

      assert_eq!(chunk.codes[0], OpCode::Literal(5.0));

      use crate::op_code::Arithmetic::Negate;

      assert_eq!(chunk.codes[1], OpCode::Arithmetic(Negate));
    }
  }

  mod lines {
    use crate::{
      compiler::{
        Chunk,
        negate::expression,
      },
      token::{
        Literal::Number,
        Token,
      },
    };

    #[test]
    fn correctly_specified() {
      let value = "5".to_string();
      let token = Token::Literal(Number(value, 1));

      let mut chunk = Chunk::new();

      expression(&token, &mut chunk);

      assert_eq!(chunk.lines[0], 1);
      assert_eq!(chunk.lines[1], 1);
    }
  }
}
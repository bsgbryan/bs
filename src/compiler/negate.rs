use crate::{
  chunk::Chunk,
  op_code::OpCode,
  token::Token,
};

use super::value::expression as value;

pub fn expression(
  token: &    Token,
  chunk: &mut Chunk,
) {
  match token {
    Token::Literal(l) => { value(l, chunk) }
    _ => (/* TODO Implement error reporting here */)
  }

  use crate::op_code::Arithmetic::Negate;

  chunk.append(&OpCode::Arithmetic(Negate));
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
        Literal::Number as NumberToken,
        Token,
      },
      value::Value::Number,
    };

    #[test]
    fn correctly_ordered() {
      let value = "5".to_string();
      let token = Token::Literal(NumberToken(value));

      let mut chunk = Chunk::new();

      expression(&token, &mut chunk);

      assert_eq!(chunk.codes[0], OpCode::Literal(Number(5.0)));

      use crate::op_code::Arithmetic::Negate;

      assert_eq!(chunk.codes[1], OpCode::Arithmetic(Negate));
    }
  }
}
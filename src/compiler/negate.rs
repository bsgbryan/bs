use crate::{
  chunk::Chunk,
  op_code::OpCode,
  token::Token,
};

use super::value::expression as value;

pub fn expression(
  token: &    Token,
  chunk: &mut Chunk,
  line:	  u64,
  column: u64,
) {
	match token {
    Token::Literal(l) => { value(l, chunk) }
    _ => {
	   	let c = column + 1;
	    panic!("line {line}, column {c}: {token} cannot be negated");
    }
  }

  use crate::op_code::Arithmetic::Negate;

  chunk.append(OpCode::Arithmetic(Negate));
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

      expression(&token, &mut chunk, 0, 0);

      assert_eq!(chunk.codes[0], OpCode::Literal(Number(5.0)));

      use crate::op_code::Arithmetic::Negate;

      assert_eq!(chunk.codes[1], OpCode::Arithmetic(Negate));
    }
  }
}

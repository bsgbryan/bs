use crate::{
  chunk::Chunk,
  op_code::OpCode,
  token::Literal::{
  	Bool,
  	Number,
	  self,
  },
  value::Value::{
	  Bool 	 as BoolValue,
	  Number as NumberValue,
  },
};

pub fn expression(token: &Literal, chunk: &mut Chunk) {
  match token {
    Number(n) => {
      if let Ok(value) = n.parse::<f64>() {
        chunk.append(OpCode::Literal(NumberValue(value)));
      }
      else { panic!("{n} is not a valid number") }
    }
    Bool(b) => chunk.append(OpCode::Literal(BoolValue(*b))),
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
    token::Literal::Number as NumberToken,
    value::Value::Number,
  };

  #[test]
  fn code() {
    let value = "5".to_string();
    let token = NumberToken(value);

    let mut chunk = Chunk::new();

    let _ = expression(&token, &mut chunk);

    assert_eq!(chunk.codes[0], OpCode::Literal(Number(5.0)));
  }
}

use std::slice::Iter;

use crate::{
  chunk::Chunk,
  compiler::{
    negate::expression as negate,
     value::expression as value,
  },
  op_code::OpCode,
  token::{
    Operator,
    Token,
  },
};

pub fn expression(
  op:    			OpCode,
  token: &    Token,
  chunk: &mut Chunk,
  iter:  &mut Iter<Token>,
  line:	  u64,
  column: u64,
) {
	match token {
    Token::Operator(o) => match o {
      Operator::Negate => if let Some(v) = iter.next() {
        negate(v, chunk, line, column);
      }
      _ => panic!("line {line}, column {column}: Invalid variant({op:?}) for Operator({o:?})")
    }
    Token::Literal(l) => { value(l, chunk); }
    _ => panic!("line {line}, column {column}: Invalid Token({token:?}) for OpCode({op:?})")
  };

  chunk.append(op);
}

#[cfg(test)]
mod validate {
  mod codes {
    mod without_negation {
      use crate::{
        compiler::{
          Chunk,
          arithmetic::expression,
        },
        op_code::OpCode,
        token::{
          Literal::Number as NumberToken,
          Token,
        },
        value::Value::Number,
      };

      #[test]
      fn add() {
        use crate::op_code::Arithmetic::Add;

        let value = "5".to_string();
        let token = Token::Literal(NumberToken(value));

        let mut chunk = Chunk::new();

        expression(OpCode::Arithmetic(Add), &token, &mut chunk, &mut vec![].iter(), 0, 0);

        assert_eq!(chunk.codes[0], OpCode::Literal(Number(5.0)));
        assert_eq!(chunk.codes[1], OpCode::Arithmetic(Add));
      }

      #[test]
      fn divide() {
        use crate::op_code::Arithmetic::Divide;

        let value = "5".to_string();
        let token = Token::Literal(NumberToken(value));

        let mut chunk = Chunk::new();

        expression(OpCode::Arithmetic(Divide), &token, &mut chunk, &mut vec![].iter(), 0, 0);

        assert_eq!(chunk.codes[0], OpCode::Literal(Number(5.0)));
        assert_eq!(chunk.codes[1], OpCode::Arithmetic(Divide));
      }

      #[test]
      fn multiply() {
        use crate::op_code::Arithmetic::Multiply;

        let value = "5".to_string();
        let token = Token::Literal(NumberToken(value));

        let mut chunk = Chunk::new();

        expression(OpCode::Arithmetic(Multiply), &token, &mut chunk, &mut vec![].iter(), 0, 0);

        assert_eq!(chunk.codes[0], OpCode::Literal(Number(5.0)));
        assert_eq!(chunk.codes[1], OpCode::Arithmetic(Multiply));
      }
    }

    mod with_negation {
      use crate::{
        compiler::{
          Chunk,
          arithmetic::expression,
        },
        op_code::{
          Arithmetic::Negate,
          OpCode,
        },
        token::{
          Literal::Number  as NumberToken,
          Operator::Negate as NegateToken,
          Token,
        },
        value::Value::Number,
      };

      #[test]
      fn add() {
        use crate::op_code::Arithmetic::Add;

        let     token = Token::Operator(NegateToken);
        let mut chunk = Chunk::new();

        let value = "5".to_string();
        let num   = vec![Token::Literal(NumberToken(value))];

        expression(OpCode::Arithmetic(Add), &token, &mut chunk, &mut num.iter(), 0, 0);

        assert_eq!(chunk.codes[0], OpCode::Literal(Number(5.0)));
        assert_eq!(chunk.codes[1], OpCode::Arithmetic(Negate));
        assert_eq!(chunk.codes[2], OpCode::Arithmetic(Add));
      }

      #[test]
      fn divide() {
        use crate::op_code::Arithmetic::Divide;

        let     token = Token::Operator(NegateToken);
        let mut chunk = Chunk::new();

        let value = "5".to_string();
        let num   = vec![Token::Literal(NumberToken(value))];

        expression(OpCode::Arithmetic(Divide), &token, &mut chunk, &mut num.iter(), 0, 0);

        assert_eq!(chunk.codes[0], OpCode::Literal(Number(5.0)));
        assert_eq!(chunk.codes[1], OpCode::Arithmetic(Negate));
        assert_eq!(chunk.codes[2], OpCode::Arithmetic(Divide));
      }

      #[test]
      fn multiply() {
        use crate::op_code::Arithmetic::Multiply;

        let     token = Token::Operator(NegateToken);
        let mut chunk = Chunk::new();

        let value = "5".to_string();
        let num   = vec![Token::Literal(NumberToken(value))];

        expression(OpCode::Arithmetic(Multiply), &token, &mut chunk, &mut num.iter(), 0, 0);

        assert_eq!(chunk.codes[0], OpCode::Literal(Number(5.0)));
        assert_eq!(chunk.codes[1], OpCode::Arithmetic(Negate));
        assert_eq!(chunk.codes[2], OpCode::Arithmetic(Multiply));
      }
    }
  }
}

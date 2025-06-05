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

pub fn expression<'a>(
  op:    &OpCode,
  token: &    Token,
  chunk: &mut Chunk,
  iter:  &mut Iter<'a, Token>,
) {
  let mut line = 0;
  
  match token {
    Token::Operator(o) => match o {
      Operator::Negate => if let Some(v) = iter.next() {
        line = negate(v, chunk);
      }
      _ => panic!("Invalid variant({op:?}) for Operator({o:?})")
    }
    Token::Literal(l) => { line = value(l, chunk); }
    _ => panic!("Invalid Token({token:?}) for OpCode({op:?})")
  };

  chunk.append(op, line);
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
          Literal::Number,
          Token,
        },
      };
  
      #[test]
      fn add() {
        use crate::op_code::Arithmetic::Add;

        let op = OpCode::Arithmetic(Add);
  
        let value = "5".to_string();
        let token = Token::Literal(Number(value, 0));
  
        let mut chunk = Chunk::new();
  
        expression(&op, &token, &mut chunk, &mut vec![].iter());
    
        assert_eq!(chunk.codes[0], OpCode::Literal(5.0));
        assert_eq!(chunk.codes[1], op);
      }
  
      #[test]
      fn divide() {
        use crate::op_code::Arithmetic::Divide;

        let op = OpCode::Arithmetic(Divide);
  
        let value = "5".to_string();
        let token = Token::Literal(Number(value, 0));
  
        let mut chunk = Chunk::new();
  
        expression(&op, &token, &mut chunk, &mut vec![].iter());
    
        assert_eq!(chunk.codes[0], OpCode::Literal(5.0));
        assert_eq!(chunk.codes[1], op);
      }
  
      #[test]
      fn multiply() {
        use crate::op_code::Arithmetic::Multiply;

        let op = OpCode::Arithmetic(Multiply);
  
        let value = "5".to_string();
        let token = Token::Literal(Number(value, 0));
  
        let mut chunk = Chunk::new();
  
        expression(&op, &token, &mut chunk, &mut vec![].iter());
    
        assert_eq!(chunk.codes[0], OpCode::Literal(5.0));
        assert_eq!(chunk.codes[1], op);
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
          Literal::Number,
          Operator::Negate as NegateToken,
          Token,
        },
      };
  
      #[test]
      fn add() {
        use crate::op_code::Arithmetic::Add;

        let     op    = OpCode::Arithmetic(Add);
        let     token = Token::Operator(NegateToken);
        let mut chunk = Chunk::new();
  
        let value = "5".to_string();
        let num   = vec![Token::Literal(Number(value, 0))];
  
        expression(&op, &token, &mut chunk, &mut num.iter());
    
        assert_eq!(chunk.codes[0], OpCode::Literal(5.0));
        assert_eq!(chunk.codes[1], OpCode::Arithmetic(Negate));
        assert_eq!(chunk.codes[2], op);
      }
  
      #[test]
      fn divide() {
        use crate::op_code::Arithmetic::Divide;

        let     op    = OpCode::Arithmetic(Divide);
        let     token = Token::Operator(NegateToken);
        let mut chunk = Chunk::new();
  
        let value = "5".to_string();
        let num   = vec![Token::Literal(Number(value, 0))];
  
        expression(&op, &token, &mut chunk, &mut num.iter());
    
        assert_eq!(chunk.codes[0], OpCode::Literal(5.0));
        assert_eq!(chunk.codes[1], OpCode::Arithmetic(Negate));
        assert_eq!(chunk.codes[2], op);
      }
  
      #[test]
      fn multiply() {
        use crate::op_code::Arithmetic::Multiply;

        let     token = Token::Operator(NegateToken);
        let     op    = OpCode::Arithmetic(Multiply);
        let mut chunk = Chunk::new();
  
        let value = "5".to_string();
        let num   = vec![Token::Literal(Number(value, 0))];
  
        expression(&op, &token, &mut chunk, &mut num.iter());
    
        assert_eq!(chunk.codes[0], OpCode::Literal(5.0));
        assert_eq!(chunk.codes[1], OpCode::Arithmetic(Negate));
        assert_eq!(chunk.codes[2], op);
      }
    }
  }

  mod lines {
    mod without_negation {
      use crate::{
        compiler::{
          Chunk,
          arithmetic::expression,
        },
        op_code::OpCode,
        token::{
          Literal::Number,
          Token,
        },
      };
  
      #[test]
      fn add() {
        let value = "5".to_string();
        let token = Token::Literal(Number(value, 1));
  
        let mut chunk = Chunk::new();

        use crate::op_code::Arithmetic::Add;

        expression(&OpCode::Arithmetic(Add), &token, &mut chunk, &mut vec![].iter());

        assert_eq!(chunk.lines[0], 1);
        assert_eq!(chunk.lines[1], 1);
      }
  
      #[test]
      fn divide() {
        let value = "5".to_string();
        let token = Token::Literal(Number(value, 1));

        let mut chunk = Chunk::new();

        use crate::op_code::Arithmetic::Divide;

        expression(&OpCode::Arithmetic(Divide), &token, &mut chunk, &mut vec![].iter());

        assert_eq!(chunk.lines[0], 1);
        assert_eq!(chunk.lines[1], 1);
      }
  
      #[test]
      fn multiply() {
        let value = "5".to_string();
        let token = Token::Literal(Number(value, 1));

        let mut chunk = Chunk::new();

        use crate::op_code::Arithmetic::Multiply;
  
        expression(&OpCode::Arithmetic(Multiply), &token, &mut chunk, &mut vec![].iter());
    
        assert_eq!(chunk.lines[0], 1);
        assert_eq!(chunk.lines[1], 1);
      }
    }
  
    mod with_negation {
      use crate::{
        compiler::{
          Chunk,
          arithmetic::expression,
        },
        op_code::OpCode,
        token::{
          Literal::Number,
          Operator::Negate as NegateToken,
          Token,
        },
      };
  
      #[test]
      fn add() {
        let     token = Token::Operator(NegateToken);
        let mut chunk = Chunk::new();

        let value = "5".to_string();
        let num   = vec![Token::Literal(Number(value, 1))];

        use crate::op_code::Arithmetic::Add;

        expression(&OpCode::Arithmetic(Add), &token, &mut chunk, &mut num.iter());

        assert_eq!(chunk.lines[0], 1);
        assert_eq!(chunk.lines[1], 1);
        assert_eq!(chunk.lines[2], 1);
      }
  
      #[test]
      fn divide() {
        let     token = Token::Operator(NegateToken);
        let mut chunk = Chunk::new();

        let value = "5".to_string();
        let num   = vec![Token::Literal(Number(value, 1))];

        use crate::op_code::Arithmetic::Divide;

        expression(&OpCode::Arithmetic(Divide), &token, &mut chunk, &mut num.iter());

        assert_eq!(chunk.lines[0], 1);
        assert_eq!(chunk.lines[1], 1);
        assert_eq!(chunk.lines[2], 1);
      }
  
      #[test]
      fn multiply() {
        let token = Token::Operator(NegateToken);

        let mut chunk = Chunk::new();

        let value = "5".to_string();
        let num   = vec![Token::Literal(Number(value, 1))];

        use crate::op_code::Arithmetic::Multiply;

        expression(&OpCode::Arithmetic(Multiply), &token, &mut chunk, &mut num.iter());

        assert_eq!(chunk.lines[0], 1);
        assert_eq!(chunk.lines[1], 1);
        assert_eq!(chunk.lines[2], 1);
      }
    }
  }
}
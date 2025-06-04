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
  op:    &OpCode,
  token: &    Token,
  chunk: &mut Chunk,
  next:   Option<&Token>,
) {
  let mut line = 0;
  
  match token {
    Token::Operator(o) => match o {
      Operator::Negate => if let Some(v) = next { line = negate(v, chunk); }
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
        let op = OpCode::Add;
  
        let value = "5".to_string();
        let token = Token::Literal(Number(value, 0));
  
        let mut chunk = Chunk::new();
  
        expression(&op, &token, &mut chunk, None);
    
        assert_eq!(chunk.codes[0], OpCode::Literal(5.0));
        assert_eq!(chunk.codes[1], op);
      }
  
      #[test]
      fn divide() {
        let op = OpCode::Divide;
  
        let value = "5".to_string();
        let token = Token::Literal(Number(value, 0));
  
        let mut chunk = Chunk::new();
  
        expression(&op, &token, &mut chunk, None);
    
        assert_eq!(chunk.codes[0], OpCode::Literal(5.0));
        assert_eq!(chunk.codes[1], op);
      }
  
      #[test]
      fn multiply() {
        let op = OpCode::Multiply;
  
        let value = "5".to_string();
        let token = Token::Literal(Number(value, 0));
  
        let mut chunk = Chunk::new();
  
        expression(&op, &token, &mut chunk, None);
    
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
        op_code::OpCode,
        token::{
          Literal::Number,
          Operator::Negate,
          Token,
        },
      };
  
      #[test]
      fn add() {
        let     op    = OpCode::Add;
        let     token = Token::Operator(Negate);
        let mut chunk = Chunk::new();
  
        let value = "5".to_string();
        let num   = Token::Literal(Number(value, 0));
        let next  = Some(&num);
  
        expression(&op, &token, &mut chunk, next);
    
        assert_eq!(chunk.codes[0], OpCode::Literal(5.0));
        assert_eq!(chunk.codes[1], OpCode::Negate);
        assert_eq!(chunk.codes[2], op);
      }
  
      #[test]
      fn divide() {
        let     op    = OpCode::Divide;
        let     token = Token::Operator(Negate);
        let mut chunk = Chunk::new();
  
        let value = "5".to_string();
        let num   = Token::Literal(Number(value, 0));
        let next  = Some(&num);
  
        expression(&op, &token, &mut chunk, next);
    
        assert_eq!(chunk.codes[0], OpCode::Literal(5.0));
        assert_eq!(chunk.codes[1], OpCode::Negate);
        assert_eq!(chunk.codes[2], op);
      }
  
      #[test]
      fn multiply() {
        let     token = Token::Operator(Negate);
        let     op    = OpCode::Multiply;
        let mut chunk = Chunk::new();
  
        let value = "5".to_string();
        let num   = Token::Literal(Number(value, 0));
        let next  = Some(&num);
  
        expression(&op, &token, &mut chunk, next);
    
        assert_eq!(chunk.codes[0], OpCode::Literal(5.0));
        assert_eq!(chunk.codes[1], OpCode::Negate);
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
  
        expression(&OpCode::Add, &token, &mut chunk, None);
    
        assert_eq!(chunk.lines[0], 1);
        assert_eq!(chunk.lines[1], 1);
      }
  
      #[test]
      fn divide() {
        let value = "5".to_string();
        let token = Token::Literal(Number(value, 1));
  
        let mut chunk = Chunk::new();
  
        expression(&OpCode::Divide, &token, &mut chunk, None);
    
        assert_eq!(chunk.lines[0], 1);
        assert_eq!(chunk.lines[1], 1);
      }
  
      #[test]
      fn multiply() {
        let value = "5".to_string();
        let token = Token::Literal(Number(value, 1));
  
        let mut chunk = Chunk::new();
  
        expression(&OpCode::Multiply, &token, &mut chunk, None);
    
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
          Operator::Negate,
          Token,
        },
      };
  
      #[test]
      fn add() {
        let     token = Token::Operator(Negate);
        let mut chunk = Chunk::new();
  
        let value = "5".to_string();
        let num   = Token::Literal(Number(value, 1));
        let next  = Some(&num);
  
        expression(&OpCode::Add, &token, &mut chunk, next);
    
        assert_eq!(chunk.lines[0], 1);
        assert_eq!(chunk.lines[1], 1);
        assert_eq!(chunk.lines[2], 1);
      }
  
      #[test]
      fn divide() {
        let     token = Token::Operator(Negate);
        let mut chunk = Chunk::new();
  
        let value = "5".to_string();
        let num   = Token::Literal(Number(value, 1));
        let next  = Some(&num);
  
        expression(&OpCode::Divide, &token, &mut chunk, next);
    
        assert_eq!(chunk.lines[0], 1);
        assert_eq!(chunk.lines[1], 1);
        assert_eq!(chunk.lines[2], 1);
      }
  
      #[test]
      fn multiply() {
        let token = Token::Operator(Negate);

        let mut chunk = Chunk::new();
  
        let value = "5".to_string();
        let num   = Token::Literal(Number(value, 1));
        let next  = Some(&num);
  
        expression(&OpCode::Multiply, &token, &mut chunk, next);
    
        assert_eq!(chunk.lines[0], 1);
        assert_eq!(chunk.lines[1], 1);
        assert_eq!(chunk.lines[2], 1);
      }
    }
  }
}
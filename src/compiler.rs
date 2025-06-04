use std::error::Error;

use crate::{
  chunk::Chunk,
  op_code::OpCode,
  scanner::scan,
  token::{
    Keyword,
    Operator,
    Token,
  }
};

mod arithmetic;
mod negate;
mod value;

use arithmetic::expression as arithmetic;
use     negate::expression as negate;
use      value::expression as value;

pub fn execute(source: &str) -> Result<Chunk, Box<dyn Error>> {
  let mut chunk  = Chunk::new();
  let     tokens = scan(source);
  let mut tokens = tokens.iter();

  #[cfg(feature = "debug")]
  println!("TOKENS: {tokens:#?}");

  loop {
    if let Some(t) = tokens.next() {
      match t {
        Token::Literal(l) => { let _ = value(&l, &mut chunk); },
        Token::Keyword(k) => {
          match k {
            Keyword::Return(line) => chunk.append(&OpCode::Return, *line),
            _ => ()
          }
        }
        Token::Operator(o) => {
          match o {
            Operator::Add => {
              if let Some(v) = tokens.next() {
                arithmetic(&OpCode::Add, &v, &mut chunk, &mut tokens);
              }
            }
            Operator::Divide => {
              if let Some(v) = tokens.next() {
                arithmetic(&OpCode::Divide, &v, &mut chunk, &mut tokens);
              }
            }
            Operator::Multiply => {
              if let Some(v) = tokens.next() {
                arithmetic(&OpCode::Multiply, &v, &mut chunk, &mut tokens);
              }
            }
            Operator::Negate => {
              if let Some(v) = tokens.next() {
                let line = negate(v, &mut chunk);

                let code_count = chunk.codes.iter().count();

                if code_count >= 3 {
                  match chunk.codes[code_count - 3] {
                    OpCode::Literal(_) => chunk.append(&OpCode::Add, line),
                    _ => ()
                  }
                }
              }
              else { /* TODO Implement error reporting here */ }
            }
            _ => ()
          }
        }
        _ => ()
      }
    }
    else { break; }
  }

  Ok(chunk)
}

#[cfg(test)]
mod validate {
  mod literal {
    mod number {
      use crate::{
        compiler::execute as compile,
        op_code::OpCode,
      };
  
      #[test]
      fn codes() {
        match compile("4") {
          Ok(output) => { assert_eq!(output.codes[0], OpCode::Literal(4.0)); }
          Err(e)     => { panic!("Source compilation failed: {e:#?}"); }
        }
      }
  
      #[test]
      fn lines() {
        match compile("4") {
          Ok(output) => { assert_eq!(output.lines[0], 1);              }
          Err(e)     => { panic!("Source compilation failed: {e:#?}"); }
        }
      }
    }
  }

  mod keyword {
    mod rēturn {
      use crate::{
        compiler::execute as compile,
        op_code::OpCode,
      };
  
      #[test]
      fn codes() {
        match compile("return") {
          Ok(output) => { assert_eq!(output.codes[0], OpCode::Return); }
          Err(e)     => { panic!("Source compilation failed: {e:#?}"); }
        }
      }
  
      #[test]
      fn lines() {
        match compile("return") {
          Ok(output) => { assert_eq!(output.lines[0], 1);              }
          Err(e)     => { panic!("Source compilation failed: {e:#?}"); }
        }
      }
    }
  }

  mod operator {
    mod add {
      use crate::{
        compiler::execute as compile,
        op_code::OpCode,
      };
  
      #[test]
      fn codes() {
        match compile("4+5") {
          Ok(output) => {
            assert_eq!(output.codes[0], OpCode::Literal(4.0));
            assert_eq!(output.codes[1], OpCode::Literal(5.0));
            assert_eq!(output.codes[2], OpCode::Add);
          }
          Err(e) => panic!("Source compilation failed: {e:#?}")
        }
      }
  
      #[test]
      fn lines() {
        match compile("4+5") {
          Ok(output) => {
            assert_eq!(output.lines[0], 1);
            assert_eq!(output.lines[1], 1);
            assert_eq!(output.lines[2], 1);
          }
          Err(e) => panic!("Source compilation failed: {e:#?}")
        }
      }
    }

    mod divide {
      use crate::{
        compiler::execute as compile,
        op_code::OpCode,
      };
  
      #[test]
      fn codes() {
        match compile("4/5") {
          Ok(output) => {
            assert_eq!(output.codes[0], OpCode::Literal(4.0));
            assert_eq!(output.codes[1], OpCode::Literal(5.0));
            assert_eq!(output.codes[2], OpCode::Divide);
          }
          Err(e) => panic!("Source compilation failed: {e:#?}")
        }
      }
  
      #[test]
      fn lines() {
        match compile("4/5") {
          Ok(output) => {
            assert_eq!(output.lines[0], 1);
            assert_eq!(output.lines[1], 1);
            assert_eq!(output.lines[2], 1);
          }
          Err(e) => panic!("Source compilation failed: {e:#?}")
        }
      }
    }

    mod multiply {
      use crate::{
        compiler::execute as compile,
        op_code::OpCode,
      };
  
      #[test]
      fn codes() {
        match compile("4*5") {
          Ok(output) => {
            assert_eq!(output.codes[0], OpCode::Literal(4.0));
            assert_eq!(output.codes[1], OpCode::Literal(5.0));
            assert_eq!(output.codes[2], OpCode::Multiply);
          }
          Err(e) => panic!("Source compilation failed: {e:#?}")
        }
      }
  
      #[test]
      fn lines() {
        match compile("4/5") {
          Ok(output) => {
            assert_eq!(output.lines[0], 1);
            assert_eq!(output.lines[1], 1);
            assert_eq!(output.lines[2], 1);
          }
          Err(e) => panic!("Source compilation failed: {e:#?}")
        }
      }
    }

    mod negate {
      use crate::{
        compiler::execute as compile,
        op_code::OpCode,
      };
  
      #[test]
      fn codes() {
        match compile("-5") {
          Ok(output) => {
            assert_eq!(output.codes[0], OpCode::Literal(5.0));
            assert_eq!(output.codes[1], OpCode::Negate);
          }
          Err(e) => panic!("Source compilation failed: {e:#?}")
        }
      }
  
      #[test]
      fn lines() {
        match compile("-5") {
          Ok(output) => {
            assert_eq!(output.lines[0], 1);
            assert_eq!(output.lines[1], 1);
          }
          Err(e) => panic!("Source compilation failed: {e:#?}")
        }
      }
    }

    mod subtract {
      use crate::{
        compiler::execute as compile,
        op_code::OpCode,
      };
  
      #[test]
      fn codes() {
        match compile("4-5") {
          Ok(output) => {
            assert_eq!(output.codes[0], OpCode::Literal(4.0));
            assert_eq!(output.codes[1], OpCode::Literal(5.0));
            assert_eq!(output.codes[2], OpCode::Negate);
            assert_eq!(output.codes[3], OpCode::Add);
          }
          Err(e) => panic!("Source compilation failed: {e:#?}")
        }
      }
  
      #[test]
      fn lines() {
        match compile("4-5") {
          Ok(output) => {
            assert_eq!(output.lines[0], 1);
            assert_eq!(output.lines[1], 1);
            assert_eq!(output.lines[2], 1);
            assert_eq!(output.lines[3], 1);
          }
          Err(e) => panic!("Source compilation failed: {e:#?}")
        }
      }
    }
  }
}
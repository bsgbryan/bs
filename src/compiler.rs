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
          use crate::op_code::ControlFlow::Return;

          match k {
            Keyword::Return => chunk.append(&OpCode::ControlFlow(Return)),
            _ => ()
          }
        }
        Token::Operator(o) => {
          match o {
            Operator::Add => {
              if let Some(v) = tokens.next() {
                use crate::op_code::Arithmetic::Add;

                arithmetic(&OpCode::Arithmetic(Add), &v, &mut chunk, &mut tokens);
              }
            }
            Operator::Divide => {
              if let Some(v) = tokens.next() {
                use crate::op_code::Arithmetic::Divide;

                arithmetic(&OpCode::Arithmetic(Divide), &v, &mut chunk, &mut tokens);
              }
            }
            Operator::Multiply => {
              if let Some(v) = tokens.next() {
                use crate::op_code::Arithmetic::Multiply;

                arithmetic(&OpCode::Arithmetic(Multiply), &v, &mut chunk, &mut tokens);
              }
            }
            Operator::Negate => {
              if let Some(v) = tokens.next() {
                let _ = negate(v, &mut chunk);

                let code_count = chunk.codes.iter().count();

                if code_count >= 3 {
                  use crate::op_code::Arithmetic::Add;

                  match chunk.codes[code_count - 3] {
                    OpCode::Literal(_) => chunk.append(&OpCode::Arithmetic(Add)),
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
        use crate::value::Value::Number;

        match compile("4") {
          Ok(output) => { assert_eq!(output.codes[0], OpCode::Literal(Number(4.0))); }
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
        use crate::op_code::ControlFlow::Return;

        match compile("return") {
          Ok(output) => { assert_eq!(output.codes[0], OpCode::ControlFlow(Return)); }
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
            use crate::value::Value::Number;

            assert_eq!(output.codes[0], OpCode::Literal(Number(4.0)));
            assert_eq!(output.codes[1], OpCode::Literal(Number(5.0)));

            use crate::op_code::Arithmetic::Add;

            assert_eq!(output.codes[2], OpCode::Arithmetic(Add));
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
            use crate::value::Value::Number;

            assert_eq!(output.codes[0], OpCode::Literal(Number(4.0)));
            assert_eq!(output.codes[1], OpCode::Literal(Number(5.0)));

            use crate::op_code::Arithmetic::Divide;

            assert_eq!(output.codes[2], OpCode::Arithmetic(Divide));
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
            use crate::value::Value::Number;

            assert_eq!(output.codes[0], OpCode::Literal(Number(4.0)));
            assert_eq!(output.codes[1], OpCode::Literal(Number(5.0)));

            use crate::op_code::Arithmetic::Multiply;

            assert_eq!(output.codes[2], OpCode::Arithmetic(Multiply));
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
            use crate::value::Value::Number;

            assert_eq!(output.codes[0], OpCode::Literal(Number(5.0)));

            use crate::op_code::Arithmetic::Negate;

            assert_eq!(output.codes[1], OpCode::Arithmetic(Negate));
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
            use crate::value::Value::Number;

            assert_eq!(output.codes[0], OpCode::Literal(Number(4.0)));
            assert_eq!(output.codes[1], OpCode::Literal(Number(5.0)));

            use crate::op_code::Arithmetic::{
              Add,
              Negate,
            };

            assert_eq!(output.codes[2], OpCode::Arithmetic(Negate));
            assert_eq!(output.codes[3], OpCode::Arithmetic(Add));
          }
          Err(e) => panic!("Source compilation failed: {e:#?}")
        }
      }
    }
  }
}
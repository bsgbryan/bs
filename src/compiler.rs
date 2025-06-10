use std::{
	error::Error,
	slice::Iter,
};

use crate::{
  chunk::Chunk,
  op_code::OpCode,
  scanner::scan,
  token::{
    Keyword,
    Operator,
    Token,
  },
  tokens::Tokens
};

mod arithmetic;
mod keyword;
mod negate;
mod value;

use arithmetic::expression as arithmetic;
use     negate::expression as negate;
use      value::expression as value;

pub fn execute(source: &str) -> Result<Chunk, Box<dyn Error>> {
  let mut chunk  = Chunk::new();
  let     tokens = scan(source);
  let mut iter   = tokens.iter();

  #[cfg(feature = "trace")]
  println!("{tokens}");

  let mut item = 0;

  loop {
    if let Some(t) = iter.next() {
     	let (line, column) = tokens.meta(item);

    	item += advance(t, &mut chunk, &mut iter, &tokens, *line, *column);
    }
    else { break; }
  }

  Ok(chunk)
}

fn advance(
	token:	&		 Token,
	chunk:	&mut Chunk,
	iter: 	&mut Iter<Token>,
	tokens: & 	 Tokens,
	line:		 		 u64,
	column: 		 u64,
) -> u64 {
	let mut processed = 0;

  match token {
    Token::Literal(l) => {
      value(&l, chunk);

      if let Some(p) = tokens.peek(column + 1) {
	     	match p {
		      Token::Operator(_) => {
						if let Some(t) = iter.next() {
							processed += 1;
							processed += advance(t, chunk, iter, tokens, line, column + processed);
						}
					}
					_ => ()
	      }
      }
    },
    Token::Keyword(k) => {
      use crate::op_code::ControlFlow::Return;

      match k {
        Keyword::Return => chunk.append(OpCode::ControlFlow(Return)),
        _ => ()
      }
    }
    Token::Operator(o) => {
	    match o {
        Operator::Add => {
          if let Some(v) = iter.next() {
        		processed += 1;

            use crate::op_code::Arithmetic::Add;

            arithmetic(OpCode::Arithmetic(Add), &v, chunk, iter, line, column + processed);
          }
        }
        Operator::Divide => {
          if let Some(v) = iter.next() {
	          processed += 1;

            use crate::op_code::Arithmetic::Divide;

            arithmetic(OpCode::Arithmetic(Divide), &v, chunk, iter, line, column + processed);
          }
        }
        Operator::Multiply => {
          if let Some(v) = iter.next() {
           	processed += 1;

            use crate::op_code::Arithmetic::Multiply;

            arithmetic(OpCode::Arithmetic(Multiply), &v, chunk, iter, line, column + processed);
          }
        }
        Operator::Negate => {
          if let Some(v) = iter.next() {
	          processed += 1;

            let _ = negate(v, chunk, line, column + processed);

            let code_count = chunk.codes.iter().count();

            if code_count >= 3 {
              use crate::op_code::Arithmetic::Add;

              match chunk.codes[code_count - 3] {
                OpCode::Literal(_) => chunk.append(OpCode::Arithmetic(Add)),
                _ => ()
              }
            }
          }
          else { /* TODO Implement error reporting here */ }
        }
        _ => ()
      }
    }
    Token::Util(u) => {
     	use crate::token::Util::Print;

      match u {
       	Print => {
	        use crate::precedence;

					loop {
						if let Some(p) = tokens.peek(column + 1) 									 &&
						 	 precedence::for_token(p) < precedence::for_token(token) &&
							 let Some(t) = iter.next()
						{
							processed += 1;
		          processed += advance(t, chunk, iter, tokens, line, column + 1);
						}
						else { break }
					}

					use crate::op_code::Util::Print;

					chunk.append(OpCode::Util(Print));
        }
      }
    }
    _ => ()
  }

  return processed;
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

use std::fmt::Debug;

use crate::{
  chunk::Chunk,
  op_code::OpCode,
  value::Value,
};

pub fn chunk(chunk: &Chunk, id: &str) {
  println!("CHUNK: {id}");

  for c in chunk.codes.iter() { instruction(&c); }
}

pub fn instruction(op_code: &OpCode) {
  use crate::op_code::{
    Arithmetic::{
      Add,
      Divide,
      Multiply,
      Negate,
      Subtract,
    },
    ControlFlow::Return,
  };

  match op_code {
    OpCode::Literal(l)     => { single_value("OP_LITERAL", l) }
    OpCode::ControlFlow(c) => {
      match c { Return => no_value("OP_RETURN") }
    }
    OpCode::Arithmetic(a)  => {
      match a {
        Negate   => no_value("OP_NEGATE"  ),
        Add      => no_value("OP_ADD"     ),
        Divide   => no_value("OP_DIVIDE"  ),
        Multiply => no_value("OP_MULTIPLY"),
        Subtract => no_value("OP_SUBTRACT"),
      }
    }
  }
}

fn no_value(name: &str) {
  println!("{name}");
}

fn single_value(name: &str, value: &Value) {
  println!("{name}({value})");
}

pub fn stack<T>(stack: &Vec<T>) where T: Debug {
  print!("🥞::[");

  let top = stack.iter().count();

  let mut i = 0;

  for s in stack {
    if i < top {
      if i < top {
        if i > 0 { print!(","); }

        print!("{s:?}");
      }
    }
    else { break }
    i += 1;
  }
  println!("]");
}

pub fn stack_push(value: &Value) {
	println!("🥞 + {value}");
}

pub fn stack_pop(value: &Value) {
  println!("🥞 - {value}");
}

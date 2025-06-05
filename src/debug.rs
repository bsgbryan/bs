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
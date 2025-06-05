use crate::{
  chunk::Chunk,
  op_code::OpCode,
  value::{
    Value,
    // ValuePool,
  },
};

pub fn chunk(chunk: &Chunk, id: &str) {
  println!("CHUNK: {id}");

  let mut i = 0;

  for c in chunk.codes.iter() {
    instruction(&c, i, chunk.lines[i as usize], i > 0 && (chunk.lines[i as usize] == chunk.lines[(i - 1) as usize]));

    i += 1;
  }
}

pub fn instruction(op_code: &OpCode, index: u64, line: u64, same_line: bool) {
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

  if same_line { print!("{:>9} | {index}=",         ""); }
  else         { print!("{:>7}{line:0>4}:{index}=", ""); }

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

// fn multi_value(name: &str, values: &ValuePool) {
//   println!("{name}({values:?})");
// }
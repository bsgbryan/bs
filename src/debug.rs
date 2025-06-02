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
  if same_line { print!("{:>9} | {index}=",         ""); }
  else         { print!("{:>7}{line:0>4}:{index}=", ""); }

  match op_code {
    // OpCode::Constant(v) =>  multi_value("OP_CONSTANT", v),
    OpCode::Literal(l)  => single_value("OP_LITERAL",  l),
    OpCode::Return      =>     no_value("OP_RETURN"     ),
    // --- Arithmetic --- /
    // Unary
    OpCode::Negate => no_value("OP_NEGATE"),
    // Binary
    OpCode::Add      => no_value("OP_ADD"     ),
    OpCode::Divide   => no_value("OP_DIVIDE"  ),
    OpCode::Multiply => no_value("OP_MULTIPLY"),
    OpCode::Subtract => no_value("OP_SUBTRACT"),
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
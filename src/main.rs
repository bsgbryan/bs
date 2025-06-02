use bs::{
  chunk::Chunk,
  op_code::OpCode,
  // value::ValuePool,
  vm::VM,
};

fn main() {
  let mut vm = VM::new();

  let mut chunk = Chunk::new();

  // let mut pool = ValuePool::new();
  // pool.set("Pie", 3.14159);
  // pool.set("Answer", 42.0);

  // chunk.append(OpCode::Constant(pool), 1);
  
  chunk.append(OpCode::Literal(1.2), 1);
  chunk.append(OpCode::Literal(3.4), 1);
  chunk.append(OpCode::Add, 1);
  
  chunk.append(OpCode::Literal(5.6), 1);
  chunk.append(OpCode::Divide, 1);

  chunk.append(OpCode::Negate, 1);
  chunk.append(OpCode::Return, 1);

  let _ = vm.interpret(&chunk);

  vm.free();
}
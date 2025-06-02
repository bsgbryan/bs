use crate::{
  op_code::OpCode,
  value::{
    Value,
    ValuePool,
  },
};

#[derive(Debug)]
pub struct Chunk {
  pub codes:     Vec<OpCode>,
  pub constants: ValuePool,
  pub lines:     Vec<u64>,
}

impl Chunk {
  pub fn new() -> Self {
    Chunk {
      codes:     vec![],
      constants: ValuePool::new(),
      lines:     vec![],
    }
  }

  pub fn define(&mut self, name: &str, value: Value) -> u64 {
    self.constants.set(name, value);

    self.constants.count() - 1
  }

  pub fn append(&mut self, code: OpCode, line: u64) {
    self.codes.push(code);
    self.lines.push(line);
  }

  pub fn free(&mut self) {
    self.codes.clear();
    self.constants.clear();
    self.lines.clear();
  }
}
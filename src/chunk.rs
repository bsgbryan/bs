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
}

impl Chunk {
  pub fn new() -> Self {
    Chunk {
      codes:     vec![],
      constants: ValuePool::new(),
    }
  }

  pub fn define(&mut self, name: &str, value: Value) -> u64 {
    self.constants.set(name, value);

    self.constants.count() - 1
  }

  pub fn append(&mut self, code: &OpCode) {
    self.codes.push(code.clone());
  }

  pub fn free(&mut self) {
    self.codes.clear();
    self.constants.clear();
  }
}
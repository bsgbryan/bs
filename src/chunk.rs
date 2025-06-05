use crate::op_code::OpCode;

#[derive(Debug)]
pub struct Chunk {
  pub codes: Vec<OpCode>,
}

impl Chunk {
  pub fn new() -> Self {
    Chunk {
      codes: vec![],
    }
  }

  pub fn append(&mut self, code: &OpCode) {
    self.codes.push(code.clone());
  }

  pub fn free(&mut self) {
    self.codes.clear();
  }
}
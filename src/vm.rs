use std::error::Error;

use crate::{
  chunk::Chunk,
  op_code::OpCode,
  value::Value,
};

fn debug_stack(stack: [Value; u8::MAX as usize], top: u8) {
  print!("::[");
  let mut i = 0;
  for s in stack {
    if i < top {
      if i < top {
        if i > 0 { print!(","); }

        print!("{s}");
      }
    }
    else { break }
    i += 1;
  }
  println!("]");
}

fn debug_push(value: Value, stack: [Value; u8::MAX as usize], top: u8) {
  print!("🥞 + {value}");
  debug_stack(stack, top);
}

fn debug_pop(value: Value, stack: [Value; u8::MAX as usize], top: u8) {
  print!("🥞 - {value}");
  debug_stack(stack, top);
}

pub struct VM {
  chunk: Option<Chunk>,
  ip:    Option<OpCode>,
  stack:    [Value; u8::MAX as usize],
  stack_top: u8,
}

impl VM {
  pub fn new() -> Self {
    Self {
      chunk: None,
      ip:    None,
      stack:    [0.0; u8::MAX as usize],
      stack_top: 0,
    }
  }

  pub fn free(&mut self) {
    self.chunk = None;
    self.ip    = None;
    self.stack =    [0.0; u8::MAX as usize];
    self.stack_top = 0;
  }

  pub fn interpret(&mut self, chunk: &Chunk) -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "debug")] {
      debug_stack(self.stack, self.stack_top);
      crate::debug::chunk(chunk, "example");
    }

    self.run(&chunk)
  }

  fn run(&mut self, chunk: &Chunk) -> Result<(), Box<dyn Error>> {
    for c in chunk.codes.iter() {
      match c {
        // OpCode::Constant(c) => for con in c.iterable() { self.push(*con); },
        OpCode::Literal(l)  => self.push(*l),
        OpCode::Return => {
          self.pop();

          return Ok(())
        },
        // --- Arithmetic --- //
        // Unary
        OpCode::Negate => {
          let value = -self.pop();

          self.push(value);
        },
        // Binary
        OpCode::Add => {
          let rhs = self.pop();
          let lhs = self.pop();

          self.push(lhs + rhs);
        },
        OpCode::Divide => {
          let rhs = self.pop();
          let lhs = self.pop();

          self.push(lhs / rhs);
        },
        OpCode::Multiply => {
          let rhs = self.pop();
          let lhs = self.pop();

          self.push(lhs * rhs);
        },
        OpCode::Subtract => {
          let rhs = self.pop();
          let lhs = self.pop();

          self.push(lhs - rhs);
        },
      }
    }

    Ok(())
  }

  pub fn pop(&mut self) -> Value {
    let new_top = self.stack_top - 1;
    let out = self.stack[new_top as usize];

    self.stack[new_top as usize] = 0.0;
    self.stack_top = new_top;

    #[cfg(feature = "debug")]
    debug_pop(out, self.stack, self.stack_top);

    out
  }

  pub fn push(&mut self, value: Value) {
    self.stack[self.stack_top as usize] = value.clone();
    self.stack_top = self.stack_top + 1;

    #[cfg(feature = "debug")]
    debug_push(value, self.stack, self.stack_top);
  }
}
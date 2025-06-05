#[cfg(feature = "debug")]
use std::fmt::Debug;

use crate::{
  chunk::Chunk,
  value::Value,
};

#[cfg(feature = "debug")]
fn debug_stack<T>(stack: &Vec<T>) where T: Debug {
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

#[cfg(feature = "debug")]
fn debug_push(value: &Value) {
  println!("🥞 + {value:?}");
}

#[cfg(feature = "debug")]
fn debug_pop(value: &Value) {
  println!("🥞 - {value:?}");
}

struct Stack<T> {
  entries: Vec<T>,
}

impl<T> Default for Stack<T> {
  fn default() -> Self {
    Self { entries: Vec::with_capacity(256) }
  }
}

impl Stack<Value> {
  pub fn push(&mut self, value: Value) {
    #[cfg(feature = "debug")] {
      debug_stack(&self.entries);
      debug_push(&value);
    }

    self.entries.push(value);

    #[cfg(feature = "debug")] {
      debug_stack(&self.entries);
      println!();
    }
  }

  pub fn pop(&mut self) -> Option<Value> {
    #[cfg(feature = "debug")]
    debug_stack(&self.entries);

    let out = self.entries.pop();

    #[cfg(feature = "debug")] {
      if let Some(ref o) = out {
        debug_pop(&o);
        debug_stack(&self.entries);
        println!();
      }
      else { eprintln!("Attempted to pop, but nothing on stack") }
    }

    out
  }
}

fn run(chunk: &Chunk) {
  use crate::op_code::{
    Arithmetic::{
      Add,
      Divide,
      Multiply,
      Negate,
      Subtract,
    },
    ControlFlow::Return,
    OpCode,
  };

  let mut stack = Stack::default();

  for c in chunk.codes.iter() {
    match c {
      OpCode::Literal(l) => {
        match l {
          Value::Bool(b)   => stack.push(Value::Bool(*b)),
          Value::Number(n) => stack.push(Value::Number(*n)),
          Value::String(s) => stack.push(Value::String(s.clone())),
        }
      }
      OpCode::ControlFlow(c) => {
        match c { Return => { stack.pop(); } }
      }
      OpCode::Arithmetic(a) => {
        match a {
          Negate => {
            match stack.pop() {
              Some(Value::Bool(b))   => eprintln!("{b} is not a valid number; cannot Negate"),
              Some(Value::Number(n)) => stack.push(Value::Number(-n)),
              Some(Value::String(s)) => eprintln!("{s} is not a valid number; cannot Negate"),

              None => eprintln!("Stack empty; cannot Negate")
            }
          }
          Add => {
            if let Some(Value::Number(rhs)) = stack.pop() &&
               let Some(Value::Number(lhs)) = stack.pop()
            { stack.push(Value::Number(lhs + rhs)) }
            else {
              eprintln!("Stack values are valid numbers; cannot Add")
            }
          }
          Divide => {
            if let Some(Value::Number(rhs)) = stack.pop() &&
               let Some(Value::Number(lhs)) = stack.pop()
            { stack.push(Value::Number(lhs / rhs)) }
            else {
              eprintln!("Stack values are valid numbers; cannot Divide")
            }
          }
          Multiply => {
            if let Some(Value::Number(rhs)) = stack.pop() &&
               let Some(Value::Number(lhs)) = stack.pop()
            { stack.push(Value::Number(lhs * rhs)) }
            else {
              eprintln!("Stack values are valid numbers; cannot Multiply")
            }
          }
          Subtract => {
            if let Some(Value::Number(rhs)) = stack.pop() &&
               let Some(Value::Number(lhs)) = stack.pop()
            { stack.push(Value::Number(lhs - rhs)) }
            else {
              eprintln!("Stack values are valid numbers; cannot Subtract")
            }
          }
        }
      }
    }
  }
}

pub fn interpret(source: &str) {
  if let Ok(chunk) = crate::compiler::execute(source) {
    #[cfg(feature = "debug")]
    println!("{chunk:#?}");

    run(&chunk);
  }
}
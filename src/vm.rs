use crate::{
  chunk::Chunk,
  value::Value,
};

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
    #[cfg(feature = "trace")] {
      use crate::trace::{
      stack,
      stack_push,
      };

      stack(&self.entries);
      stack_push(&value);
    }

    self.entries.push(value);

    #[cfg(feature = "trace")] {
    	use crate::trace::stack;
      stack(&self.entries);
      println!();
    }
  }

  pub fn pop(&mut self) -> Option<Value> {
    #[cfg(feature = "trace")] {
    	use crate::trace::stack;

    	stack(&self.entries);
    }

    let out = self.entries.pop();

    #[cfg(feature = "trace")] {
      if let Some(ref o) = out {
	      use crate::trace::{
					stack,
					stack_pop,
				};

        stack_pop(&o);
        stack(&self.entries);
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
	    OpCode::Arithmetic(a) => {
	     	#[cfg(feature = "trace")]
	     	println!("🤖::{a}");

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
			OpCode::ControlFlow(c) => {
	      #[cfg(feature = "trace")]
	     	println!("🤖::{c}");

        match c { Return => { stack.pop(); } }
      }
    	OpCode::Literal(l) => {
      	#[cfg(feature = "trace")]
      	println!("🤖::literal({l})");

        match l {
          Value::Bool(b)   => stack.push(Value::Bool(*b)),
          Value::Number(n) => stack.push(Value::Number(*n)),
          Value::String(s) => stack.push(Value::String(s.clone())),
        }
      }
      OpCode::Util(u) => {
      	use crate::op_code::Util::Print;

      	match u {
       		Print => {
         		if let Some(value) = stack.pop() { println!("{value}"); 			 }
            else 														 { panic!("Nothing to print"); }
	        }
       	}
      }
    }
  }
}

pub fn interpret(source: &str) {
  if let Ok(chunk) = crate::compiler::execute(source) {
    #[cfg(feature = "trace")]
    println!("{chunk}");

    run(&chunk);
  }
}

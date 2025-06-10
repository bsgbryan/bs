use std::fmt::Display;

use crate::op_code::OpCode;

pub struct Chunk {
  pub codes: Vec<OpCode>,
}

impl Chunk {
  pub fn new() -> Self {
    Chunk {
      codes: vec![],
    }
  }

  pub fn append(&mut self, code: OpCode) {
    self.codes.push(code);
  }

  pub fn free(&mut self) {
    self.codes.clear();
  }
}

impl Display for Chunk {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = writeln!(f, "Chunk:");

		for c in self.codes.iter() {
			use crate::op_code::OpCode::{
				Arithmetic,
				ControlFlow,
				Literal,
				Util,
			};

			match c {
				Arithmetic(a) => {
					use crate::op_code::Arithmetic::{
						Add,
						Divide,
						Multiply,
						Negate,
						Subtract,
					};

					match a {
						Add 	 	 =>	{ let _ = writeln!(f, "\tAdd"); 	 	 }
						Divide 	 =>	{ let _ = writeln!(f, "\tDivide"); 	 }
						Multiply => { let _ = writeln!(f, "\tMultiply"); }
						Negate   =>	{ let _ = writeln!(f, "\tNegate"); 	 }
						Subtract => { let _ = writeln!(f, "\tSubstract"); }
					}
				}
				ControlFlow(c) => {
					use crate::op_code::ControlFlow::Return;

					match c {
						Return => { let _ = writeln!(f, "\tReturn"); }
					}
				}
				Literal(l) => {
					use crate::value::Value::{
						Bool,
						Number,
						String,
					};

					match l {
						Bool(b) 	=> { let _ = writeln!(f, "\tLiteral: {}", 		b); }
						Number(n) => { let _ = writeln!(f, "\tLiteral: {}", 		n); }
						String(s) => { let _ = writeln!(f, "\tLiteral: \"{}\"", s); }
					}
				}
				Util(u) => {
					use crate::op_code::Util::Print;

					match u {
						Print => { let _ = writeln!(f, "\tprint"); }
					}
				}
			}
		}

		Ok(())
	}
}

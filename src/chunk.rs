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
				Command,
				ControlFlow,
        Equality,
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
				Command(c) => {
					use crate::op_code::Command::Invert;

					match c {
						Invert => { let _ = writeln!(f, "\tInvert"); }
					}
				}
				ControlFlow(c) => {
					use crate::op_code::ControlFlow::Return;

					match c {
						Return => { let _ = writeln!(f, "\tReturn"); }
					}
				}
        Equality(e) => {
          use crate::op_code::Equality::{
            Equal,
            Greater,
            GreaterOrEqual,
            Less,
            LessOrEqual,
            NotEqual,
          };

          match e {
            Equal          => { let _ = writeln!(f, "\tEqual"         ); }
            Greater        => { let _ = writeln!(f, "\tGreater"       ); }
            GreaterOrEqual => { let _ = writeln!(f, "\tGreaterOrEqual"); }
            Less           => { let _ = writeln!(f, "\tLess"          ); }
            LessOrEqual    => { let _ = writeln!(f, "\tLessOrEqual"   ); }
            NotEqual       => { let _ = writeln!(f, "\tNotEqual"      ); }
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

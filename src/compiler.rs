use crate::error::RuntimeError;

use crate::lexer::Tokenizer;
use crate::tokens::{
  Token,
  TokenKind::{
    EndOfLine,
    IntegerLiteral,
    NonInterpolatedStringLiteral,
    Plus,
  },
};

enum OpCode {
  Add,
  Divide,
  Multiply,
  Subtract,
}

enum Precedence {
  None,
  Assignment,
  Or,
  And,
  Equality,
  Comparison,
  Term,
  Factor,
  Unary,
  Call,
  Primary,
}

pub struct Compiler<'a> {
  tokenizer: Tokenizer<'a>,
  tokens: Vec<Token>,
  // context: HashMap<TokenKind, ParseContext>,
}

impl<'a> Compiler<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      tokens: Vec::new(),
      tokenizer: Tokenizer::new(input),
    }
  }

  pub fn compile(&mut self) {
    while let Ok(token) = self.tokenizer.iterate() {
      match token.kind {
        NonInterpolatedStringLiteral { value: s } => {
          println!("Non-interpolated string \"{}\"", s);
          // self.tokens.push(r);
        }
        IntegerLiteral { value: i  } => {
          println!("Integer literal {}", i);
          // self.tokens.push(r);
        }
        Plus => {
          println!("Plus");
          // self.tokens.push(r);
        }
        EndOfLine => {
          println!("MCP: End of Line");
          break;
        }
        _ => println!("Something cool"),
      }
    }
  }

  pub fn process(&mut self, _result: Result<Token, RuntimeError>) {
    
  }

  fn parse(&mut self, _precedence: &Precedence) {

  }

  fn expression(&mut self) {

  }

  fn number(&mut self) {

  }

  fn binary(&mut self) {
    let kind = self.tokens.last();
    // let rule = getRule(kind);
    // self.parse(Precedence::from(0));

    if let Some(k) = kind {
      match k.kind {
        Plus => self.emitByte(OpCode::Add),
        _ => println!("OHAI"),
      }
    }

  }

  fn emitByte(&mut self, _oc: OpCode) {

  }
}

use std::slice::Iter;

use crate::token::{
  Literal::String as BSString,
  Token,
};

pub fn process<'a>(
  lexeme: &    str,
  line:        u64,
  iter:   &mut Iter<'a, &str>,
) -> Option<Token> {
  if lexeme == "\"" {
    let mut value = String::new();

    loop {
      if let Some(&v) = iter.next() {
        if v != "\"" { value += v; }
        else         { break;      }
      }
    }

    Some(Token::Literal(BSString(value, line)))
  }
  else { None }
}

#[cfg(test)]
mod validate {
  use crate::token::Token;

  #[test]
  fn output() {
    use crate::token::Literal::String;

    let     value = vec!["\"", "Hello,", " ", "World!", "\""];
    let mut iter  = value.iter();
    
    if let Some(lexeme) = iter.next() {
      if let Some(output) = super::process(lexeme, 1, &mut iter) {
        let value    = "Hello, World!".to_string();
        let greeting = String(value, 1);

        assert_eq!(output, Token::Literal(greeting));
      }
      else { panic!("Expected output, not None") }
    }
  }
}
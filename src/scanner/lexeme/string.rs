use std::slice::Iter;

use crate::token::{
  Literal::String as BSString,
  Token,
};

pub fn process<'a>(
  lexeme: &    str,
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

    Some(Token::Literal(BSString(value)))
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
      if let Some(output) = super::process(lexeme, &mut iter) {
        let value    = "Hello, World!".to_string();
        let greeting = String(value);

        assert_eq!(output, Token::Literal(greeting));
      }
      else { panic!("Expected output, not None") }
    }
  }
}
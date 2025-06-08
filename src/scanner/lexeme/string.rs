use std::slice::Iter;

use crate::token::{
  Literal::String as BSString,
  Token,
};

pub fn process<'a>(
  lexeme: &    str,
  iter:   &mut Iter<'a, (usize, &str)>,
  column: u64,
) -> Option<(u64, Token)> {
  if lexeme == "\"" {
    let mut value = String::new();

    loop {
      if let Some((_, v)) = iter.next() {
        if *v != "\"" { value += v; }
        else         	{ break;      }
      }
    }

    Some((column, Token::Literal(BSString(value))))
  }
  else { None }
}

#[cfg(test)]
mod validate {
  use crate::token::Token;

  #[test]
  fn output() {
    use crate::token::Literal::String;

    let     value = vec![(0, "\""), (1, "Hello"), (6, ","), (7, " "), (8, "World"), (13, "!"), (14, "\"")];
    let mut iter  = value.iter();

    if let Some((column, lexeme)) = iter.next() {
      if let Some(output) = super::process(lexeme, &mut iter, *column as u64) {
        let value    = "Hello, World!".to_string();
        let greeting = String(value);

        assert_eq!(output, (0, Token::Literal(greeting)));
      }
      else { panic!("Expected output, not None") }
    }
  }
}

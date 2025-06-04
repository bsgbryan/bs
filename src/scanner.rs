use unicode_segmentation::UnicodeSegmentation;

use crate::token::Token;

mod lexeme;

use lexeme::process;

pub fn scan(line: &str) -> Vec<Token> {
  let     split   = line.split_word_bounds().collect::<Vec<&str>>();
  let mut lexemes = split.iter();
  let mut tokens  = vec![];
  let mut line    = 1;

  loop {
    if let Some(&lexeme) = lexemes.next() {
      if lexeme == "\n" { line += 1; continue; }
      if let Some(first) = lexeme.chars().nth(0) {
        if first.is_whitespace() { continue; }
      }

      for token in process(lexeme, line, &mut lexemes) {
        tokens.push(token);
      }
    }
    else { break; }
  }

  tokens
}

#[cfg(test)]
mod validate {
  mod equality {
    use crate::{
      scanner::scan,
      token::{
        Equality::Equal,
        Token,
      },
    };

    #[test]
    fn equal() {
      let tokens = scan("==");
      let mut tokens = tokens.iter();

      assert_eq!(tokens.clone().count(), 1);

      let first = tokens.next();

      match first {
        Some(t) => { assert_eq!(*t, Token::Equality(Equal(1))) }
        None    => { panic!("Expected a Token, got None")      }
      }
    }
  }

  mod operator {
    use crate::{
      scanner::scan,
      token::{
        Literal::Number,
        Operator::Assign,
        Token,
      },
    };

    #[test]
    fn assign() {
      let     tokens = scan("=4");
      let mut tokens = tokens.iter();

      assert_eq!(tokens.clone().count(), 2);

      let first  = tokens.next();
      let second = tokens.next();

      match first {
        Some(t) => { assert_eq!(*t, Token::Operator(Assign(1))) }
        None    => { panic!("Expected a Token, got None")       }
      }

      match second {
        Some(t) => {
          let value = "4".to_string();
          assert_eq!(*t, Token::Literal(Number(value, 1)));
        }
        None => { panic!("Expected a Token, got None") }
      }
    }
  }
}

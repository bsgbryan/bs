use std::slice::Iter;

use crate::token::Token;

pub fn process<'a>(
  lexeme: &    str,
  iter:   &mut Iter<'a, &str>,
) -> Vec<Token> {
  match lexeme {
    "=" => {
      if let Some(&n) = iter.next() {
        if n == "=" {
          use crate::token::Equality::Equal;

          let mut out = Vec::with_capacity(1);

          out.push(Token::Equality(Equal));

          return out
        }
        // Assignment
        else {
          use crate::token::Operator::Assign;

          let mut out = Vec::with_capacity(2);
          
          out.push(Token::Operator(Assign));
          out.append(&mut super::process(n, iter));

          return out
        }
      }
      else { return Vec::with_capacity(0) }
    }
    ">" => {
      if let Some(&n) = iter.next() {
        if n == "=" {
          use crate::token::Equality::GreaterOrEqual;

          let mut out = Vec::with_capacity(1);

          out.push(Token::Equality(GreaterOrEqual));

          return out
        }
        else {
          use crate::token::Equality::Greater;

          let mut out = Vec::with_capacity(2);

          out.push(Token::Equality(Greater));
          out.append(&mut super::process(n, iter));

          return out
        }
      }
      else { return Vec::with_capacity(0) }
    }
    "<" => {
      if let Some(&n) = iter.next() {
        if n == "=" {
          use crate::token::Equality::LessOrEqual;

          let mut out = Vec::with_capacity(1);

          out.push(Token::Equality(LessOrEqual));

          return out
        }
        else {
          use crate::token::Equality::Less;

          let mut out = Vec::with_capacity(2);

          out.push(Token::Equality(Less));
          out.append(&mut super::process(n, iter));

          return out
        }
      }
      else { return Vec::with_capacity(0) }
    }
    "!" => {
      if let Some(&n) = iter.next() {
        if n == "=" {
          use crate::token::Equality::NotEqual;

          let mut out = Vec::with_capacity(1);

          out.push(Token::Equality(NotEqual));

          return out
        }
        else {
          use crate::token::{
            Keyword::{
              False,
              True,
            },
            Operator::Invert,
          };

          let tokens = super::process(n, iter);
          
          if tokens[0] == Token::Keyword(False) ||
             tokens[0] == Token::Keyword(True)
          {
            let mut out = Vec::with_capacity(2);

            out.append(&mut super::process(n, iter));
            out.push(Token::Operator(Invert));
  
            return out
          }
          else { return Vec::with_capacity(0) }
        }
      }
      else { return Vec::with_capacity(0) }
    }
    _ => Vec::with_capacity(0)
  }
}

#[cfg(test)]
mod validate {
  use crate::token::Token;

  #[test]
  fn assign() {
    use crate::token::Operator::Assign;

    let     lexemes = vec!["=", "4"];
    let mut iter    = lexemes.iter();
    
    if let Some(token) = iter.next() {
      let tokens = super::process(token, &mut iter);

      assert_eq!(tokens.iter().count(), 2);

      if let Some(t) = tokens.iter().next() {
        assert_eq!(*t, Token::Operator(Assign));
      }
      else { panic!("Expected a Token, got None"); }
    }
  }

  #[test]
  fn equal() {
    use crate::token::Equality::Equal;

    let     lexemes = vec!["=", "="];
    let mut iter    = lexemes.iter();
    
    if let Some(token) = iter.next() {
      let tokens = super::process(token, &mut iter);

      assert_eq!(tokens.iter().count(), 1);

      if let Some(t) = tokens.iter().next() {
        assert_eq!(*t, Token::Equality(Equal));
      }
      else { panic!("Expected a Token, got None"); }
    }
  }

  #[test]
  fn greater() {
    use crate::token::{
      Equality::Greater,
      Literal::Number,
    };

    let     lexemes = vec![">", "2"];
    let mut iter    = lexemes.iter();
    
    if let Some(token) = iter.next() {
      let     tokens = super::process(token, &mut iter);
      let mut tokens = tokens.iter();

      assert_eq!(tokens.clone().count(), 2);

      let first  = tokens.next();
      let second = tokens.next();

      if let Some(t) = first {
        assert_eq!(*t, Token::Equality(Greater));
      }
      else { panic!("Expected a Token, got None"); }

      if let Some(t) = second {
        let value = "2".to_string();
        assert_eq!(*t, Token::Literal(Number(value)));
      }
      else { panic!("Expected a Token, got None"); }
    }
  }

  #[test]
  fn greater_or_equal() {
    use crate::token::Equality::GreaterOrEqual;

    let     lexemes = vec![">", "="];
    let mut iter    = lexemes.iter();
    
    if let Some(token) = iter.next() {
      let tokens = super::process(token, &mut iter);

      assert_eq!(tokens.iter().count(), 1);

      if let Some(t) = tokens.iter().next() {
        assert_eq!(*t, Token::Equality(GreaterOrEqual));
      }
      else { panic!("Expected a Token, got None"); }
    }
  }

  #[test]
  fn invert() {
    use crate::token::{
      Operator::Invert,
      Keyword::False,
    };

    let     lexemes = vec!["!", "false"];
    let mut iter    = lexemes.iter();
    
    if let Some(token) = iter.next() {
      let     tokens = super::process(token, &mut iter);
      let mut tokens = tokens.iter();

      assert_eq!(tokens.clone().count(), 2);

      let first  = tokens.next();
      let second = tokens.next();

      if let Some(t) = first {
        assert_eq!(*t, Token::Keyword(False));
      }
      else { panic!("Expected a Token, got None"); }

      if let Some(t) = second {
        assert_eq!(*t, Token::Operator(Invert));
      }
      else { panic!("Expected a Token, got None"); }
    }
  }

  #[test]
  fn less() {
    use crate::token::{
      Equality::Less,
      Literal::Number,
    };

    let     lexemes = vec!["<", "2"];
    let mut iter    = lexemes.iter();
    
    if let Some(token) = iter.next() {
      let     tokens = super::process(token, &mut iter);
      let mut tokens = tokens.iter();

      assert_eq!(tokens.clone().count(), 2);

      let first  = tokens.next();
      let second = tokens.next();

      if let Some(t) = first {
        assert_eq!(*t, Token::Equality(Less));
      }
      else { panic!("Expected a Token, got None"); }

      if let Some(t) = second {
        let value = "2".to_string();
        assert_eq!(*t, Token::Literal(Number(value)));
      }
      else { panic!("Expected a Token, got None"); }
    }
  }

  #[test]
  fn less_or_equal() {
    use crate::token::Equality::LessOrEqual;

    let     lexemes = vec!["<", "="];
    let mut iter    = lexemes.iter();
    
    if let Some(token) = iter.next() {
      let tokens = super::process(token, &mut iter);

      assert_eq!(tokens.iter().count(), 1);

      if let Some(t) = tokens.iter().next() {
        assert_eq!(*t, Token::Equality(LessOrEqual));
      }
      else { panic!("Expected a Token, got None"); }
    }
  }

  #[test]
  fn not_equal() {
    use crate::token::Equality::NotEqual;

    let     lexemes = vec!["!", "="];
    let mut iter    = lexemes.iter();
    
    if let Some(token) = iter.next() {
      let tokens = super::process(token, &mut iter);

      assert_eq!(tokens.iter().count(), 1);

      if let Some(t) = tokens.iter().next() {
        assert_eq!(*t, Token::Equality(NotEqual));
      }
      else { panic!("Expected a Token, got None"); }
    }
  }
}

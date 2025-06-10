use std::slice::Iter;

use crate::token::Token;

pub fn process<'a>(
  lexeme: &    str,
  iter:   &mut Iter<'a, (usize, &str)>,
  column: u64,
) -> Vec<(u64, Token)> {
  match lexeme {
    "=" => {
      if let Some((c, n)) = iter.next() {
        if *n == "=" {
          use crate::token::Equality::Equal;

          if let Some((_, l)) = iter.next() {
            let mut out = Vec::with_capacity(2);
  
            out.append(&mut super::process(l, iter, *c as u64));
            out.push((column, Token::Equality(Equal)));
  
            return out
          }
          else { Vec::with_capacity(0) }
        }
        // Assignment
        else {
          use crate::token::Operator::Assign;

          let mut out = Vec::with_capacity(2);

          out.push((column, Token::Operator(Assign)));
          out.append(&mut super::process(n, iter, *c as u64));

          return out
        }
      }
      else { return Vec::with_capacity(0) }
    }
    ">" => {
      if let Some((c, n)) = iter.next() {
        if *n == "=" {
          use crate::token::Equality::GreaterOrEqual;

          if let Some((_, l)) = iter.next() {
            let mut out = Vec::with_capacity(2);
  
            out.append(&mut super::process(l, iter, *c as u64));
            out.push((column, Token::Equality(GreaterOrEqual)));
  
            return out
          }
          else { Vec::with_capacity(0) }
        }
        else {
          use crate::token::Equality::Greater;

          if let Some((c, l)) = iter.next() {
            let mut out = Vec::with_capacity(2);
  
            out.append(&mut super::process(l, iter, *c as u64));
            out.push((column, Token::Equality(Greater)));
  
            return out
          }
          else { Vec::with_capacity(0) }
        }
      }
      else { return Vec::with_capacity(0) }
    }
    "<" => {
      if let Some((c, n)) = iter.next() {
        if *n == "=" {
          use crate::token::Equality::LessOrEqual;

          if let Some((_, l)) = iter.next() {
            let mut out = Vec::with_capacity(2);
  
            out.append(&mut super::process(l, iter, *c as u64));
            out.push((column, Token::Equality(LessOrEqual)));
  
            return out
          }
          else { Vec::with_capacity(0) }
        }
        else {
          use crate::token::Equality::Less;

          if let Some((c, l)) = iter.next() {
            let mut out = Vec::with_capacity(2);
  
            out.append(&mut super::process(l, iter, *c as u64));
            out.push((column, Token::Equality(Less)));
  
            return out
          }
          else { Vec::with_capacity(0) }
        }
      }
      else { return Vec::with_capacity(0) }
    }
    "!" => {
      if let Some((c, n)) = iter.next() {
        if *n == "=" {
          use crate::token::Equality::NotEqual;

          if let Some((_, l)) = iter.next() {
            let mut out = Vec::with_capacity(2);

            out.append(&mut super::process(l, iter, *c as u64));
            out.push((column, Token::Equality(NotEqual)));

            return out
          }
          else { Vec::with_capacity(0) }
        }
        else {
          use crate::token::Operator::Invert;

          let mut tokens = super::process(n, iter, *c as u64);
          let mut out 	 = Vec::with_capacity(2);

          out.append(&mut tokens);
          out.push((column, Token::Operator(Invert)));

          return out
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

    let     lexemes = vec![(0, "="), (1, "4")];
    let mut iter    = lexemes.iter();

    if let Some((column, token)) = iter.next() {
      let tokens = super::process(token, &mut iter, *column as u64);

      assert_eq!(tokens.iter().count(), 2);

      if let Some(t) = tokens.iter().next() {
        assert_eq!(*t, (0, Token::Operator(Assign)));
      }
      else { panic!("Expected a Token, got None"); }
    }
  }

  #[test]
  fn equal() {
    use crate::token::Equality::Equal;

    let     lexemes = vec![(0, "="), (1, "=")];
    let mut iter    = lexemes.iter();

    if let Some((column, token)) = iter.next() {
      let tokens = super::process(token, &mut iter, *column as u64);

      assert_eq!(tokens.iter().count(), 1);

      if let Some(t) = tokens.iter().next() {
        assert_eq!(*t, (0, Token::Equality(Equal)));
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

    let     lexemes = vec![(0, ">"), (1, "2")];
    let mut iter    = lexemes.iter();

    if let Some((column, token)) = iter.next() {
      let     tokens = super::process(token, &mut iter, *column as u64);
      let mut tokens = tokens.iter();

      assert_eq!(tokens.clone().count(), 2);

      let first  = tokens.next();
      let second = tokens.next();

      if let Some(t) = first {
        assert_eq!(*t, (0, Token::Equality(Greater)));
      }
      else { panic!("Expected a Token, got None"); }

      if let Some(t) = second {
        let value = "2".to_string();
        assert_eq!(*t, (1, Token::Literal(Number(value))));
      }
      else { panic!("Expected a Token, got None"); }
    }
  }

  #[test]
  fn greater_or_equal() {
    use crate::token::Equality::GreaterOrEqual;

    let     lexemes = vec![(0, ">"), (1, "=")];
    let mut iter    = lexemes.iter();

    if let Some((column, token)) = iter.next() {
      let tokens = super::process(token, &mut iter, *column as u64);

      assert_eq!(tokens.iter().count(), 1);

      if let Some(t) = tokens.iter().next() {
        assert_eq!(*t, (0, Token::Equality(GreaterOrEqual)));
      }
      else { panic!("Expected a Token, got None"); }
    }
  }

  #[test]
  fn invert() {
    use crate::token::{
	   	Literal::Bool,
	    Operator::Invert,
    };

    let     lexemes = vec![(0, "!"), (1, "false")];
    let mut iter    = lexemes.iter();

    if let Some((column, token)) = iter.next() {
      let     tokens = super::process(token, &mut iter, *column as u64);
      let mut tokens = tokens.iter();

      assert_eq!(tokens.clone().count(), 2);

      let first  = tokens.next();
      let second = tokens.next();

      if let Some(t) = first {
        assert_eq!(*t, (1, Token::Literal(Bool(false))));
      }
      else { panic!("Expected a Token, got None"); }

      if let Some(t) = second {
        assert_eq!(*t, (0, Token::Operator(Invert)));
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

    let     lexemes = vec![(0, "<"), (1, "2")];
    let mut iter    = lexemes.iter();

    if let Some((column, token)) = iter.next() {
      let     tokens = super::process(token, &mut iter, *column as u64);
      let mut tokens = tokens.iter();

      assert_eq!(tokens.clone().count(), 2);

      let first  = tokens.next();
      let second = tokens.next();

      if let Some(t) = first {
        assert_eq!(*t, (0, Token::Equality(Less)));
      }
      else { panic!("Expected a Token, got None"); }

      if let Some(t) = second {
        let value = "2".to_string();
        assert_eq!(*t, (1, Token::Literal(Number(value))));
      }
      else { panic!("Expected a Token, got None"); }
    }
  }

  #[test]
  fn less_or_equal() {
    use crate::token::Equality::LessOrEqual;

    let     lexemes = vec![(0, "<"), (1, "=")];
    let mut iter    = lexemes.iter();

    if let Some((column, token)) = iter.next() {
      let tokens = super::process(token, &mut iter, *column as u64);

      assert_eq!(tokens.iter().count(), 1);

      if let Some(t) = tokens.iter().next() {
        assert_eq!(*t, (0, Token::Equality(LessOrEqual)));
      }
      else { panic!("Expected a Token, got None"); }
    }
  }

  #[test]
  fn not_equal() {
    use crate::token::Equality::NotEqual;

    let     lexemes = vec![(0, "!"), (1, "=")];
    let mut iter    = lexemes.iter();

    if let Some((column, token)) = iter.next() {
      let tokens = super::process(token, &mut iter, *column as u64);

      assert_eq!(tokens.iter().count(), 1);

      if let Some(t) = tokens.iter().next() {
        assert_eq!(*t, (0, Token::Equality(NotEqual)));
      }
      else { panic!("Expected a Token, got None"); }
    }
  }
}

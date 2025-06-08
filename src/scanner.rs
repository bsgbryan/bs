use unicode_segmentation::UnicodeSegmentation;

use crate::tokens::Tokens;

mod lexeme;

use lexeme::process;

pub fn scan(source: &str) -> Tokens {
  let mut tokens = Tokens::default();
  let mut num 	 = 1;

	for line in source.split_terminator("\n") {
	  let     lexemes = line.split_word_bound_indices().collect::<Vec<(usize, &str)>>();
	  let mut lexemes = lexemes.iter();

	  loop {
	    if let Some((column, lexeme)) = lexemes.next() {
	      if let Some(first) = lexeme.chars().nth(0) {
	        if first.is_whitespace() { continue; }
	      }

	      for (c, token) in process(lexeme, &mut lexemes, *column as u64) {
					tokens.push(token, num, c);
	      }
	    }
	    else { break; }
	  }

		num += 1;
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
        Some(t) => { assert_eq!(*t, Token::Equality(Equal)) }
        None    => { panic!("Expected a Token, got None")   }
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
        Some(t) => { assert_eq!(*t, Token::Operator(Assign)) }
        None    => { panic!("Expected a Token, got None")    }
      }

      match second {
        Some(t) => {
          let value = "4".to_string();
          assert_eq!(*t, Token::Literal(Number(value)));
        }
        None => { panic!("Expected a Token, got None") }
      }
    }
  }
}

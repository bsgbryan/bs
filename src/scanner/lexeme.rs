use std::slice::Iter;

use crate::token::{
  Keyword,
  Literal::{
	  Bool,
	  self,
  },
  Operator,
  Scope,
  Error,
  Token,
  Util,
};

mod assign_equality_invert;
use assign_equality_invert::process as assign_equality_invert;

mod string;
use string::process as string;

pub fn process<'a>(
  lexeme: &    str,
  iter:   &mut Iter<'a, (usize, &str)>,
  column: u64,
) -> Vec<(u64, Token)> {
  let mut out = Vec::with_capacity(1);

  match lexeme {
    "const"  => { out.push((column, Token::Keyword(Keyword::Const))); },
    "else"   => { out.push((column, Token::Keyword(Keyword::Else))); },
    "fun"    => { out.push((column, Token::Keyword(Keyword::Fun))); },
    "if"     => { out.push((column, Token::Keyword(Keyword::If))); },
    "loop"   => { out.push((column, Token::Keyword(Keyword::Loop))); },
    "print"  => { out.push((column, Token::Util(Util::Print))); },
    "return" => { out.push((column, Token::Keyword(Keyword::Return))); },
    "var"    => { out.push((column, Token::Keyword(Keyword::Var))); },
    "while"  => { out.push((column, Token::Keyword(Keyword::While))); },

    "false" => { out.push((column, Token::Literal(Bool(false)))); },
    "true"  => { out.push((column, Token::Literal(Bool(true)))); },

    "+" => { out.push((column, Token::Operator(Operator::Add))); },
    "/" => { out.push((column, Token::Operator(Operator::Divide))); },
    "*" => { out.push((column, Token::Operator(Operator::Multiply))); },
    "-" => { out.push((column, Token::Operator(Operator::Negate))); },
    "!" => { out.push((column, Token::Operator(Operator::Invert))); },

    ":"    => { out.push((column, Token::Scope(Scope::Colon))); },
    ","    => { out.push((column, Token::Scope(Scope::Comma))); },
    "."    => { out.push((column, Token::Scope(Scope::Dot))); },
    "]"    => { out.push((column, Token::Scope(Scope::CloseBracet))); },
    ")"    => { out.push((column, Token::Scope(Scope::CloseParen))); },
    "["    => { out.push((column, Token::Scope(Scope::OpenBracet))); },
    "("    => { out.push((column, Token::Scope(Scope::OpenParen))); },
    "self" => { out.push((column, Token::Scope(Scope::SELF))); },

    "#" => (/* TODO Implement Markdown processing */),
     _  => {
      let tokens = assign_equality_invert(lexeme, iter, column);
      if tokens.iter().count() > 0 { return tokens; }

      if let Some(token) = string(lexeme, iter, column) {
        let mut out = Vec::with_capacity(1);

        out.push(token);

        return out
      }
      else if lexeme == "`" {
        let mut value = String::new();

        loop {
          if let Some((_, v)) = iter.next() {
            if *v != "`" { value += v; }
            else         { break;    	 }
          }
        }

        out.push((column, Token::Literal(Literal::InterpolatedString(value))));
      }
      else if let Some(first) = lexeme.chars().nth(0) {
        if first.is_numeric() {
          out.push((column, Token::Literal(Literal::Number(lexeme.to_string()))));
        }
        else if first.is_alphabetic() {
          out.push((column, Token::Literal(Literal::Identifier(lexeme.to_string()))));
        }
        else {
          // TODO Update this when I figure out better error handling/reporting
          out.push((column, Token::Error(Error::Invalid(lexeme.to_string()))));
        }
      }
    },
  }

  out
}

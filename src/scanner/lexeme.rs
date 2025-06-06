use std::slice::Iter;

use crate::token::{
  Keyword,
  Literal,
  Operator,
  Scope,
  Error,
  Token,
};

mod assign_equality_invert;
use assign_equality_invert::process as equality_assign_invert;

mod string;
use string::process as string;

pub fn process<'a>(
  lexeme: &    str,
  iter:   &mut Iter<'a, &str>,
) -> Vec<Token> {
  let mut out = Vec::with_capacity(1);

  match lexeme {
    "and"    => { out.push(Token::Keyword(Keyword::And)); },
    "const"  => { out.push(Token::Keyword(Keyword::Const)); },
    "else"   => { out.push(Token::Keyword(Keyword::Else)); },
    "false"  => { out.push(Token::Keyword(Keyword::False)); },
    "fun"    => { out.push(Token::Keyword(Keyword::Fun)); },
    "if"     => { out.push(Token::Keyword(Keyword::If)); },
    "loop"   => { out.push(Token::Keyword(Keyword::Loop)); },
    "or"     => { out.push(Token::Keyword(Keyword::Or)); },
    "print"  => { out.push(Token::Keyword(Keyword::Print)); },
    "return" => { out.push(Token::Keyword(Keyword::Return)); },
    "true"   => { out.push(Token::Keyword(Keyword::True)); },
    "var"    => { out.push(Token::Keyword(Keyword::Var)); },
    "while"  => { out.push(Token::Keyword(Keyword::While)); },

    "+" => { out.push(Token::Operator(Operator::Add)); },
    "/" => { out.push(Token::Operator(Operator::Divide)); },
    "*" => { out.push(Token::Operator(Operator::Multiply)); },
    "-" => { out.push(Token::Operator(Operator::Negate)); },
    "!" => { out.push(Token::Operator(Operator::Invert)); },

    ":"    => { out.push(Token::Scope(Scope::Colon)); },
    ","    => { out.push(Token::Scope(Scope::Comma)); },
    "."    => { out.push(Token::Scope(Scope::Dot)); },
    "]"    => { out.push(Token::Scope(Scope::CloseBracet)); },
    ")"    => { out.push(Token::Scope(Scope::CloseParen)); },
    "["    => { out.push(Token::Scope(Scope::OpenBracet)); },
    "("    => { out.push(Token::Scope(Scope::OpenParen)); },
    "self" => { out.push(Token::Scope(Scope::SELF)); },

    "#" => (/* TODO Implement Markdown processing */),
     _  => {
      let tokens = equality_assign_invert(lexeme, iter);
      if tokens.iter().count() > 0 { return tokens; }

      if let Some(token) = string(lexeme, iter) {
        let mut out = Vec::with_capacity(1);

        out.push(token);

        return out
      }
      else if lexeme == "`" {
        let mut value = String::new();

        loop {
          if let Some(&v) = iter.next() {
            if v != "`" { value += v; }
            else        { break;      }
          }
        }

        out.push(Token::Literal(Literal::InterpolatedString(value)));
      }
      else if let Some(first) = lexeme.chars().nth(0) {
        if first.is_numeric() {
          out.push(Token::Literal(Literal::Number(lexeme.to_string())));
        }
        else if first.is_alphabetic() {
          out.push(Token::Literal(Literal::Identifier(lexeme.to_string())));
        }
        else {
          // TODO Update this when I figure out better error handling/reporting
          out.push(Token::Error(Error::Invalid(lexeme.to_string())));
        }
      }
    },
  }

  out
}

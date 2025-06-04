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
  line:        u64,
  iter:   &mut Iter<'a, &str>,
) -> Vec<Token> {
  let mut out = Vec::with_capacity(1);

  match lexeme {
    "and"    => { out.push(Token::Keyword(Keyword::And(line))); },
    "const"  => { out.push(Token::Keyword(Keyword::Const(line))); },
    "else"   => { out.push(Token::Keyword(Keyword::Else(line))); },
    "false"  => { out.push(Token::Keyword(Keyword::False)); },
    "fun"    => { out.push(Token::Keyword(Keyword::Fun(line))); },
    "if"     => { out.push(Token::Keyword(Keyword::If(line))); },
    "loop"   => { out.push(Token::Keyword(Keyword::Loop(line))); },
    "or"     => { out.push(Token::Keyword(Keyword::Or(line))); },
    "print"  => { out.push(Token::Keyword(Keyword::Print(line))); },
    "return" => { out.push(Token::Keyword(Keyword::Return(line))); },
    "true"   => { out.push(Token::Keyword(Keyword::True)); },
    "var"    => { out.push(Token::Keyword(Keyword::Var(line))); },
    "while"  => { out.push(Token::Keyword(Keyword::While(line))); },

    "+" => { out.push(Token::Operator(Operator::Add)); },
    "/" => { out.push(Token::Operator(Operator::Divide)); },
    "*" => { out.push(Token::Operator(Operator::Multiply)); },
    "-" => { out.push(Token::Operator(Operator::Negate)); },
    "!" => { out.push(Token::Operator(Operator::Not(line))); },

    ":"    => { out.push(Token::Scope(Scope::Colon(line))); },
    ","    => { out.push(Token::Scope(Scope::Comma(line))); },
    "."    => { out.push(Token::Scope(Scope::Dot(line))); },
    "]"    => { out.push(Token::Scope(Scope::CloseBracet(line))); },
    ")"    => { out.push(Token::Scope(Scope::CloseParen(line))); },
    "["    => { out.push(Token::Scope(Scope::OpenBracet(line))); },
    "("    => { out.push(Token::Scope(Scope::OpenParen(line))); },
    "self" => { out.push(Token::Scope(Scope::SELF(line))); },

    "#" => (/* TODO Implement Markdown processing */),
     _  => {
      let tokens = equality_assign_invert(lexeme, line, iter);
      if tokens.iter().count() > 0 { return tokens; }

      if let Some(token) = string(lexeme, line, iter) {
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

        out.push(Token::Literal(Literal::InterpolatedString(value, line)));
      }
      else if let Some(first) = lexeme.chars().nth(0) {
        if first.is_numeric() {
          out.push(Token::Literal(Literal::Number(lexeme.to_string(), line)));
        }
        else if first.is_alphabetic() {
          out.push(Token::Literal(Literal::Identifier(lexeme.to_string(), line)));
        }
        else {
          out.push(Token::Error(Error::Invalid(lexeme.to_string(), line)));
        }
      }
    },
  }

  out
}

use std::str;

use crate::{
  error::{
    RuntimeError,
  },
  tokens::{
    single_char_tokens,
    TokenKind, equality_tokens, is_comment_token,
  },
};

#[derive(Debug, PartialEq)]
pub struct Token {
  pub kind: TokenKind,
  pub line: usize,
  pub column: usize,
  pub length: usize,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, RuntimeError> {
  let mut tokens = Vec::new();

  let mut line = 0;
  let mut column = 0;

  let mut chars = input.chars().peekable();
  let mut current = chars.next();

  loop {
    match current {
      Some(current) => {
        if let Some(p) = chars.peek() {
          if is_comment_token(format!("{current}{p}").as_str()) {
            column += 1;
            // TODO Need to make sure leangth includes the actual comment
            tokens.push(Token { kind: TokenKind::Comment, line, column, length: 2 });
            chars.next();
          }
          else if let Some(k) = equality_tokens().get(format!("{current}{p}").as_str()) {
            column += 1;
            tokens.push(Token { kind: *k, line, column, length: 2 });
            chars.next();
          }
          else if let Some(k) = equality_tokens().get(format!("{current}").as_str()) {
            tokens.push(Token { kind: *k, line, column, length: 1 });
          }
          else if let Some(k) = single_char_tokens().get(format!("{current}").as_str()) {
            tokens.push(Token { kind: *k, line, column, length: 1 });
          }
        }
        else if let Some(k) = single_char_tokens().get(format!("{current}").as_str()) {
          tokens.push(Token { kind: *k, line, column, length: 1 });
        }
        else {
          return Err(
            RuntimeError {
              message: format!("Unknown token '{current}' at {line}:{column}")
            }
          )
        }
      }
      None => break
    }

    if let Some(t) = tokens.last() {
      if t.kind == TokenKind::EndOfLine {
        line += 1;
      }
    }

    column += 1;

    current = chars.next();
  }

  Ok(tokens)
}

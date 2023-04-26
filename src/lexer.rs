use std::{
  iter::Peekable,
  str::{
    Chars,
    self,
  },
};

use crate::{
  error::RuntimeError,
  tokens::{
    Token,
    TokenKind,
    Keyword::*,
    equality_tokens,
    generate_numeric_token,
    is_comment_token,
    is_const_keyword,
    is_decorate_keyword,
    is_expose_keyword,
    is_f32_keyword,
    is_f64_keyword,
    is_fun_keyword,
    is_i32_keyword,
    is_i64_keyword,
    is_integer,
    is_lowercase_alphabetic_character,
    is_mut_keyword,
    is_non_interpolated_string_boundary,
    is_otherwise_keyword,
    is_return_keyword,
    is_struct_keyword,
    is_uppercase_alphabetic_character,
    is_use_keyword,
    is_when_keyword,
    single_char_tokens,
  },
};

pub struct Tokenizer<'a> {
  pub line: usize,
  pub column: usize,
  chars: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      chars: input.chars().peekable(),
      column: 0,
      line: 0,
    }
  }

  pub fn iterate(&mut self) -> Result<Token, RuntimeError> {
    let current = self.chars.next();
  
    loop {
      match current {
        Some(current) => {
          if let Some(p) = self.chars.peek() {
            if is_lowercase_alphabetic_character(current) {
              if is_fun_keyword(current, self.chars.clone()) {
                let kind = TokenKind::Keyword { value: Fun };
  
                self.column += 2;
                let _ = self.chars.advance_by(2);

                return Ok(Token { kind, line: self.line, column: self.column, length: 3 });
              }
              else if is_when_keyword(current, self.chars.clone()) {
                let kind = TokenKind::Keyword { value: When };
  
                self.column += 3;
                let _ = self.chars.advance_by(3);

                return Ok(Token { kind, line: self.line, column: self.column, length: 4 })
              }
              else if is_otherwise_keyword(current, self.chars.clone()) {
                let kind = TokenKind::Keyword { value: Otherwise };
  
                self.column += 8;
                let _ = self.chars.advance_by(8);

                return Ok(Token { kind, line: self.line, column: self.column, length: 9 });
              }
              else if is_struct_keyword(current, self.chars.clone()) {
                let kind = TokenKind::Keyword { value: Struct };
  
                self.column += 5;
                let _ = self.chars.advance_by(5);

                return Ok(Token { kind, line: self.line, column: self.column, length: 6 });
              }
              else if is_expose_keyword(current, self.chars.clone()) {
                let kind = TokenKind::Keyword { value: Expose };
  
                self.column += 5;
                let _ = self.chars.advance_by(5);

                return Ok(Token { kind, line: self.line, column: self.column, length: 6 });
              }
              else if is_use_keyword(current, self.chars.clone()) {
                let kind = TokenKind::Keyword { value: Use };
  
                self.column += 2;
                let _ = self.chars.advance_by(2);

                return Ok(Token { kind, line: self.line, column: self.column, length: 3 });
              }
              else if is_const_keyword(current, self.chars.clone()) {
                let kind = TokenKind::Keyword { value: Const };
  
                self.column += 4;
                let _ = self.chars.advance_by(4);

                return Ok(Token { kind, line: self.line, column: self.column, length: 5 });
              }
              else if is_mut_keyword(current, self.chars.clone()) {
                let kind = TokenKind::Keyword { value: Mut };
  
                self.column += 2;
                let _ = self.chars.advance_by(2);

                return Ok(Token { kind, line: self.line, column: self.column, length: 3 });
              }
              else if is_i32_keyword(current, self.chars.clone()) {
                let kind = TokenKind::Keyword { value: I32 };
  
                self.column += 2;
                let _ = self.chars.advance_by(2);

                return Ok(Token { kind, line: self.line, column: self.column, length: 3 });
              }
              else if is_i64_keyword(current, self.chars.clone()) {
                let kind = TokenKind::Keyword { value: I64 };
  
                self.column += 2;
                let _ = self.chars.advance_by(2);

                return Ok(Token { kind, line: self.line, column: self.column, length: 3 });
              }
              else if is_f32_keyword(current, self.chars.clone()) {
                let kind = TokenKind::Keyword { value: F32 };
  
                self.column += 2;
                let _ = self.chars.advance_by(2);

                return Ok(Token { kind, line: self.line, column: self.column, length: 3 });
              }
              else if is_f64_keyword(current, self.chars.clone()) {
                let kind = TokenKind::Keyword { value: F64 };
  
                self.column += 2;
                let _ = self.chars.advance_by(2);

                return Ok(Token { kind, line: self.line, column: self.column, length: 3 });
              }
              else if is_return_keyword(current, self.chars.clone()) {
                let kind = TokenKind::Keyword { value: Return };
  
                self.column += 5;
                let _ = self.chars.advance_by(5);

                return Ok(Token { kind, line: self.line, column: self.column, length: 6 });
              }
              else if is_decorate_keyword(current, self.chars.clone()) {
                let kind = TokenKind::Keyword { value: Decorate };
  
                self.column += 7;
                let _ = self.chars.advance_by(7);

                return Ok(Token { kind, line: self.line, column: self.column, length: 8 });
              }
              else {
                let start = self.column;
                let mut value = vec![];
  
                value.push(current);
  
                while let Some(&next) = self.chars.peek() {
                  if is_lowercase_alphabetic_character(next) || is_uppercase_alphabetic_character(next) {
                    self.column += 1;
  
                    value.push(next);
  
                    self.chars.next();
                  }
                  else if next == ' ' {
                    self.column += 1;
                    self.chars.next();
                    break;
                  }
                  else {
                    let message = format!("Invalid character '{next}' in label {}:{}", self.line, self.column);
                    return Err(RuntimeError {message});
                  }
                }
  
                let kind = TokenKind::Label { value: value.iter().collect::<String>() };
  
                return Ok(Token { kind, line: self.line, column: start, length: value.len() });
              }
            }
            else if is_uppercase_alphabetic_character(current) {
              let start = self.column;
              let mut value = vec![];
  
              value.push(current);
  
              while let Some(&next) = self.chars.peek() {
                if is_lowercase_alphabetic_character(next) || is_uppercase_alphabetic_character(next) {
                  self.column += 1;
  
                  value.push(next);
  
                  self.chars.next();
                }
                else if next == ' ' {
                  self.column += 1;
                  self.chars.next();
                  break;
                }
                else {
                  let message = format!("Invalid character '{next}' in type name {}:{}", self.line, self.column);
                  return Err(RuntimeError {message});
                }
              }
  
              let kind = TokenKind::Type { name: value.iter().collect::<String>() };
  
              return Ok(Token { kind, line: self.line, column: start, length: value.len() });
            }
            else if is_non_interpolated_string_boundary(format!("{current}").as_str()) {
              let start = self.column;
              let mut value = String::new();
              let mut terminated = false;
  
              // TODO Follow the pattern used in the is_type_name block starting on line 146 above
              while let Some(next) = self.chars.peek() {
                if !is_non_interpolated_string_boundary(format!("{next}").as_str()) {
                  self.column += 1;
  
                  value.push_str(format!("{next}").as_str());
  
                  self.chars.next();
                }
                else {
                  terminated = true;
                  break
                }
              }
  
              if !terminated {
                let message = format!("Unterminated string '{}' starting at {}:{}", value.clone(), self.line, start);
                return Err(RuntimeError {message});
              }

              self.column += 1;
              self.chars.next();

              let kind = TokenKind::NonInterpolatedStringLiteral { value: value.clone() };
              return Ok(Token { kind, line: self.line, column: start, length: value.len() });
            }
            else if is_integer(current.clone()) {
              let result = generate_numeric_token(self.line, self.column, &current, self.chars.clone());

              if let Ok(token) = &result {
                self.column += token.length;
                let _ = self.chars.advance_by(token.length);
              }

              return result;
            }
            else if is_comment_token(format!("{current}{p}").as_str()) {
              self.column += 2;
              let _ = self.chars.advance_by(2);
              
              // TODO Need to make sure length includes the actual comment
              return Ok(Token { kind: TokenKind::Comment, line: self.line, column: self.column, length: 2 });
            }
            else if let Some(k) = equality_tokens().get(format!("{current}{p}").as_str()) {
              self.column += 2;
              let _ = self.chars.advance_by(2);

              return Ok(Token { kind: k.clone(), line: self.line, column: self.column, length: 2 });
            }
            else if let Some(k) = equality_tokens().get(format!("{current}").as_str()) {
              self.column += 1;
              self.chars.next();

              return Ok(Token { kind: k.clone(), line: self.line, column: self.column, length: 1 });
            }
            else if let Some(k) = single_char_tokens().get(format!("{current}").as_str()) {
              self.column += 1;
              self.chars.next();

              return Ok(Token { kind: k.clone(), line: self.line, column: self.column, length: 1 });
            }
          }
          else if let Some(k) = single_char_tokens().get(format!("{current}").as_str()) {
            self.column += 1;
            self.chars.next();

            return Ok(Token { kind: k.clone(), line: self.line, column: self.column, length: 1 });
          }
          else {
            if is_integer(current.clone()) {
              let result = generate_numeric_token(self.line, self.column, &current, self.chars.clone());

              if let Ok(token) = &result {
                self.column += token.length;
                let _ = self.chars.advance_by(token.length);
              }

              return result;
            }
            else {
              let message = format!("Unknown token '{current}' at {}:{}", self.line, self.column);
              return Err(RuntimeError {message});
            }
          }
        }
        None => {
          let token = Token { kind: TokenKind::EndOfLine, line: self.line, column: self.column, length: 0 };

          self.column = 0;
          self.line += 1;

          return Ok(token);
        }
      }
    }
  }
}


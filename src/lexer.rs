use std::str;

use crate::{
  error::RuntimeError,
  tokens::{
    Token,
    TokenKind,
    Keyword::*,
    equality_tokens,
    is_comment_token,
    is_const_keyword,
    is_decorate_keyword,
    is_dot,
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
  }
};

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
          if is_lowercase_alphabetic_character(current) {
            if is_fun_keyword(current, chars.clone()) {
              let kind = TokenKind::Keyword { value: Fun };

              tokens.push(Token { kind, line, column, length: 3 });
              column += 2;
              let _ = chars.advance_by(2);
            }
            else if is_when_keyword(current, chars.clone()) {
              let kind = TokenKind::Keyword { value: When };

              tokens.push(Token { kind, line, column, length: 4 });
              column += 3;
              let _ = chars.advance_by(3);
            }
            else if is_otherwise_keyword(current, chars.clone()) {
              let kind = TokenKind::Keyword { value: Otherwise };

              tokens.push(Token { kind, line, column, length: 9 });
              column += 8;
              let _ = chars.advance_by(8);
            }
            else if is_struct_keyword(current, chars.clone()) {
              let kind = TokenKind::Keyword { value: Struct };

              tokens.push(Token { kind, line, column, length: 6 });
              column += 5;
              let _ = chars.advance_by(5);
            }
            else if is_expose_keyword(current, chars.clone()) {
              let kind = TokenKind::Keyword { value: Expose };

              tokens.push(Token { kind, line, column, length: 6 });
              column += 5;
              let _ = chars.advance_by(5);
            }
            else if is_use_keyword(current, chars.clone()) {
              let kind = TokenKind::Keyword { value: Use };

              tokens.push(Token { kind, line, column, length: 3 });
              column += 2;
              let _ = chars.advance_by(2);
            }
            else if is_const_keyword(current, chars.clone()) {
              let kind = TokenKind::Keyword { value: Const };

              tokens.push(Token { kind, line, column, length: 5 });
              column += 4;
              let _ = chars.advance_by(4);
            }
            else if is_mut_keyword(current, chars.clone()) {
              let kind = TokenKind::Keyword { value: Mut };

              tokens.push(Token { kind, line, column, length: 3 });
              column += 2;
              let _ = chars.advance_by(2);
            }
            else if is_i32_keyword(current, chars.clone()) {
              let kind = TokenKind::Keyword { value: I32 };

              tokens.push(Token { kind, line, column, length: 3 });
              column += 2;
              let _ = chars.advance_by(2);
            }
            else if is_i64_keyword(current, chars.clone()) {
              let kind = TokenKind::Keyword { value: I64 };

              tokens.push(Token { kind, line, column, length: 3 });
              column += 2;
              let _ = chars.advance_by(2);
            }
            else if is_f32_keyword(current, chars.clone()) {
              let kind = TokenKind::Keyword { value: F32 };

              tokens.push(Token { kind, line, column, length: 3 });
              column += 2;
              let _ = chars.advance_by(2);
            }
            else if is_f64_keyword(current, chars.clone()) {
              let kind = TokenKind::Keyword { value: F64 };

              tokens.push(Token { kind, line, column, length: 3 });
              column += 2;
              let _ = chars.advance_by(2);
            }
            else if is_return_keyword(current, chars.clone()) {
              let kind = TokenKind::Keyword { value: Return };

              tokens.push(Token { kind, line, column, length: 6 });
              column += 5;
              let _ = chars.advance_by(5);
            }
            else if is_decorate_keyword(current, chars.clone()) {
              let kind = TokenKind::Keyword { value: Decorate };

              tokens.push(Token { kind, line, column, length: 8 });
              column += 7;
              let _ = chars.advance_by(7);
            }
            else {
              let start = column;
              let mut value = vec![];

              value.push(current);

              while let Some(&next) = chars.peek() {
                if is_lowercase_alphabetic_character(next) || is_uppercase_alphabetic_character(next) {
                  column += 1;

                  value.push(next);

                  chars.next();
                }
                else if next == ' ' {
                  column += 1;
                  chars.next();
                  break;
                }
                else {
                  return Err(
                    RuntimeError {
                      message: format!("Invalid character '{next}' in label {line}:{column}")
                    }
                  )
                }
              }

              let kind = TokenKind::Label { value: value.iter().collect::<String>() };

              tokens.push(Token { kind, line, column: start, length: value.len() });
            }
          }
          else if is_uppercase_alphabetic_character(current) {
            let start = column;
            let mut value = vec![];

            value.push(current);

            while let Some(&next) = chars.peek() {
              if is_lowercase_alphabetic_character(next) || is_uppercase_alphabetic_character(next) {
                column += 1;

                value.push(next);

                chars.next();
              }
              else if next == ' ' {
                column += 1;
                chars.next();
                break;
              }
              else {
                return Err(
                  RuntimeError {
                    message: format!("Invalid character '{next}' in type name {line}:{column}")
                  }
                )
              }
            }

            let kind = TokenKind::Type { name: value.iter().collect::<String>() };

            tokens.push(Token { kind, line, column: start, length: value.len() });
          }
          else if is_non_interpolated_string_boundary(format!("{current}").as_str()) {
            let start = column;
            let mut value = String::new();
            let mut terminated = false;

            // TODO Follow the pattern used in the is_type_name block starting on line 146 above
            while let Some(next) = chars.peek() {
              if !is_non_interpolated_string_boundary(format!("{next}").as_str()) {
                column += 1;

                value.push_str(format!("{next}").as_str());

                chars.next();
              }
              else {
                terminated = true;
                break
              }
            }

            if !terminated {
              return Err(
                RuntimeError {
                  message: format!("Unterminated string '{value}' starting at {line}:{start}")
                }
              )
            }

            let kind = TokenKind::NonInterpolatedStringLiteral { value };

            tokens.push(Token { kind, line, column: start, length: column - start });
            // TODO Double check if this is needed - if seems like it's not
            column += 1;
            chars.next();
          }
          else if is_integer(current.clone()) {
            let start = column;
            let mut value = String::new();
            let mut dotted = false;
            let mut valid = true;

            value.push_str(format!("{current}").as_str());

            // TODO Follow the pattern used in the is_type_name block starting on line 146 above
            while let Some(next) = chars.peek() {
              if is_integer(next.clone()) {
                column += 1;
                value.push_str(format!("{next}").as_str());
                chars.next();
              }
              else if is_dot(next.clone()) {
                if !dotted {
                  column += 1;
                  value.push_str(format!("{next}").as_str());
                  chars.next();
                  dotted = true;
                }
                else {
                  value.push_str(format!("{next}").as_str());

                  return Err(
                    RuntimeError {
                      message: format!("'{value}' starting at {line}:{start} is not a valid floating point number")
                    }
                  )
                }
              }
              else if next.clone() == ' ' {
                chars.next();
                break
              }
              else {
                valid = false;
                column += 1;

                value.push_str(format!("{next}").as_str());

                break
              }
            }

            if valid {
              column += 1;

              if dotted {
                if let Ok(v) = value.parse::<f32>() {
                  let kind = TokenKind::FloatLiteral { value: v };

                  tokens.push(Token { kind, line, column: start, length: column - start });
                }
                else {
                  return Err(
                    RuntimeError {
                      message: format!("'{value}' starting at {line}:{start} is not a valid f64")
                    }
                  )
                }
              }
              else {
                if let Ok(v) = value.parse::<usize>() {
                  let kind = TokenKind::IntegerLiteral { value: v };

                  tokens.push(Token { kind, line, column: start, length: column - start });
                }
                else {
                  return Err(
                    RuntimeError {
                      message: format!("'{value}' starting at {line}:{start} is not a valid integer")
                    }
                  )
                }
              }
            }
            else {
              return Err(
                RuntimeError {
                  message: format!("'{value}' starting at {line}:{start} is not a valid numeric value")
                }
              )
            }
          }
          else if is_comment_token(format!("{current}{p}").as_str()) {
            // TODO Need to make sure leangth includes the actual comment
            tokens.push(Token { kind: TokenKind::Comment, line, column, length: 2 });
            column += 1;
            chars.next();
          }
          else if let Some(k) = equality_tokens().get(format!("{current}{p}").as_str()) {
            tokens.push(Token { kind: k.clone(), line, column, length: 2 });
            column += 1;
            chars.next();
          }
          else if let Some(k) = equality_tokens().get(format!("{current}").as_str()) {
            tokens.push(Token { kind: k.clone(), line, column, length: 1 });
          }
          else if let Some(k) = single_char_tokens().get(format!("{current}").as_str()) {
            tokens.push(Token { kind: k.clone(), line, column, length: 1 });
          }
        }
        else if let Some(k) = single_char_tokens().get(format!("{current}").as_str()) {
          tokens.push(Token { kind: k.clone(), line, column, length: 1 });
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

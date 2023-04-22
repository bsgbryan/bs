use std::{
  collections::HashMap,
  iter::Peekable,
  str::Chars,
};

const NEW_LINE: &str = "\n";
const LEFT_PAREN: &str = "(";
const RIGHT_PAREN: &str = ")";
const LEFT_BRACE: &str = "{";
const RIGHT_BRACE: &str = "}";
const SEMICOLON: &str = ";";
const COMMA: &str = ",";
const DOT: &str = ".";
const MINUS: &str = "-";
const PLUS: &str = "+";
const SLASH: &str = "/";
const STAR: &str = "*";
const SINGLE_QUOTE: &str = "'";
const BACK_QUOTE: &str = "`";
const BANG: &str = "!";
const EQUAL: &str = "=";
const LESS: &str = "<";
const GREATER: &str = ">";

#[derive(Debug, PartialEq)]
pub struct Token {
  pub kind: TokenKind,
  pub line: usize,
  pub column: usize,
  pub length: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
  Fun,
  Otherwise,
  When,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
  EndOfLine,
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  Semicolon,
  Comma,
  Dot,
  Minus,
  Plus,
  Slash,
  Star,
  SingleQuote,
  BackQuote,
  Bang,
  Equal,
  Less,
  Greater,
  BangEqual,
  EqualEqual,
  LessEqual,
  GreaterEqual,
  Comment,
  NonInterpolatedStringLiteral { value: String },
  IntegerLiteral { value: usize },
  FloatLiteral { value: f32 },
  Keyword { value: Keyword }
}

pub fn single_char_tokens() -> HashMap<&'static str, TokenKind> {
  use TokenKind::*;

  let mut simple_token_map: HashMap<&str, TokenKind> = HashMap::new();

  simple_token_map.insert(NEW_LINE, EndOfLine);
  simple_token_map.insert(LEFT_PAREN, LeftParen);
  simple_token_map.insert(RIGHT_PAREN, RightParen);
  simple_token_map.insert(LEFT_BRACE, LeftBrace);
  simple_token_map.insert(RIGHT_BRACE, RightBrace);
  simple_token_map.insert(SEMICOLON, Semicolon);
  simple_token_map.insert(COMMA, Comma);
  simple_token_map.insert(DOT, Dot);
  simple_token_map.insert(MINUS, Minus);
  simple_token_map.insert(PLUS, Plus);
  simple_token_map.insert(SLASH, Slash);
  simple_token_map.insert(STAR, Star);
  simple_token_map.insert(SINGLE_QUOTE, SingleQuote);
  simple_token_map.insert(BACK_QUOTE, BackQuote);

  simple_token_map
}

const BANG_EQUAL: &str = "!=";
const EQUAL_EQUAL: &str = "==";
const LESS_EQUAL: &str = "<=";
const GREATER_EQUAL: &str = ">=";

pub fn equality_tokens() -> HashMap<&'static str, TokenKind> {
  use TokenKind::*;

  let mut equality_token_map: HashMap<&str, TokenKind> = HashMap::new();

  equality_token_map.insert(BANG, Bang);
  equality_token_map.insert(EQUAL, Equal);
  equality_token_map.insert(LESS, Less);
  equality_token_map.insert(GREATER, Greater);
  equality_token_map.insert(BANG_EQUAL, BangEqual);
  equality_token_map.insert(EQUAL_EQUAL, EqualEqual);
  equality_token_map.insert(LESS_EQUAL, LessEqual);
  equality_token_map.insert(GREATER_EQUAL, GreaterEqual);

  equality_token_map
}

pub fn is_comment_token(t: &str) -> bool {
  t == "//"
}

pub fn is_non_interpolated_string_boundary(s: &str) -> bool {
  s == "\""
}

pub fn is_integer(i: char) -> bool {
  i >= '0' && i <= '9'
}

pub fn is_dot(d: char) -> bool {
  d == '.'
}

pub fn is_lowercase_alphabetic_character(c: char) -> bool {
  c >= 'a' && c <= 'z'
}

pub fn is_fun_keyword(k: char, mut iter: Peekable<Chars<'_>>) -> bool {
  k == 'f' && iter.next() == Some('u') && iter.next() == Some('n')
}

pub fn is_when_keyword(k: char, mut iter: Peekable<Chars<'_>>) -> bool {
  k == 'w' && iter.next() == Some('h') && iter.next() == Some('e') && iter.next() == Some('n')
}

pub fn is_otherwise_keyword(k: char, mut iter: Peekable<Chars<'_>>) -> bool {
  k           ==      'o'  &&
  iter.next() == Some('t') &&
  iter.next() == Some('h') &&
  iter.next() == Some('e') &&
  iter.next() == Some('r') &&
  iter.next() == Some('w') &&
  iter.next() == Some('i') &&
  iter.next() == Some('s') &&
  iter.next() == Some('e')
}

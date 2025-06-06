use crate::token::Token;

pub const NONE:       u8 =  0;
pub const ASSIGNMENT: u8 =  1; // =
pub const OR:         u8 =  2; // or
pub const AND:        u8 =  3; // and
pub const EQUALITY:   u8 =  4; // == !=
pub const COMPARISON: u8 =  5; // < > <= >=
pub const TERM:       u8 =  6; // + -
pub const FACTOR:     u8 =  7; // * /
pub const UNARY:      u8 =  8; // ! -
pub const CALL:       u8 =  9; // . ()
pub const PRIMARY:    u8 = 10;

pub fn for_token(token: &Token) -> u8 {
  match token {
    Token::Equality(e) => {
      use crate::token::Equality::{
        Equal,
        Greater,
        GreaterOrEqual,
        Less,
        LessOrEqual,
        NotEqual,
      };

      match e {
        Equal    => EQUALITY,
        NotEqual => EQUALITY,

        Greater        => COMPARISON,
        GreaterOrEqual => COMPARISON,
        Less           => COMPARISON,
        LessOrEqual    => COMPARISON,
      }
    }
    Token::Error(e) => {
      use crate::token::Error::Invalid;

      match e { Invalid(_) => NONE }
    }
    Token::Keyword(k) => {
      use crate::token::Keyword::{
        And,
        Const,
        Else,
        False,
        Fun,
        If,
        Loop,
        Or,
        Print,
        Return,
        True,
        Var,
        While,
      };

      match k {
        And    => NONE,
        Const  => NONE,
        Else   => NONE,
        False  => NONE,
        Fun    => NONE,
        If     => NONE,
        Loop   => NONE,
        Or     => NONE,
        Print  => NONE,
        Return => NONE,
        True   => NONE,
        Var    => NONE,
        While  => NONE,
      }
    }
    Token::Literal(l) => {
      use crate::token::Literal::{
        Identifier,
        InterpolatedString,
        Number,
        String,
      };

      match l {
        Identifier(_)         => NONE,
        InterpolatedString(_) => NONE,
        Number(_)             => NONE,
        String(_)             => NONE,
      }
    }
    Token::Operator(o) => {
      use crate::token::Operator::{
        Add,
        Assign,
        Divide,
        Invert,
        Multiply,
        Negate,
      };

      match o {
        Add      => TERM,
        Assign   => ASSIGNMENT,
        Divide   => FACTOR,
        Invert   => UNARY,
        Multiply => FACTOR,
        Negate   => UNARY,
      }
    }
    Token::Scope(s) => {
      use crate::token::Scope::{
        CloseBracet,
        CloseParen,
        Colon,
        Comma,
        Dot,
        OpenBracet,
        OpenParen,
        SELF,
      };

      match s {
        CloseBracet => NONE,
        CloseParen  => NONE,
        Colon       => NONE,
        Comma       => NONE,
        Dot         => NONE,
        OpenBracet  => NONE,
        OpenParen   => NONE,
        SELF        => NONE,
      }
    }
  }
}

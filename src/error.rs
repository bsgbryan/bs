#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SourcePos {
  pub line: usize,
  pub column: usize,
}

#[derive(Debug, PartialEq)]
pub struct RuntimeError {
  pub message: String,
}

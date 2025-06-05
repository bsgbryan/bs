#[derive(Clone, Debug, PartialEq)]
pub enum Value {
  Number(f64),
  Bool(bool),
  String(String),
}
use std::{
  collections::{
    hash_map::Values,
    HashMap,
  },
  fmt::Debug,
};

pub type Value = f64;

#[derive(Clone)]
pub struct ValuePool {
  values: HashMap<String, Value>,
}

impl Debug for ValuePool {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut i = 0;

    for (k, v) in self.values.iter() {
      if i == 0 { write!(f,  "{}:{}", k, v)?; }
      else      { write!(f, ",{}:{}", k, v)?; }
      i += 1;
    }

    Ok(())
  }
}

impl ValuePool {
  pub fn new() -> Self {
    Self { values: HashMap::new() }
  }

  pub fn set(&mut self, id: &str, value: Value) {
    self.values.insert(id.to_string(), value);
  }

  pub fn clear(&mut self) {
    self.values.clear();
  }

  pub fn count(&self) -> u64 {
    self.values.iter().count() as u64
  }

  pub fn iterable(&self) -> Values<String, Value> {
    self.values.values()
  }
}
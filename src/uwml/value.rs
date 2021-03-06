use super::prelude::*;

#[derive(Debug)]
pub enum Value {
  String(String),
  Int(i64),
  Element(Element),
  Array(Vec<Value>),
  Block(Option<Vec<Node>>),
}

impl From<String> for Value {
  fn from(s: String) -> Self { Value::String(s) }
}

impl From<i64> for Value {
  fn from(i: i64) -> Self { Value::Int(i) }
}

impl From<Element> for Value {
  fn from(elem: Element) -> Self { Value::Element(elem) }
}

impl From<Vec<Value>> for Value {
  fn from(array: Vec<Value>) -> Self { Value::Array(array) }
}

impl From<Option<Vec<Node>>> for Value {
  fn from(block: Option<Vec<Node>>) -> Self { Value::Block(block) }
}
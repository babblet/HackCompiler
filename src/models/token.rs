use std::collections::HashMap;
use std::ffi::OsString;
use crate::grammar::LexicalElement;

pub struct Token {
  pub element: LexicalElement,
  pub data: String,
  pub keyword_key: Option<String>
}
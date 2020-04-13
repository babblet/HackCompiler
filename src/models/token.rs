use std::collections::HashMap;
use std::ffi::OsString;
use crate::enums::grammar::LexicalElement;

struct Token {
  type: LexicalElement,
  data: String,
  keyword_key: Option<String>
}
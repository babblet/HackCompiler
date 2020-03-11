use super::grammar;
use std::ffi::OsString;

pub struct Tokenizer {}

impl Tokenizer {
  pub fn new(input_lines: &Vec<OsString>) -> Result<Tokenizer, String> {

    let input_iterator = input_lines.iter();

    Ok(Tokenizer {})
  }
  
  pub fn advance(&mut self) {}

  pub fn has_more_tokens() -> bool {
    return true;
  }

  pub fn token_type() -> Option<grammar::LexicalElement> {
    return None;
  }
  pub fn key_word() -> Option<grammar::LexicalElementKeyword> {
    return None;
  }
  pub fn symbol() -> char {
    return ' ';
  }
  pub fn identifier() -> String {
    return "".to_string();
  }
  pub fn int_val() -> i16 {
    return 0;
  }
  pub fn string_val() -> String {
    return "".to_string();
  }
}

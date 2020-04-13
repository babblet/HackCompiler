use crate::enums::grammar::LexicalElement;
use crate::enums::grammar::SYMBOLS;
use crate::models::Token;
use std::slice::Iter;
use std::ffi::OsString;
use std::str::Chars;

pub struct Tokenizer {
  input: Chars,
}

impl Tokenizer {
  pub fn new(input_lines: &Vec<OsString>) -> Tokenizer {
    let input = String::new();

    //Implement comment removal later
    for _line in input_lines {
      match _line.into_string(self) {
        Ok(line) => input = [input, line].concat(),
        _ => continue,
      };
    }
    Tokenizer {
      input: input.chars(),
    }
  }
  
  pub fn advance(self) -> Option<Token> {
    let current_build = String::new();

    loop {
      match self.input.next() {
        Some(c) => {
          if is_whitespace(c) {
            if current_build.len() > 0 {
              break create_identifier_token(current_build); 
            } else {
              continue;
            }
          }
          current_build.push(c);
          
          match create_token(current_build) {
            Some(token) => break token,
            None => continue;
          };
        },
        None => {
          if current_build.len() > 0 {
            break create_identifier_token(current_build);
          } else {
            break None;
          }
        },
      }
    }
  }

  fn remove_comment(line: String) {

  }

  fn is_whitespace(c: char) {
    if c == ' '  ||
       c == '\t' {
      return true;
    } else {
      return false
    }
  }

  fn create_identifier_token(input: String) -> Token {
    Token {
      type: LexicalElement::IDENTIFIER,
      data: input,
      keyword_key: None,
    }
  }

  fn create_token(input: String) -> Option<Token> {
    match find_token_type(input) {
      Some(token_type) => {
        Token {
          type: token_type,
          data: input,
          keyword_key: None,
        }
      },
      _ => {
        match find_keyword(input) {
          Some(keyword_key) {
            Token {
              type: LexicalElement::Keyword,
              data: input,
              keyword_key: keyword_key,
            }
          },
          _ => None,
        }
      },
    }
  }

  fn find_token_type(input: String) -> Option<LexicalElement> {
    if is_symbol(input) {
      return Some(LexicalElement::Symbol);
    } else if is_integer_constant(input) {
      return Some(LexicalElement::IntegerConstant);
    } else if is_string_constant(input) {
      return Some(LexicalElement::StringConstant);
    } 
  }

  fn is_symbol(input: String) -> bool {
    let mut is_symbol = false;
    for symbol in __SYMBOLS__ {
      if input == symbol {
        is_symbol = true
      } else {
        continue
      }
    }
  }

  fn is_integer_constant(input: String) -> bool {
    if 
  }

  fn is_string_constant(input: String) -> bool {

  }

  fn find_keyword(input: String) -> Option<String>{

  }
}

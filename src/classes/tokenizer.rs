use crate::grammar::LexicalElement;
use crate::grammar::SYMBOLS;
use crate::models::token::Token;
use std::slice::Iter;
use std::ffi::OsString;
use std::str::Chars;

pub struct Tokenizer {
  input: Vec<char>,
  input_index: usize,
}

impl Tokenizer {
  pub fn new(input_lines: &Vec<OsString>) -> Tokenizer {
    //Implement comment removal later
    let mut input: Vec<char> = Vec::new();
    let mut cc = false;

    for _line in input_lines.iter() {
      match (*_line).clone().into_string() {
        Ok(line) => {
          let chars = line.chars();
          for c in chars {
            if c == '/' { if cc { break}; cc = true; } else { cc = false }
            input.push(c);
          }
        },
        _=> continue,
      }
    }
    Tokenizer {
      input: input,
      input_index: 0,
    }
  }

  fn next(&mut self) -> Option<char> {
    if self.input_index > self.input.len() { return None; }
    self.input_index = self.input_index + 1;
    return Some(self.input[self.input_index - 1]);
  }

  pub fn advance(&mut self) -> Option<Token> {
    let mut current_build = String::new();

    loop {
      match self.next() {
        Some(c) => {
          if is_whitespace(c) {
            if current_build.len() > 0 {
              break Some(create_identifier_token(&current_build));
            } else {
              continue;
            }
          }
          current_build.push(c);

          match create_token(&current_build) {
            Some(token) => break Some(token),
            None => continue,
          };
        },
        None => {
          if current_build.len() > 0 {
            break Some(create_identifier_token(&current_build));
          } else {
            break None;
          }
        },
      }
    }
  }
}

fn create_token(input: &String) -> Option<Token> {
  match find_token_type(input) {
    Some(token_type) => {
      Some(Token {
        element: token_type,
        data: input.to_string(),
        keyword_key: None,
      })
    },
    _ => {
      match find_keyword(input) {
        Some(keyword_key) => {
          Some(Token {
            element: LexicalElement::Keyword,
            data: input.to_string(),
            keyword_key: Some(keyword_key),
          })
        },
        _ => None,
      }
    },
  }
}

fn find_token_type(input: &String) -> Option<LexicalElement> {
  if is_symbol(input) {
    return Some(LexicalElement::Symbol);
  } else if is_integer_constant(input) {
    return Some(LexicalElement::IntegerConstant);
  } else if is_string_constant(input) {
    return Some(LexicalElement::StringConstant);
  } else {
    return None;
  }
}

fn is_symbol(input: &String) -> bool {
  for symbol in SYMBOLS {
    if input == &symbol.to_string() {
      return true;
    } else {
      continue
    }
  }
  return false;
}

fn is_integer_constant(input: &String) -> bool {
  false
}

fn is_string_constant(input: &String) -> bool {
  false
}

fn find_keyword(input: &String) -> Option<String>{
  None
}

fn create_identifier_token(input: &String) -> Token {
  Token {
    element: LexicalElement::Identifier,
    data: input.to_string(),
    keyword_key: None,
  }
}

fn is_whitespace(c: char) -> bool {
  if c == ' '  ||
     c == '\t' {
    return true;
  } else {
    return false
  }
}
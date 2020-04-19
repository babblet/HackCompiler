use crate::grammar::LexicalElement;
use crate::grammar::LexicalElementKind;
use crate::grammar::SYMBOLS;
use crate::grammar::Keyword;
use crate::grammar::KeywordKind;
use crate::models::token::Token;
use std::slice::Iter;
use std::ffi::OsString;
use std::str::Chars;

pub struct Tokenizer {
  pub input: Vec<char>,
  pub input_index: usize,
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
            if c == '/' { if cc { break; } cc = true; continue; } else { cc = false; }
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
    if self.input_index >= self.input.len() { return None; }
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
              break Some(create_token(LexicalElementKind::Identifier, &current_build, None::<KeywordKind>));
            } else {
              continue;
            }
          } else if is_symbol(c) {
            if current_build.len() > 0 {
              self.input_index = self.input_index - 1;
              break Some(
                create_token(
                  LexicalElementKind::Identifier,
                  &current_build.to_string(),
                  None::<KeywordKind>
                )
              );
            } else {
              break Some(
                create_token(
                  LexicalElementKind::Symbol,
                  &c.to_string(),
                  None::<KeywordKind>
                )
              );
            }
          }

          current_build.push(c);

          match find_token(&current_build) {
            Some(token) => break Some(token),
            None => continue,
          };
        },
        None => {
          if current_build.len() > 0 {
            break Some(
              create_token(
                LexicalElementKind::Identifier,
                &current_build,
                None::<KeywordKind>
              )
            );
          } else {
            break None;
          }
        },
      }
    }
  }
}

pub fn find_token(input: &String) -> Option<Token> {
  match find_constant(input) {
    Some(token_type) => Some(
      create_token(token_type,
        input,
        None::<KeywordKind>
      )
    ),
    _ => {
      match find_keyword(input) {
        Some(key) => Some(
          create_token(
            LexicalElementKind::Keyword,
            input,
            Some(key)
          )
        ),
        _ => None,
      }
    },
  }
}

pub fn create_token(identifier: LexicalElementKind, data: &String, key: Option<KeywordKind>) -> Token {
  Token {
    element: LexicalElement::new(identifier),
    data: data.to_string(),
    keyword_key: match key {
      Some(_key) => Some(Keyword::new(_key)),
      None => None,
    }
  }
}

pub fn find_constant(input: &String) -> Option<LexicalElementKind> {
  if is_integer_constant(input) {
    return Some(LexicalElementKind::IntegerConstant);
  } else if is_string_constant(input) {
    return Some(LexicalElementKind::StringConstant);
  } else {
    return None;
  }
}

pub fn is_symbol(input: char) -> bool {
  for symbol in SYMBOLS {
    if input == *symbol {
      return true;
    } else {
      continue
    }
  }
  return false;
}

pub fn is_integer_constant(input: &String) -> bool {
  return match input.parse::<u16>() {
    Ok(i) => if i <= 2u16.pow(15) { true } else { false },
    _=> false,
  }
}

pub fn is_string_constant(input: &String) -> bool {
  let mut count: usize = 0;
  for c in input.chars() {
    if c == '"' {
      count = count + 1;
    }
  }
  if count == 2 { true } else { false }
}

pub fn find_keyword(input: &String) -> Option<KeywordKind> {
  for key in &Keyword::getKindArray() {
    if input == &key.as_string {
      return Some(key.kind);
    }
  }
  None
}

pub fn is_whitespace(c: char) -> bool {
  if c == ' '  || c == '\t' { true } else { false }
}
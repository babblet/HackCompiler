use std::ffi::OsString;
use crate::grammar::IdentifierKind;
use crate::grammar::LexicalElement;
use crate::grammar::LexicalElementKind;
use crate::grammar::SYMBOLS;
use crate::grammar::Keyword;
use crate::grammar::KeywordKind;
use crate::models::token::Token;

pub struct Tokenizer {
  pub input: Vec<char>,
  pub input_index: usize,
  latest_token: Option<Token>
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
      latest_token: None
    }
  }

  fn next(&mut self) -> Option<char> {
    if self.input_index >= self.input.len() { return None; }
    self.input_index = self.input_index + 1;
    return Some(self.input[self.input_index - 1]);
  }

  pub fn advance(&mut self) -> Result<Token, bool> {
    let mut current_build = String::new();

    loop {
      match self.next() {
        Some(c) => {
          if is_whitespace(c) {
            if current_build.len() > 0 {
              break Ok(self.create_identifier_token(&current_build.to_string()));
            } else {
              continue;
            }
          } else if is_symbol(c) {
            if current_build.len() > 0 {
              self.input_index = self.input_index - 1;
              break Ok(
                self.create_identifier_token(
                  &current_build.to_string(),
                )
              );
            } else {
              break Ok(
                self.create_plain_token(
                  LexicalElementKind::Symbol,
                  &c.to_string(),
                )
              );
            }
          }

          current_build.push(c);

          match self.find_token(&current_build) {
            Some(token) => break Ok(token),
            None => continue,
          };
        },
        None => {
          if current_build.len() > 0 {
            break Ok(
              self.create_identifier_token(&current_build.to_string())
            );
          } else {
            if self.input.len() <= self.input_index {
              break Err(false)
            } else {
              break Err(true)
            }
          }
        },
      }
    }
  }

  pub fn create_plain_token(self, element: LexicalElementKind, data: &String) -> Token {
    return create_token(element, data, None::<KeywordKind>, None::<IdentifierKind>);
  }
  
  pub fn create_keyword_token(self, data: &String, keyword_kind: KeywordKind) -> Token {
    return create_token(LexicalElementKind::Keyword, data, Some(keyword_kind), None::<IdentifierKind>);
  }
  
  pub fn create_identifier_token(self, data: &String) -> Token {
    let latest_token_keyword_kind = self.latest_token.unwrap().keyword_data.unwrap().kind;
    let identifier_kind = match latest_token_keyword_kind {
      KeywordKind::Method      |
      KeywordKind::Function    |
      KeywordKind::Constructor => IdentifierKind::SubroutineName,
      KeywordKind::Class       => IdentifierKind::ClassName,
      _ => IdentifierKind::VarName
    };
    return create_token(LexicalElementKind::Identifier, data, None::<KeywordKind>, Some(identifier_kind));
  }

  pub fn find_token(self, input: &String) -> Option<Token> {
    match find_constant(input) {
      Some(token_type) => Some(
        self.create_plain_token(
          token_type,
          input
        )
      ),
      _ => {
        match find_keyword(input) {
          Some(key) => Some(
            self.create_keyword_token(
              input,
              key
            )
          ),
          _ => None,
        }
      },
    }
  }
}

pub fn create_token (element: LexicalElementKind, data: &String, keyword_kind: Option<KeywordKind>, identifier_kind: Option<IdentifierKind>) -> Token {
  Token {
    element: LexicalElement::new(element),
    data: data.to_string(),
    keyword_data: match keyword_kind {
      Some(kind) => Some(Keyword::new(kind)),
      None => None,
    },
    identifier_kind: match identifier_kind {
      Some(_kind) => Some(_kind),
      None => None,
    },
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

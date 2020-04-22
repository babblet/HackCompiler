use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::ffi::OsString;
use create::models::token::Token;
use create::grammar::LexicalElementKind;
use create::grammar::KeywordKind;

pub struct CompilationEngine {
  input_tokens: Vec<Token>,
  input_index: usize,
}

impl CompilationEngine {
  pub fn new(input_tokens: Vec<Token>) -> CompilationEngine {
    return CompilationEngine {
      input_tokens: input_tokens
      current_token: 0,
    }
  }

  pub fn run(self) -> Result<String, String> {
    match compile_class() {
      Ok(build) => build,
      Err(e) => Err(format!("Error: Failed to compile =>\n{}", e))
    }
  }

  fn next(&mut self) -> Some(Token) {
    if self.input_index >= self.input_tokens.len() { return None; }
    self.input_index = self.input_index + 1;
    return Some(self.input_tokens[self.input_index - 1]);
  }

  fn compile_class(self) -> Result<String, String> {
    let class_token = match self.next() {
      Some(token) => {
        match token.keyword {
          Some(keyword) => {
            if keyword.kind == KeywordKind::CLASS {
              token
            } else {
              return Err(format!("Syntax error: Expected class, got {}", keyword.as_string));
            }
          },
          _ => { return Err(format!"Syntax error: Expected class, got {} {}", token.element.as_string, token.data)); }
        }
      },
      _ => { panic!("Was unable to read tokens"); }
    };

    let class_identifier = match self.next() {
      Some(token) => {
        if token.element.kind == LexicalElementKind::Identifier {
          token
        } else {
          return Err(format!("Syntax error: Expected class identifier, got {} {}", token.element.as_string, token.data));
        }
      },
      _ => { panic!("Was unable to read class identifier token"); }
    }
  }

  fn compile_class_var_dec() {

  }

  fn compile_subroutine_dec() {

  }

  fn compile_parameter_list() {

  }

  fn compile_subroutine_body() {

  }

  fn compile_var_dec() {

  }

  fn compile_statements() {

  }

  fn compile_let() {

  }

  fn compile_if () {

  }

  fn compile_while () {

  }

  fn compile_do() {

  }

  fn compile_return() {

  }

  fn compile_expression() {

  }

  fn compile_term() {

  }

  fn compile_expression_list() {

  }
}
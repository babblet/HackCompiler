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
  pub output_string: String,
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
      Err(e) => Err(format!("Error: Failed to compile,\n{}", e))
    }
  }

  fn next(&mut self) -> Some(Token) {
    if self.input_index >= self.input_tokens.len() { return None; }
    self.input_index = self.input_index + 1;
    return Some(self.input_tokens[self.input_index - 1]);
  }

  fn compile_class(mut self) -> Result<String, String> {

    let class_key = match self.next() {
      Some(token) => {
        match token.keyword {
          Some(keyword) => {
            if keyword.kind == KeywordKind::CLASS {
              token
            } else {
              return Err(format!(""));
            }
          },
          _ => { return Err(format!("")); }
        }
      },
      _ => { panic!("Was unable to read tokens"); }
    };

    let class_identifier = match self.next() {
      Some(token) => {
        if token.element.kind == LexicalElementKind::Identifier {
          token
        } else {
          return Err(format!(""));
        }
      },
      _ => { panic!("Was unable to read class identifier token"); }
    }
    
    let class_opening_bracket = match self.next() {
      Some(token) => {
        if token.element.kind == LexicalElementKind::Symbol {
          if token.data.as_str() == "{" {
            token
          } else {
            return Err(format!(""));
          }
        } else {
          return Err(format!(""));
        }
      },
      _ => { panic!("Was unable to read class identifier token"); }
    }

    let class_variables = match self.compile_class_var_dec() {
      Ok(variables) => variables,
      Err(e) => return Err(format!(""))
    }

    let class_subroutines = match self.compile_subroutine_dec() {
      Ok(subroutines) => subroutines,
      Err(e) => return Err(format!(""))
    }

    let class_closing_bracket = match self.next() {
      Some(token) => {
        if token.element.kind == LexicalElementKind::Symbol {
          if token.data.as_str() == "{" {
            token
          } else {
            return Err(format!(""));
          }
        } else {
          return Err(format!(""));
        }
      },
      _ => { panic!("Was unable to read class identifier token"); }
    }

    return Ok(
      format!(
        """
        <class>\n
        \t{}\n
        \t{}\n
        \t{}\n
        \t\t{}\n
        \t\t{}\n
        \t{}\n
        </class>\n
        """,
        class_key,
        class_identifier,
        class_opening_bracket,
        class_variables,
        class_subroutines,
        class_closing_bracket
      )
    )
  }

  fn compile_class_var_dec(self) Result<String, String> {
    let output_string = String::new();

    loop {
      match self.next() {}assert!()
    }
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
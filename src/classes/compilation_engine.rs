use crate::models::token::Token;
use crate::grammar::KeywordKind;
use crate::grammar::LexicalElementKind;
use crate::grammar::IdentifierKind;

pub struct CompilationEngine {
  pub input_tokens: Vec<Token>,
  pub input_index: usize,
}

impl CompilationEngine {
  pub fn new(input_tokens: Vec<Token>) -> CompilationEngine {
    return CompilationEngine {
      input_tokens: input_tokens,
      input_index: 0
    }
  }

  pub fn run(mut self) -> Result<String, String> {
    match self.compile_class() {
      Ok(build) => Ok(build),
      Err(e) => Err(format!("Error: Failed to compile,\n{}", e))
    }
  }

  fn next(&mut self) -> Option<&Token> {
    if self.input_index >= self.input_tokens.len() { return None; }
    self.input_index = self.input_index + 1;
    return Some(&self.input_tokens[self.input_index - 1]);
  }

  fn compile_class(&mut self) -> Result<String, String> {
    let class_key = match self.next() {
      Some(token) => {
        match &token.keyword_data {
          Some(keyword) => {
            if keyword.kind == KeywordKind::Class {
              format!("<keyword> {} </keyword>", token.data)
            } else {
              return Err(format!(""));
            }
          },
          _ => { return Err(format!("")); },
        }
      },
      _ => { panic!("Was unable to read tokens"); }
    };

    let class_identifier = match self.next() {
      Some(token) => {
        if token.element.kind == LexicalElementKind::Identifier {
          format!("<identifier> {} </identifier>", token.data)
        } else {
          return Err(format!(""));
        }
      },
      _ => { panic!("Was unable to read class identifier token"); }
    };

    let class_opening_bracket = match self.next() {
      Some(token) => {
        if token.element.kind == LexicalElementKind::Symbol {
          if token.data.as_str() == "{" {
            format!("<symbol> {} </symbol>", token.data)
          } else {
            return Err(format!(""));
          }
        } else {
          return Err(format!(""));
        }
      },
      _ => { panic!("Was unable to read class identifier token"); }
    };

    let class_variables = match self.compile_class_var_dec() {
      Ok(variables) => variables,
      Err(e) => return Err(format!(""))
    };

    let class_subroutines = match self.compile_subroutine_dec() {
      Ok(subroutines) => subroutines,
      Err(e) => return Err(format!(""))
    };

    let class_closing_bracket = match self.next() {
      Some(token) => {
        if token.element.kind == LexicalElementKind::Symbol {
          if token.data.as_str() == "{" {
            format!("<symbol> {} </symbol>", token.data)
          } else {
            return Err(format!(""));
          }
        } else {
          return Err(format!(""));
        }
      },
      _ => { panic!("Was unable to read class identifier token"); }
    };

    return Ok(
      format!(
        "
        <class>\n
        \t{}\n
        \t{}\n
        \t{}\n
        \t\t{}\n
        \t\t{}\n
        \t{}\n
        </class>\n
        ",
        class_key,
        class_identifier,
        class_opening_bracket,
        class_variables,
        class_subroutines,
        class_closing_bracket
      )
    )
  }

  fn compile_class_var_dec(&mut self) -> Result<String, String> {
    let mut output_string = "<classVarDec>\n".to_string();

    loop {
      match self.next() {
        Some(token) => {
          match &token.keyword_data {
            Some(keyword) => {
              if keyword.kind == KeywordKind::Field ||
                 keyword.kind == KeywordKind::Static {
                output_string.push_str(format!("<keyword> {} </keyword>\n", token.data.as_str()));
              } else {
                self.input_index = self.input_index - 1;
                return Ok("".to_string());
              }
            },
            _=> return Err(format!(""))
          }
        },
        _ => return Err(format!(""))
      }

      match self.next() {
        Some(token) => {
          if token.element.kind == LexicalElementKind::Identifier {
            match token.identifier_kind {
              Some(kind) => {
                if kind == IdentifierKind::ClassName {
                  output_string.push_str(format!("<identifier> {} </identifier>\n",token.data.as_str())); 
                } else {
                  return Err(format!(""));
                }
              },
              None => return Err(format!(""))
            }
          } else {
            match &token.keyword_data {
              Some(keyword) => {
                if keyword.kind == KeywordKind::Int ||
                   keyword.kind == KeywordKind::Boolean ||
                   keyword.kind == KeywordKind::Char {
                  output_string.push_str(format!("<keyword> {} </keyword>\n",token.data.as_str())); 
                } else {
                  return Err(format!(""))
                }
              },
              None => return Err(format!(""))
            }
          }
        },
        None => return Err(format!(""))
      }

      loop {
        match self.next() {
          Some(token) => {
            match token.identifier_kind {
              Some(kind) => {
                if kind == IdentifierKind::VarName {
                  output_string.push_str(format!("<identifier> {} </identifier>\n", token.data.as_str()));
                } else {
                  None => return Err(format!(""))
                }
              },
              None => return Err(format!(""))
            }
          },
          None => return Err(format!(""))
        };
        match self.next() {
          Some(token) => {
            if token.element == LexicalElementKind::Symbol {
              if token.data.as_str() == "," {
                output_string.push_str(format!("<symbol> {} </symbol>\n", token.data.as_str()));
              } else if token.data.as_str() == ";" {
                output_string.push_str(format!("<symbol> {} </symbol>\n", token.data.as_str()));
                break;
              } else {
                return Err(format!(""))
              }
            } else {
              return Err(format!(""))
            }
          },
          None => return Err(format!(""))
        }
      }
    }
    output_string.push_str(format!("</classVarDec>\n").to_string());
    return Ok(output_string);
  }

  fn compile_subroutine_dec(&mut self) -> Result<String, String> {
    let mut output_string = "<subroutineDec>\n".to_string();
    loop {
      match self.next() {
        Some(token) => {
          match token.keyword_data {
            Some(keyword) {
              if keyword.kind == KeywordKind::Method ||
                 keyword.kind == KeywordKind::Function ||
                 keyword.kind == KeywordKind::Constructor {
                output_string.push_str(format!("<keyword> {} </>\n", token.data.as_str()));
              } else {
                return Err(format!(""));
              }
            },
            None => return Err(format!(""))
          };
        },
        None => return Err(format!(""))
      }

      match self.next() {
        Some(token) => {
          match token.identifier_kind {
            Some(kind) => {
              if kind == IdentifierKind::ClassName {
                output_string.push_str(format!("<identifier> {} </identifier>\n", token.data.as_str()));
              } else {
                None => return Err(format!{""})
              }
            },
            None => {
              match token.keyword_data {
                Some(keyword) => {
                  match keyword.kind {
                    KeywordKind::Int  |
                    KeywordKind::Void |
                    KeywordKind::Char |
                    KeywordKind::Boolean => {
                      output_string.push_str(format!("<keyword> {} </keyword>", token.data.as_str())); 
                    },
                    _ => return Err(format!(""))
                  };
                },
                None => return Err(format!(""))
              };
            }
          };
        },
        None => return Err(format!{""})
      }
    }

    let mut output_string = "</subroutineDec>\n".to_string();
    return Ok(output_string);
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

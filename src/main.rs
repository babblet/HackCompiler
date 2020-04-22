use ::std::ffi::OsString;
use ::std::fs::File;
use ::std::io::Read;
use ::std::io::prelude::*;
use ::std::path::Path;

use hack_compiler::models::token::Token;
use hack_compiler::classes::arguments::Arguments;
use hack_compiler::classes::tokenizer::Tokenizer;
use hack_compiler::classes::compilation_engine::CompilationEngine;

fn main() {
  let environment_arguments: Vec<OsString> = std::env::args_os().collect();
  let arguments: Arguments = match Arguments::new(environment_arguments) {
    Ok(arguments_object) => arguments_object,
    Err(e) => {
      println!("Argument error: {}", e);
      return ();
    }
  };


  let input_lines: Vec<OsString> = match File::open(Path::new(&arguments.input)) {
    Ok(mut file) => {
      let mut buffer = String::new();
      match file.read_to_string(&mut buffer) {
        Ok(size) => println!("Read {} bytes", size),
        Err(e) => println!("Was unable to read from file: {}", e),
      };
      buffer.lines().map(|line| OsString::from(line)).collect()
    },
    _=> panic!("Failed to read file")
  };

  let mut file_buffer = String::new();
  let mut tokenizer = Tokenizer::new(&input_lines);
  let mut tokens: Vec<Token> = Vec::new(); 
  loop {
    match tokenizer.advance() {
      Ok(token) => {
        file_buffer.push_str(
          format!("<{}> {} </{}>\n",
            token.element.as_string,
            token.data,
            token.element.as_string
          ).as_str()
        );
        tokens.push(token);
      },
      Err(more) => if !more { break; },
    };
  }


  //Why should the implementation be like this? Debuging???
  let tokenizer_path = Path::new("t.xml");
  match File::create(tokenizer_path) {
    Ok(mut file) => {
      match file.write(file_buffer.as_bytes()) {
        Ok(total_bytes_writen) => println!("Wrote {} bytes to tokenizer file", total_bytes_writen),
        Err(e) => panic!("Error when writing to tokenizer file: {}", e)
      };
    },
    Err(e) => panic!("Error could not create tokenizer file: {}", e)
  };

  // unsure if this is needed... Should actually sync everything with
  // the tokens if I am not mistaken, and if so then this is a bad implementation.
  //let compilation_engine = CompilationEngine::new(tokens, Path::new("c.xml"));
  //compilation_engine.compile_class();
}

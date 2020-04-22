use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::ffi::OsString;

pub struct CompilationEngine {
  input_lines: Vec<OsString>,
  output_buffer: String,
}

impl CompilationEngine {
  pub fn new(input: &Path, output: &Path) -> Result<CompilationEngine, String> {
    let mut in_file: File = match File::open(input) {
      Ok(file) => file,
      Err(e) => panic!("Failed to load {}: {}", input.to_str().unwrap(), e)
    };

    let file_name = match input.file_name() {
      Some(file_name) => file_name.to_os_string(),
      None => OsString::from(""),
    };

    let mut buffer = String::new();
    match in_file.read_to_string(&mut buffer) {
      Ok(size) => println!("Read {} bytes from {}", size, input.to_str().unwrap()),
      Err(e) => println!("Was unable to read from file: {}", e),
    };

    let lines: Vec<OsString> = buffer.lines().map(|x| OsString::from(x)).collect();

    return Ok(CompilationEngine {
      input_lines: lines,
      output_buffer: String::new(),
    });
  }

  fn run(self) {
  }

  fn compile_class() {
    
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
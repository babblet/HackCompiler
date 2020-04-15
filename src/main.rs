use ::std::ffi::OsString;
use ::std::fs::File;
use ::std::io::Read;
use ::std::path::Path;
use hack_compiler::classes::arguments::Arguments;
use hack_compiler::classes::tokenizer::Tokenizer;

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
    Ok(file) => {
      let mut buffer = String::new();
      match file.read_to_string(&mut buffer) {
        Ok(size) => println!("Read {} bytes", size),
        Err(e) => println!("Was unable to read from file: {}", e),
      };
      buffer.lines().map(|x| OsString::from(x)).collect()
    },
    _=> panic!("Failed to read file")
  };

  let tokenizer = Tokenizer::new(&input_lines);
}

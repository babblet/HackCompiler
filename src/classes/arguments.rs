use std::ffi::OsString;
use std::path::Path;

pub struct Arguments {
  pub input: OsString,
  pub output: OsString,
}

impl Arguments {
  pub fn new(arguments: Vec<OsString>) -> Result<Arguments, String>{
    let input: OsString;
    let mut output = OsString::from("");
    let mut arguments_iterator = arguments.iter();
    arguments_iterator.next(); // Just pass process name

    match arguments_iterator.next() {
      Some(__input) => {
        let _input = Path::new(__input);
        if _input.is_file() {
          match _input.extension() {
            Some(extension) => {
              if extension == "jack" {
                input = OsString::from(_input.to_str().unwrap());
              } else {
                return Err(format!("Input has incorrect file format! Expecting <file>.vm instead of <file>.{}", extension.to_str().unwrap_or_default()));
              }
            },
            None => return Err("Input has incorrent file format! Expecting <file>.vm".to_string()),
          }
        } else {
          return Err("Input is not a file!".to_string());
        };
      }
      None => return Err("Missing input argument!".to_string()),
    }

    match arguments_iterator.next() {
      Some(__output) => {
        let _output = Path::new(__output);
        match _output.extension() {
          Some(_) => {
            output = OsString::from(_output.to_str().unwrap());
          },
          _=> return Err("Output file wrong format!".to_string()),
        };
      }
      _=>(),
    }

    return Ok(Arguments {
      input: input,
      output: output,
    })
  }
}

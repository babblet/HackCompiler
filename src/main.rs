use ::std::ffi::OsString;
use hack_compiler::classes::arguments::Arguments;
use hack_compiler::classes::analyzer::SyntaxAnalyzer;

fn main() {
  let environment_arguments: Vec<OsString> = std::env::args_os().collect();
  let arguments: Arguments = match Arguments::new(environment_arguments) {
    Ok(arguments_object) => arguments_object,
    Err(e) => {
      println!("Argument error: {}", e);
      return ();
    }
  };

  //let analyzer = SyntaxAnalyzer(arguments.input, arguments.output)
}

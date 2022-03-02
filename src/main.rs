#![allow(unused_variables, dead_code)]

mod cli;
mod vm;

use cli::{Parser, Cli};

const KIND2_HVM_CODE: &str = include_str!("../kind2.hvm");

fn main() {
  match run() {
    Ok(()) => {}
    Err(txt) => {
      eprintln!("{}", txt);
      std::process::exit(1);
    }
  }
}

const KI: usize = 1024;
const MI: usize = 1024 * 1024;
const GI: usize = 1024 * 1024 * 1024;

fn run() -> Result<(), String> {
  let cli_matches = Cli::parse();

  match cli_matches.command {
    cli::CliCmd::Compile{ file } => {
      let code = load_file(&file)?;
      Err("Compiling not implemented".to_string())
    }
    cli::CliCmd::Run{ file, debug } => {
      let code_str = load_file(&file)?;

      // Grows the stack. Without this, the code overflows sometimes on debug
      // mode. TODO: Investigate.
      stacker::grow(
        64 * MI, 
        || do_the_thing(KIND2_HVM_CODE, &code_str)
      )?;
      Ok(())
    }
  }
}

// TODO: this will be renamed, eventually
fn do_the_thing(kind2_code: &str, input_code: &str) -> Result<(), String> {
  use hvm::language as lang;
  use hvm::runtime as rt;
  use hvm::rulebook as rb;
  use hvm::builder as bd;
  use hvm::readback as rd;

  let mut worker = rt::new_worker();

  // Parses and reads the input file
  let file = lang::read_file(kind2_code)?;

  // Converts the HVM "file" to a Rulebook
  let book = rb::gen_rulebook(&file);

  // Builds dynamic functions
  let functions = bd::build_runtime_functions(&book);

  let str_term = vm::build_str_term(input_code);

  // dbg!(&str_term);

  let main_call = lang::Term::Ctr {
    name: "Main".to_string(),
    args: vec![ Box::new(str_term) ],
  };
  let main_pos = bd::alloc_term(&mut worker, &book, &main_call);

  rt::normal(&mut worker, main_pos, &functions, Some(&book.id_to_name), false);

  // Reads it back to a HVM string
  let book = Some(book);
  let result = match rd::as_term(&worker, &book, main_pos) {
    Ok(x)   => format!("{}", x),
    Err(..) => rd::as_code(&worker, &book, main_pos),
  };

  println!("Result:\n{}", result);

  Ok(())
}

fn load_file(file_name: &str) -> Result<String, String> {
  std::fs::read_to_string(file_name).map_err(|err| err.to_string())
}

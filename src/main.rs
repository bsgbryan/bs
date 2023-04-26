#![feature(iter_advance_by)]
mod compiler;
mod error;
mod lexer;
mod tokens;

use std::env;

use rustyline::{
  error::ReadlineError,
  DefaultEditor,
  Result as RustyResult,
};

use compiler::Compiler;

fn main() -> RustyResult<()> {
  let args: Vec<String> = env::args().collect();

  if args.len() == 2 {
    println!("Read from {}", args[1].to_string());
  }
  else {
    let mut rl = DefaultEditor::new()?;
    
    rl.load_history(".bs_history")?;

    loop {
      let readline = rl.readline(">> ");
      match readline {
        Ok(input) => {
          rl.add_history_entry(input.as_str())?;

          let mut compiler = Compiler::new(&input);
          compiler.compile();
        },
        Err(ReadlineError::Interrupted) => {
          println!("Master Control Program: End of line.");
          break
        },
        Err(ReadlineError::Eof) => {
          println!("Master Control Program: End of line.");
          break
        },
        Err(err) => {
          println!("Error: {:?}", err);
          break
        }
      }
    }

    rl.save_history(".bs_history")?;
  }

  Ok(())
}

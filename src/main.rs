#![feature(iter_advance_by)]
mod tokens;
mod error;
mod lexer;

use std::env;

use error::RuntimeError;

use rustyline::{
  error::ReadlineError,
  DefaultEditor,
  Result as RustyResult,
};

use lexer::tokenize;
use tokens::Token;

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
        Ok(line) => {
          rl.add_history_entry(line.as_str())?;

          let callback = |result: Result<&Token, RuntimeError>| {
            match result {
              Ok(r) => println!("Got Token: {:?}", r),
              Err(e) => println!("{}", e.message),
            }
          };

          tokenize(&line, &callback)
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

mod tokens;
mod error;
mod lexer;

use std::env;

use rustyline::error::ReadlineError;
use rustyline::{
  DefaultEditor,
  Result,
};

use lexer::tokenize;

fn main() -> Result<()> {
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

          match tokenize(&line) {
            Ok(tokens) => {
              for t in tokens.iter() {
                println!("Token {:?}", t.kind)
              }
            }
            Err(e) => {
              println!("{}", e.message)
            }
          }
        },
        Err(ReadlineError::Interrupted) => {
          println!("MCP: End of line.");
          break
        },
        Err(ReadlineError::Eof) => {
          println!("MCP: End of line.");
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

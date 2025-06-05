fn main() -> std::io::Result<()> {
  let args: Vec<String> = std::env::args().collect();
  let count = args.iter().count();

  if      count == 1 { bs::repl::execute()?;           }
  else if count == 2 { bs::file::execute(&args[1]);    }
  else               { eprintln!("Usage: bsg [path]"); }

  Ok(())
}
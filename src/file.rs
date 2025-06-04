pub fn execute(path: &str) {
  match std::fs::read_to_string(path) {
    Ok(f) => println!("Successfully read: {f}"),
    Err(e) => {
      eprintln!("Failed reading {path} with error {e:#?}");
      std::process::exit(-1);
    }
  }
}
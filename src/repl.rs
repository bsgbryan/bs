use std::io::Write;

use console::{
  Key,
  Term,
};

use crate::vm::interpret;

pub fn execute() -> std::io::Result<()> {
  let mut term = Term::stdout();

  term.write_line("🍻 shift + return to execute, command + c to exit")?;
  
  let mut chars = vec![String::new()];
  let mut line  = 0;

  loop {
    if let Ok(key) = term.read_key() {
      match key {

        // Execute code
        Key::End => {
          term.write_line("\nExecuting!")?;
          
          let source = chars.join("\n");

          interpret(&source);

          chars.clear();
          chars.push(String::new());
        },

        // Manage cursor
        Key::Enter => {
          chars.push(String::new());
          line += 1;
          term.write(&['\n' as u8])?;
        },
        Key::Tab => {
          chars[line] += "\t";
          term.write(&['\t' as u8])?;
        }
        Key::Backspace => {
          if chars[line].is_empty() {
            term.move_cursor_up(1)?;
            line -= 1;
            term.move_cursor_right(chars[line].chars().count())?;
          }
          else {
            let _ = chars[line].pop();
            term.clear_chars(1)?;
          }
        },

        // Print typed character
        Key::Char(c) => {
          chars[line].push(c);
          term.write(&[c as u8])?;
        },

        // Ignore
        _ => ()
      }
    }
  }
}
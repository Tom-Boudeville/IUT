use std::io::{self, Write};

use exquit_corpse::make_response;


fn main() -> io::Result<()>
 {
  let mut text = String::from("");
  let mut line = String::new();
  let _ = io::stdin().read_line(&mut line);
  let reponse = make_response(&line,&mut text);

  let _ = io::stdout().write_all(reponse.as_bytes());
  Ok(())  
}
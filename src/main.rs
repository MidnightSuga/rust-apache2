use std::process::Command;

fn main() {
  println!("Running whoami command");
  Command::new("whoami")
    .spawn()
    .expect("Error running command");
}

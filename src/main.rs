use std::process::Command;

fn run_command(command: &str) {
  println!("Running {}", command);
  Command::new(command)
      .spawn()
      .expect("error running command");
}

fn main() {
  let commands: Vec<&str> = vec![
    "whoami",
    "ifconfig",
    "/usr/bin/ip",
    "/usr/sbin/ip",
    "/bin/uname",
    "/bin/cat",
    "/bin/crontab",
    "hostname",
    "iptables",
    "netstat",
    "pwd",
    "route"
  ];

  for command in commands.iter() {
    run_command(command);
  };
}

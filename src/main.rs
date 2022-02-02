extern crate daemonize;
extern crate log;
extern crate simplelog;

use daemonize::Daemonize;
use log::info;
use std::fs::File;
use std::path::Path;

mod config;
mod machine;

use simplelog::*;

fn main() {
  let cfg = config::load();

  CombinedLogger::init(vec![TermLogger::new(
    LevelFilter::Warn,
    Config::default(),
    TerminalMode::Mixed,
    ColorChoice::Auto,
  )])
  .unwrap();
  let stdout = File::create("/tmp/daemon.out").unwrap();
  let stderr = File::create("/tmp/daemon.err").unwrap();
  let working_dir = Path::new(&cfg.working_dir);

  let daemonize = Daemonize::new()
    .pid_file(working_dir.join("machine.pid")) // Every method except `new` and `start`
    .chown_pid_file(true) // is optional, see `Daemonize` documentation
    .working_directory(working_dir) // for default behaviour.
    .user("ekonetzni")
    .group("daemon") // Group name
    .group(2) // or group id.
    .umask(0o777) // Set umask, `0o027` by default.
    .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
    .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
    .exit_action(|| println!("Executed before master process exits"))
    .privileged_action(|| "Executed before drop privileges");

  match daemonize.start() {
    Ok(_) => loop {
      println!("LOOPING");
    },
    Err(e) => eprintln!("Error, {}", e),
  }
}

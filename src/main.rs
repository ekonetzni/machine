extern crate daemonize;

use daemonize::Daemonize;
use std::fs::File;
use std::path::Path;
use std::thread::sleep;
use std::time;

mod config;
mod machine;

fn main() {
  let cfg = config::load();
  let working_dir = Path::new(&cfg.working_dir);
  let stdout = File::create(working_dir.join("daemon.out")).unwrap();
  let stderr = File::create(working_dir.join("daemon.err")).unwrap();

  let daemonize = Daemonize::new()
    .pid_file(working_dir.join("machine.pid"))
    .chown_pid_file(false)
    .working_directory(working_dir)
    .user("ekonetzni")
    .group("staff")
    .umask(0o777)
    .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
    .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
    .privileged_action(|| "Executed before drop privileges");

  match daemonize.start() {
    Ok(_) => {
      println!("Starting machine.");
      loop {
        let now = time::Instant::now();
        sleep(time::Duration::from_millis(5000));
        println!("Loop took {} seconds.", now.elapsed().as_secs());
      }
    }
    Err(e) => eprintln!("Error, {}", e),
  }
}

mod config;
mod machine;

extern crate log;
extern crate simplelog;

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
    machine::exe(cfg.daemon);
}

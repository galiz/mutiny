//! Some simple tools that can be used to test process monitoring systems

#![feature(plugin)]
#![feature(std_misc)]
#![plugin(docopt_macros)]

extern crate docopt;
#[macro_use] extern crate log;
extern crate simple_logger;
extern crate psutil;
extern crate "rustc-serialize" as rustc_serialize;

use std::path::Path;

mod memory;

docopt!(Args derive Debug, "
Usage:  mutiny [options] memory
        mutiny (--help | --version)

Options:
    -p <file>, --pidfile=<file>     A path to write a pidfile to.
    -h, --help                      Show this message.
", flag_pidfile: Option<String>);

/// Announce some information about the current program, and create a pidfile
pub fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());

    simple_logger::init();

    info!("Proccess PID is {}", psutil::getpid());

    if let Some(pidfile) = args.flag_pidfile {
        let path = std::env::current_dir().unwrap().join(&Path::new(&pidfile));
        psutil::pidfile::write_pidfile(&path).unwrap();
        info!("Wrote PID to {}", path.display());
    }

    if args.cmd_memory {
        memory::start()
    }
}

//! Some simple tools that can be used to test process monitoring systems

#![feature(exit_status)]
#![feature(old_io)]
#![feature(plugin)]
#![feature(std_misc)]
#![plugin(docopt_macros)]

// These is currently no replacement for `std::old_io::timer`
#![allow(deprecated)]

extern crate docopt;
#[macro_use] extern crate log;
extern crate simple_logger;
extern crate psutil;
extern crate "rustc-serialize" as rustc_serialize;

use std::env::{get_exit_status,set_exit_status};
use std::old_io::timer::sleep;
use std::path::Path;
use std::time::duration::Duration;

use psutil::getpid;
use psutil::process::Process;

docopt!(Args derive Debug, "
Usage:  mutiny [options] cpu [--force]
        mutiny [options] exit [<code>]
        mutiny [options] nothing
        mutiny [options] memory [<bytes>]
        mutiny (--help | --version)

Subcommands:
    cpu [--force]                   Use CPU time. Requires --force.
    exit [code]                     Exit with [code]. Defaults to 1.
    memory [bytes]                  Consume [bytes] memory. Defaults to 200Mb.
    nothing                         Do absolutely nothing. Ignores --duration.

Options:
    -d <secs>, --duration=<secs>    Time the command should take [default: 10].
    -p <file>, --pidfile=<file>     A path to write a pidfile to.
    -h, --help                      Show this message.
    -v, --version                   Show the program version.
",
    flag_pidfile: Option<String>,
    flag_duration: i64,
    arg_code: Option<i32>,
    arg_bytes: Option<i64>
);

fn exit(code: i32, duration: Duration) {
    set_exit_status(code);
    info!("Will exit with code {} in {} seconds", code, duration.num_seconds());
    sleep(duration);
    warn!("Exiting with code {}", get_exit_status());
}

fn nothing() {
    info!("Doing absolutely nothing, forever.");
    loop {
        sleep(Duration::max_value());
    }
}

fn memory(bytes: i64, duration: Duration) {
    let mut vec: Vec<u8> = Vec::with_capacity(0);
    let seconds = duration.num_seconds();
    let bytes_per_second = bytes / seconds;

    assert_eq!(bytes, bytes * std::mem::size_of::<u8>() as i64);

    info!("Consuming at least {}b over ~{} seconds ({}b bytes per second)",
        bytes, duration.num_seconds(), bytes_per_second);

    let before_alloc = Process::new(getpid()).unwrap().memory().unwrap().rss;
    info!("Using {}b before allocation", before_alloc);

    for i in 0..seconds {
        // This slows down the rate `Vec` reserves memory but doesn't avoid it
        // automatically allocating more memory than it will actually need
        vec.reserve_exact(bytes_per_second as usize);

        // Allocate memory by pushing byte-size numbers into a vector
        for _ in 0..bytes_per_second {
            vec.push(0);
        }

        // Announce current memory usage
        let memory = Process::new(getpid()).unwrap().memory().unwrap();
        trace!("Tick {}: using {}b (vector should be using {}b)",
            i, memory.rss, vec.len());

        sleep(Duration::seconds(1));
    }

    let after_alloc = Process::new(getpid()).unwrap().memory().unwrap().rss;
    info!("Using {}b after allocation (difference of {}b)",
        after_alloc, after_alloc - before_alloc);

    // Once memory is being used, do nothing instead of exiting
    nothing();
}

pub fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    let duration = Duration::seconds(args.flag_duration);

    if args.flag_version {
        println!("mutiny v{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    simple_logger::init();
    info!("Proccess PID is {}", psutil::getpid());

    if let Some(pidfile) = args.flag_pidfile {
        // TODO: Move `current_dir().join()` into psutil
        let path = std::env::current_dir().unwrap().join(&Path::new(&pidfile));
        psutil::pidfile::write_pidfile(&path).unwrap();
        info!("Wrote PID to {}", path.display());
    }

    if args.cmd_exit {
        exit(args.arg_code.unwrap_or(1), duration);
    } else if args.cmd_nothing {
        nothing();
    } else if args.cmd_memory {
        memory(args.arg_bytes.unwrap_or(200000000), duration);
    }
}

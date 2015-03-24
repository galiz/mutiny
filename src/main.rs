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
extern crate riemann_client;
extern crate "rustc-serialize" as rustc_serialize;

use std::env::{get_exit_status,set_exit_status};
use std::net::ToSocketAddrs;
use std::old_io::timer::sleep;
use std::path::Path;
use std::time::duration::Duration;

use psutil::getpid;
use psutil::process::Process;
use riemann_client::Client;

docopt!(Args derive Debug, "
Usage:  mutiny [options] cpu [--force]
        mutiny [options] exit [<code>]
        mutiny [options] nothing
        mutiny [options] memory [<bytes>]
        mutiny (--help | --version)

Subcommands:
    cpu [--force]                   Use CPU time. Requires --force.
    exit [code]                     Exit with [code]. Defaults to 1.
    memory [bytes]                  Allocate [bytes] memory. Defaults to 200Mb.
    nothing                         Do absolutely nothing. Ignores --duration.

Options:
    --riemann-host=<host>           Hostname for the Riemann server to use.
    --riemann-port=<port>           Port for the Riemann server [default: 5555].
    -d <secs>, --duration=<secs>    Time the command should take [default: 10].
    -p <file>, --pidfile=<file>     A path to write a pidfile to.
    -h, --help                      Show this message.
    -v, --version                   Show the program version.
",
    flag_riemann_host: Option<String>,
    flag_riemann_port: u16,
    flag_pidfile: Option<String>,
    flag_duration: i64,
    arg_code: Option<i32>,
    arg_bytes: Option<i64>
);

struct Mutiny {
    client: Option<Client>
}

impl Mutiny {
    fn new() -> Self {
        info!("Proccess PID is {}", psutil::getpid());
        Mutiny { client: None }
    }

    fn connect<A: ToSocketAddrs + ?Sized>(&mut self, addr: &A) {
        self.client = Some(riemann_client::Client::connect(addr).unwrap());
        info!("Connecting to Riemann at {}", addr.to_socket_addrs().unwrap().next().unwrap());
    }

    fn pidfile(&mut self, path: &Path) {
        let path = std::env::current_dir().unwrap().join(path);
        psutil::pidfile::write_pidfile(&path).unwrap();
        info!("Wrote PID to {}", path.display());
    }

    fn exit(&mut self, code: i32, duration: Duration) {
        set_exit_status(code);
        info!("Will exit with code {} in {} seconds",
            code, duration.num_seconds());
        sleep(duration);
        warn!("Exiting with code {}", get_exit_status());
    }

    fn nothing(&mut self) {
        info!("Doing absolutely nothing, forever.");
        loop {
            sleep(Duration::max_value());
        }
    }

    fn memory(&mut self, bytes: i64, duration: Duration) {
        let mut vec: Vec<u8> = Vec::with_capacity(0);
        let seconds = duration.num_seconds();
        let bytes_per_second = bytes / seconds;

        assert_eq!(bytes, bytes * std::mem::size_of::<u8>() as i64);

        info!("Consuming at least {}b over ~{} seconds ({}b bytes per second)",
            bytes, duration.num_seconds(), bytes_per_second);

        let before_alloc = Process::new(getpid()).unwrap().memory().unwrap();
        info!("Using {}b before allocation", before_alloc.resident);

        for i in 0..seconds {
            // This slows down the rate `Vec` reserves memory but doesn't avoid
            // it automatically allocating more memory than it needs
            vec.reserve_exact(bytes_per_second as usize);

            // Allocate memory by pushing byte-size numbers into a vector
            for _ in 0..bytes_per_second {
                vec.push(0);
            }

            // Announce current memory usage
            let memory = Process::new(getpid()).unwrap().memory().unwrap();
            trace!("Tick {}: using {}b (vector should be using {}b)",
                i, memory.resident, vec.len());

            if let Some(ref mut client) = self.client {
                let mut event = riemann_client::proto::Event::new();
                event.set_service("mutiny".to_string());
                event.set_host(riemann_client::utils::hostname().unwrap());
                // This is currently broken
                // event.set_metric_sint64(memory.resident as i64);
                client.send_event(event).unwrap();
            };

            sleep(Duration::seconds(1));
        }

        let after_alloc = Process::new(getpid()).unwrap().memory().unwrap();
        info!("Using {}b after allocation (difference of {}b)",
            after_alloc.resident, after_alloc.resident - before_alloc.resident);

        // Once memory is being used, do nothing instead of exiting
        self.nothing();
    }
}

pub fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    let duration = Duration::seconds(args.flag_duration);

    if args.flag_version {
        println!("mutiny v{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    simple_logger::init().unwrap();

    let mut mutiny = Mutiny::new();

    if let Some(host) = args.flag_riemann_host {
        mutiny.connect(&(&host[..], args.flag_riemann_port));
    }

    if let Some(path) = args.flag_pidfile {
        mutiny.pidfile(&Path::new(&path));
    }

    if args.cmd_exit {
        mutiny.exit(args.arg_code.unwrap_or(1), duration);
    } else if args.cmd_nothing {
        mutiny.nothing();
    } else if args.cmd_memory {
        mutiny.memory(args.arg_bytes.unwrap_or(200000000), duration);
    }
}

#![feature(collections)]
#![feature(core)]
#![feature(env)]
#![feature(io)]
#![feature(path)]
#![feature(std_misc)]

extern crate getopts;
extern crate term;
extern crate psutil;

mod memory;

/// Makes `print_meta` work like `format!` :)
macro_rules! print_meta(
    ($name:expr, $($arg:expr),+) => (
        println!("{:>12} {}", $name, $($arg),+);
    )
);

fn print_usage(program: String, options: getopts::Options) {
    let brief = options.short_usage(program.as_slice()) + " COMMAND";
    let usage = options.usage(brief.as_slice());
    print!("{}", usage);
}

/// Announce some information about the current program, and create a pidfile
pub fn main() {
    let mut options = getopts::Options::new();
    options.optopt("p", "pidfile", "Set pidfile path", "PATH");
    options.optflag("h", "help", "Print this help menu");

    let arguments: Vec<String> = std::env::args().collect();
    let matches = match options.parse(arguments.tail()) {
        Ok(matches) => matches,
        Err(error) => { panic!("{}", error); }
    };

    if matches.opt_present("h") || matches.free.len() < 1 {
        print_usage(arguments[0].clone(), options);
        return;
    }

    // Create a pidfile
    let path_arg = matches.opt_str("p").unwrap_or("mutiny.pid".to_string());
    let path = std::env::current_dir().unwrap().join(&Path::new(path_arg));
    psutil::pidfile::write_pidfile(&path).unwrap();

    // Show the pidfile path and pid
    print_meta!("Pidfile", path.display());
    print_meta!("PID", psutil::getpid());

    match matches.free[0].as_slice() {
        "memory" => memory::start(),
        _ => unreachable!()
    }
}

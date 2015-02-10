#![feature(collections)]
#![feature(core)]
#![feature(env)]
#![feature(io)]
#![feature(path)]
#![feature(os)]

extern crate getopts;
extern crate term;
extern crate psutil;

/// Makes `print_meta` work like `format!` :)
macro_rules! print_meta(
    ($name:expr, $($arg:expr),+) => (
        println!("{:>12} {}", $name, $($arg),+);
    )
);

fn args() -> Vec<String> {
    std::env::args().map(|s| s.into_string().unwrap()).collect()
}

fn print_usage(program: String, options: getopts::Options) {
    let brief = options.short_usage(program.as_slice());
    let usage = options.usage(brief.as_slice());
    print!("{}", usage);
}

/// Announce some information about the current program, and create a pidfile
pub fn start(description: &str) {
    let mut options = getopts::Options::new();
    options.reqopt("p", "", "set pidfile path", "PATH");
    options.optflag("h", "help", "print this help menu");

    let arguments = args();
    let matches = match options.parse(arguments.tail()) {
        Ok(matches) => matches,
        Err(error) => { panic!("{}", error); }
    };

    if matches.opt_present("h") {
        print_usage(arguments[0].clone(), options);
        return;
    }

    // Create a pidfile
    let path_arg = Path::new(matches.opt_str("p").unwrap());
    let path = std::env::current_dir().unwrap().join(&path_arg);
    psutil::pidfile::write_pidfile(&path).unwrap();

    print_meta!("Description", description);
    print_meta!("Pidfile", path.display());
    print_meta!("PID", psutil::getpid());
}

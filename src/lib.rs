#![feature(io)]
#![feature(path)]
#![feature(os)]

extern crate term;
extern crate psutil;

use std::old_io::File;
use std::os;

/// Creates a pidfile from a name and returns it's path path
pub fn pidfile_path(name: String) -> Path {
    let mut path = Path::new(name);
    path.set_extension("pid");
    path = os::make_absolute(&path).unwrap();
    return path;
}

/// Prints a name and description in the same way cargo does
fn print_meta(name: &str, description: String) {
    let mut terminal = term::stdout().unwrap();
    terminal.fg(term::color::GREEN).unwrap();
    write!(terminal, "{:>12} ", name).unwrap();
    terminal.reset().unwrap();
    writeln!(terminal, "{}", description).unwrap();
}

/// Makes `print_meta` work like `format!` :)
macro_rules! print_meta(
    ($name:expr, $($arg:expr),+) => (
        $crate::print_meta($name, format!("{}", $($arg),+));
    )
);

/// Announce some information about the current program, and create a pidfile
pub fn start(description: &str) {
    let path = pidfile_path(os::args()[0].clone());
    psutil::pidfile::write_pidfile(&path);

    print_meta!("Description", description);
    print_meta!("Pidfile", path.display());
    print_meta!("PID", psutil::getpid());
}

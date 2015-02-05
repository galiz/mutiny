#![feature(io)]
#![feature(path)]
#![feature(os)]

extern crate term;
extern crate psutil;

use std::old_io::File;
use std::os;

/// Writes the PID of the current process to a file
pub fn pidfile(path: &Path) {
    write!(&mut File::create(path).unwrap(), "{}", psutil::getpid()).unwrap();
}

/// Creates a pidfile from a name and returns it's path path
pub fn pidfile_for(name: String) -> Path {
    let mut path = Path::new(name);
    path.set_extension("pid");
    path = os::make_absolute(&path).unwrap();
    pidfile(&path.clone());
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
    let path = pidfile_for(os::args()[0].clone());

    // Announce information about the program
    print_meta!("Description", description);
    print_meta!("Pidfile", path.display());
    print_meta!("PID", psutil::getpid());
}

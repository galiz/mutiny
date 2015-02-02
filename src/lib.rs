#![allow(unstable)]

extern crate libc;
extern crate term;

pub type PID = i32;

/// Get the PID of the current process
///
/// At some point this is worth moving into rust-psutil
fn getpid() -> PID {
    unsafe { libc::getpid() }
}

/// Get the PID of the parent process
///
/// At some point this is worth moving into rust-psutil
fn getppid() -> PID {
    unsafe { libc::funcs::posix88::unistd::getppid() }
}

/// Prints a name and description in the same way cargo does
fn print_meta(name: &str, description: String) {
    let mut terminal = term::stdout().unwrap();
    terminal.fg(term::color::GREEN).unwrap();
    write!(terminal, "{:>12} ", name).unwrap();
    terminal.reset().unwrap();
    writeln!(terminal, "{}", description).unwrap();
}

/// Announce some information about the current program
pub fn announce(description: &str) {
    print_meta("Description", description.to_string());
    print_meta("PID", getpid().to_string());
    print_meta("Parent PID", getppid().to_string());
}

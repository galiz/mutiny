#![allow(unstable)]

extern crate libc;

use std::io;

fn main() {
    unsafe { println!("{}", libc::getpid()); }

    let mut vec = Vec::new();

    for i in 0..10000000 {
        vec.push(i);
    }

    for line in io::stdin().lock().lines() {
        print!("{}", line.unwrap());
    }
}

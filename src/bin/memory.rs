#![allow(unstable)]

extern crate wrecking_ball;

use std::io;

fn main() {
    wrecking_ball::announce("Consume memory and wait to be killed");

    let mut vec = Vec::new();

    for i in 0..10000000 {
        vec.push(i);
    }

    for line in io::stdin().lock().lines() {
        print!("{}", line.unwrap());
    }
}

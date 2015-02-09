#![feature(core)]
#![feature(io)]
#![feature(std_misc)]

extern crate wrecking_ball;

use std::old_io as io;
use std::old_io::timer::Timer;
use std::time::duration::Duration;

fn main() {
    wrecking_ball::start("Consume memory and wait to be killed");

    let mut vec = Vec::new();
    let mut timer = Timer::new().unwrap();

    for _ in 0..20 {
        timer.sleep(Duration::milliseconds(500));
        for i in 0..1000000 {
            vec.push(i);
        }
    }

    for line in io::stdin().lock().lines() {
        print!("{}", line.unwrap());
    }
}

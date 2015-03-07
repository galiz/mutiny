use std::old_io as io;
use std::old_io::timer::Timer;
use std::time::duration::Duration;

use psutil::getpid;
use psutil::process::Process;

pub fn start() {
    let mut vec: Vec<u64> = Vec::with_capacity(0);
    let mut timer = Timer::new().unwrap();

    for t in 0..10 {
        // Allocate memory
        for i in 0..10000000 {
            vec.push(i);
        }
        vec.shrink_to_fit();

        // Announce current memory usage
        let memory = Process::new(getpid()).unwrap().memory().unwrap();
        info!("Iteration {}: using {} Mb ({} * u64)",
            t, memory.rss / 1024 / 1024, vec.capacity());

        // Sleep so that memory is allocated over time
        timer.sleep(Duration::seconds(1));
    }

    let mut stdin = io::stdin();
    for line in stdin.lock().lines() {
        print!("{}", line.unwrap());
    }
}

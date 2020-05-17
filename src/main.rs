mod game_object;
mod timer;

use std::time::{Instant}; // timer
use game_object::refresh;
use timer::Timer;

fn main() {
    let start = Instant::now();

    let game_timer = Timer::new(1000);
    
    game_timer.execute();
    
    println!("time cost: {:?} ms", start.elapsed().as_millis());// ms
    println!("time cost: {:?} us", start.elapsed().as_micros());// us
    println!("time cost: {:?} ns", start.elapsed().as_nanos());// us
}
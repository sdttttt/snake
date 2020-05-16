
mod game_object;

use std::time::{Instant}; // timer
use game_object::refresh;

fn main() {
    let start = Instant::now();

 	refresh();

    println!("time cost: {:?} ms", start.elapsed().as_millis());// ms
    println!("time cost: {:?} us", start.elapsed().as_micros());// us
    println!("time cost: {:?} ns", start.elapsed().as_nanos());// us
}
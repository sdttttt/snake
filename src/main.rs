mod game_object;
mod timer;
mod input_event;

use std::thread;
use timer::Timer;
use input_event::watch_player;

fn main() {
    thread::spawn(watch_player);
    let game_timer = Timer::new(1000);
    game_timer.execute();
}
